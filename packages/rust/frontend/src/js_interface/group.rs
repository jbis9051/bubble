use crate::api::BubbleApi;
use crate::application_message::{GroupStatus, Message};
use crate::helper::bubble_group::BubbleGroup;
use crate::helper::helper::get_this_client_mls_resources;
use crate::helper::resource_fetcher::ResourceFetcher;
use crate::js_interface::user::UserOut;
use crate::js_interface::FrontendInstance;
use crate::mls_provider::MlsProvider;
use crate::models::account::group::Group as GroupModel;
use crate::types::MLS_GROUP_CONFIG;
use crate::Error;
use bridge_macro::bridge;
use common::base64::Base64;
use openmls::group::MlsGroup;
use openmls::prelude::{GroupId, ProtocolVersion, TlsSerializeTrait};
use openmls_traits::OpenMlsCryptoProvider;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[bridge]
#[derive(Serialize, Deserialize)]
pub struct UserGroupInfo {
    pub info: UserOut,
    pub clients: Vec<Uuid>,
}

#[bridge]
#[derive(Serialize, Deserialize)]
pub struct Group {
    pub uuid: Uuid,
    pub name: Option<String>,
    pub image: Option<Vec<u8>>,
    pub members: HashMap<Uuid, UserGroupInfo>,
}

impl FrontendInstance {
    #[bridge]
    pub async fn get_groups(&self) -> Result<Vec<Group>, Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::NoGlobalAccountData)?;
        let account_db = &global_data.database;
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let resource_fetcher = ResourceFetcher::new(api.clone(), account_db.clone());

        let db_groups = GroupModel::all_in_group(account_db).await?;

        let mut out = Vec::with_capacity(db_groups.len());

        for group in db_groups {
            let mls_group = BubbleGroup::new_from_uuid(&group.uuid, &mls_provider)
                .ok_or_else(|| Error::MLSGroupLoad)?;
            let members = mls_group.get_group_members()?;
            let mut out_members: HashMap<_, _> = HashMap::with_capacity(members.len());
            for member in members {
                let client = resource_fetcher
                    .get_client_partial_authentication(&member.client_uuid)
                    .await?;
                let user = resource_fetcher
                    .get_user_partial_authentication(&client.user_uuid)
                    .await?;
                out_members
                    .entry(client.user_uuid)
                    .or_insert_with(|| UserGroupInfo {
                        info: UserOut {
                            uuid: client.user_uuid,
                            username: user.username,
                            name: user.name,
                            primary_client_uuid: user.primary_client_uuid,
                            identity: Base64(user.identity),
                        },
                        clients: Vec::with_capacity(1),
                    })
                    .clients
                    .push(member.client_uuid);
            }
            out.push(Group {
                uuid: group.uuid,
                name: group.name,
                image: group.image,
                members: out_members,
            });
        }
        Ok(out)
    }

    #[bridge]
    pub async fn create_group(&self) -> Result<Uuid, Error> {
        let global = self.account_data.read().await;
        let account_data = global.as_ref().ok_or_else(|| Error::NoGlobalAccountData)?;
        let account_db = &account_data.database;
        let user_uuid = &account_data.user_uuid;
        let client_uuid = account_data.client_uuid.read().await.unwrap();

        let mls_provider = MlsProvider::new(account_db.clone());
        let (signature, credential_with_key) =
            get_this_client_mls_resources(user_uuid, &client_uuid, account_db, &mls_provider)
                .await?;
        let uuid = Uuid::new_v4();
        let mut group = MlsGroup::new_with_group_id(
            &mls_provider,
            &signature,
            &MLS_GROUP_CONFIG,
            GroupId::from_slice((uuid).as_ref()),
            credential_with_key,
        )?;
        group.save(&mls_provider)?;

        GroupModel {
            id: 0,
            uuid,
            name: None,
            image: None,
            updated_at: NaiveDateTime::default(),
            in_group: true,
        }
        .create(account_db)
        .await?;

        Ok(uuid)
    }

    #[bridge]
    pub async fn add_member(&self, group_uuid: Uuid, user_uuid: Uuid) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::NoGlobalAccountData)?;
        let account_db = &global_data.database;
        let client_uuid = global_data.client_uuid.read().await.unwrap();
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let resource_fetcher = ResourceFetcher::new(api.clone(), account_db.clone());
        let (signature, _) = get_this_client_mls_resources(
            &global_data.user_uuid,
            &client_uuid,
            account_db,
            &mls_provider,
        )
        .await?;

        let mut group = BubbleGroup::new(
            MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider)
                .ok_or_else(|| Error::MLSGroupLoad)?,
        );

        let clients = resource_fetcher
            .get_clients_full_authentication(&user_uuid)
            .await?;

        if clients.is_empty() {
            return Err(Error::NoClientsFound);
        }

        let mut key_packages = Vec::with_capacity(clients.len());
        let mut client_uuids = Vec::with_capacity(clients.len());
        for client in &clients {
            client_uuids.push(client.uuid);
            let key_package_in = api.request_key_package(&client.uuid).await?;
            let key_package = key_package_in
                .validate(mls_provider.crypto(), ProtocolVersion::default())
                .unwrap();
            key_packages.push(key_package);
        }

        let old_members = group
            .get_group_members()?
            .into_iter()
            .filter(|m| m.client_uuid != client_uuid) // we don't want to send the message to ourselves
            .map(|m| m.client_uuid)
            .collect::<Vec<_>>();

        let (mls_message_out, welcome_out, _group_info) =
            group.add_members(&mls_provider, &signature, &key_packages)?;
        // TODO what happens if we add a member that is already in the group?

        group.merge_pending_commit(&mls_provider).unwrap();

        let mls_message_out = mls_message_out.tls_serialize_detached()?;
        let welcome_out = welcome_out.tls_serialize_detached()?;

        // we send the welcome message to the new members first, because if it fails, it's easier to recover from

        api.send_message(client_uuids, welcome_out, group_uuid)
            .await?;
        api.send_message(old_members, mls_message_out, group_uuid)
            .await?;

        group.save_if_needed(&mls_provider)?;

        Ok(())
    }

    #[bridge]
    pub async fn remove_member(&self, group_uuid: Uuid, user_uuid: Uuid) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::NoGlobalAccountData)?;
        let account_db = &global_data.database;
        let client_uuid = global_data.client_uuid.read().await.unwrap();
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let my_client_uuid = &global_data
            .client_uuid
            .read()
            .await
            .ok_or_else(|| Error::ReadClientUUID)?;

        let (signature, _) = get_this_client_mls_resources(
            &global_data.user_uuid,
            &client_uuid,
            account_db,
            &mls_provider,
        )
        .await?;
        let mut group = BubbleGroup::new(
            MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider)
                .ok_or_else(|| Error::MLSGroupLoad)?,
        );

        let members_to_remove = group
            .get_group_members()?
            .into_iter()
            .filter(|m| m.user_uuid == user_uuid)
            .map(|m| m.index)
            .collect::<Vec<_>>();

        let (mls_message_out, welcome_out, _group_info) =
            group.remove_members(&mls_provider, &signature, &members_to_remove)?;

        if welcome_out.is_some() {
            return Err(Error::UnexpectedWelcome);
        }

        group.merge_pending_commit(&mls_provider).unwrap();

        group
            .send_message(&api, &mls_message_out, &[*my_client_uuid])
            .await?;

        group.save_if_needed(&mls_provider)?;

        Ok(())
    }

    #[bridge]
    pub async fn leave_group(&self, group_uuid: Uuid) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::NoGlobalAccountData)?;
        let account_db = &global_data.database;
        let user_uuid = &global_data.user_uuid;
        let client_uuid = global_data.client_uuid.read().await.unwrap();
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );

        let (signature, _) =
            get_this_client_mls_resources(user_uuid, &client_uuid, account_db, &mls_provider)
                .await?;
        let mut group = BubbleGroup::new(
            MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider)
                .ok_or_else(|| Error::MLSGroupLoad)?,
        );
        let mut group_model = GroupModel::from_uuid(account_db, group.group_uuid())
            .await?
            .unwrap();

        let my_user_uuid = &global_data.user_uuid;
        let my_client_uuid = &global_data
            .client_uuid
            .read()
            .await
            .ok_or_else(|| Error::ReadClientUUID)?;

        // get all clients for our user with the exception of our own client
        let members_to_remove = group
            .get_group_members()?
            .into_iter()
            .filter(|m| &m.user_uuid == my_user_uuid && &m.client_uuid != my_client_uuid)
            .map(|m| m.index)
            .collect::<Vec<_>>();

        // remove all members except our own client

        if !members_to_remove.is_empty() {
            let (mls_message_out, welcome_out, _group_info) =
                group.remove_members(&mls_provider, &signature, &members_to_remove)?;

            group.merge_pending_commit(&mls_provider).unwrap();

            if welcome_out.is_some() {
                // we do not support proposals so no proposals should exist
                return Err(Error::UnexpectedWelcome);
            }

            group
                .send_message(&api, &mls_message_out, &[*my_client_uuid])
                .await?;
        }

        // finally we leave the group for our client
        let leave_message = group.leave_group(&mls_provider, &signature)?;

        group.merge_pending_commit(&mls_provider).unwrap();

        group
            .send_message(&api, &leave_message, &[*my_client_uuid])
            .await?;

        group.save_if_needed(&mls_provider)?;

        group_model.in_group = false;
        group_model.update(account_db).await?;

        Ok(())
    }

    #[bridge]
    pub async fn update_group(&self, group_uuid: Uuid, name: Option<String>) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::NoGlobalAccountData)?;
        let account_db = &global_data.database;
        let mls_provider = MlsProvider::new(account_db.clone());
        let mut group = BubbleGroup::new(
            MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider)
                .ok_or_else(|| Error::MLSGroupLoad)?,
        );
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let client_uuid = global_data.client_uuid.read().await.unwrap();

        let (signature, _) = get_this_client_mls_resources(
            &global_data.user_uuid,
            &client_uuid,
            account_db,
            &mls_provider,
        )
        .await?;

        let message = Message::GroupStatus(GroupStatus {
            name: name.clone(),
            image: None,
        });

        group
            .send_application_message(&mls_provider, &api, &signature, &message, &[client_uuid])
            .await?;

        let mut group = GroupModel::from_uuid(account_db, group_uuid)
            .await?
            .unwrap();
        group.name = name;
        group.updated_at = Utc::now().naive_utc();
        group.update(account_db).await?;

        Ok(())
    }

    #[bridge]
    pub async fn send_group_status(&self, group_uuid: Uuid) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::NoGlobalAccountData)?;
        let account_db = &global_data.database;
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let group = GroupModel::from_uuid(account_db, group_uuid)
            .await?
            .unwrap();
        let client_uuid = global_data.client_uuid.read().await.unwrap();

        let message = Message::GroupStatus(GroupStatus {
            name: group.name,
            image: group.image.map(Base64),
        });

        let (signature, _) = get_this_client_mls_resources(
            &global_data.user_uuid,
            &client_uuid,
            account_db,
            &mls_provider,
        )
        .await?;

        let mut group = BubbleGroup::new(
            MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider)
                .ok_or_else(|| Error::MLSGroupLoad)?,
        );

        group
            .send_application_message(&mls_provider, &api, &signature, &message, &[client_uuid])
            .await?;

        Ok(())
    }
}
