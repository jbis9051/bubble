/*
function get_groups(): Group[] {}
function create_group(): uuid {}
function add_member(group_uuid: uuid, user: uuid){}
function remove_member(group_uuid: uuid, user: uuid){}
function leave_group(group_uuid: uuid){}
 */

use crate::api::BubbleApi;
use crate::helper::bubble_group::BubbleGroup;
use crate::helper::helper::get_this_client_mls_resources;
use crate::helper::resource_fetcher::ResourceFetcher;
use crate::js_interface::FrontendInstance;
use crate::mls_provider::MlsProvider;
use crate::models::account::group::Group as GroupModel;
use crate::Error;
use bridge_macro::bridge;
use openmls::group::MlsGroup;
use openmls::prelude::{GroupId, MlsGroupConfig, TlsSerializeTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[bridge]
#[derive(Serialize, Deserialize)]
pub struct Group {
    pub uuid: Uuid,
    pub name: String,
    pub image: Vec<u8>,
    pub members: HashMap<Uuid, Vec<Uuid>>,
}

impl FrontendInstance {
    #[bridge]
    pub async fn get_groups(&self) -> Result<Vec<Group>, Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::DBReference)?;
        let account_db = &global_data.database;
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            global_data.bearer.read().await.clone(),
        );
        let resource_fetcher = ResourceFetcher::new(api.clone(), account_db.clone());

        let db_groups = GroupModel::all(account_db).await?;

        let mut out = Vec::with_capacity(db_groups.len());

        for group in db_groups {
            let mls_group = BubbleGroup::new(
                MlsGroup::load(&GroupId::from_slice(group.uuid.as_ref()), &mls_provider)
                    .ok_or_else(|| Error::MLSGroupLoad)?,
            );
            let members = mls_group.get_group_members()?;
            let mut out_members: HashMap<_, Vec<_>> = HashMap::with_capacity(members.len());
            for (client_uuid, _) in members {
                let client = resource_fetcher
                    .get_client_partial_authentication(&client_uuid)
                    .await?;
                out_members
                    .entry(client.user_uuid)
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push(client.uuid);
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
        let account_db = &global.as_ref().ok_or_else(|| Error::DBReference)?.database;

        let mls_provider = MlsProvider::new(account_db.clone());
        let (signature, credential_with_key) =
            get_this_client_mls_resources(account_db, &mls_provider).await?;
        let uuid = Uuid::new_v4();
        let mut group = MlsGroup::new_with_group_id(
            &mls_provider,
            &signature,
            &MlsGroupConfig::default(),
            GroupId::from_slice((uuid).as_ref()),
            credential_with_key,
        )?;
        group.save(&mls_provider)?;

        Ok(uuid)
    }

    #[bridge]
    pub async fn add_member(&self, group_uuid: Uuid, user_uuid: Uuid) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::DBReference)?;
        let account_db = &global_data.database;
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            global_data.bearer.read().await.clone(),
        );
        let resource_fetcher = ResourceFetcher::new(api.clone(), account_db.clone());
        let (signature, _) = get_this_client_mls_resources(account_db, &mls_provider).await?;

        let mut group = BubbleGroup::new(
            MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider)
                .ok_or_else(|| Error::MLSGroupLoad)?,
        );

        let clients = resource_fetcher
            .get_clients_full_authentication(&user_uuid)
            .await?;

        let mut key_packages = Vec::with_capacity(clients.len());
        let mut client_uuids = Vec::with_capacity(clients.len());
        for client in &clients {
            client_uuids.push(client.uuid);
            let key_package = api.request_key_package(&client.uuid).await?;
            key_packages.push(key_package);
        }

        let old_members = group
            .get_group_members()?
            .into_iter()
            .map(|(uuid, _)| uuid)
            .collect::<Vec<_>>();

        let (mls_message_out, welcome_out, _group_info) =
            group.add_members(&mls_provider, &signature, &key_packages)?;
        // TODO what happens if we add a member that is already in the group?

        let mls_message_out = mls_message_out.tls_serialize_detached()?;
        let welcome_out = welcome_out.tls_serialize_detached()?;

        // we send the welcome message to the new members first, because if it fails, it's easier to recover from

        api.send_message(client_uuids, welcome_out).await?;
        api.send_message(old_members, mls_message_out).await?;

        group.save_if_needed(&mls_provider)?;

        Ok(())
    }

    #[bridge]
    pub async fn remove_member(&self, group_uuid: Uuid, user_uuid: Uuid) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::DBReference)?;
        let account_db = &global_data.database;
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            global_data.bearer.read().await.clone(),
        );
        let resource_fetcher = ResourceFetcher::new(api.clone(), account_db.clone());
        let my_client_uuid = &global_data
            .client_uuid
            .read()
            .await
            .ok_or_else(|| Error::ReadClientUUID)?;

        let (signature, _) = get_this_client_mls_resources(account_db, &mls_provider).await?;
        let mut group = BubbleGroup::new(
            MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider)
                .ok_or_else(|| Error::MLSGroupLoad)?,
        );

        let members_to_remove = group
            .get_group_members_by_user_uuid(&user_uuid, &resource_fetcher)
            .await?
            .into_iter()
            .map(|(_, index)| index)
            .collect::<Vec<_>>();

        let (mls_message_out, welcome_out, _group_info) =
            group.remove_members(&mls_provider, &signature, &members_to_remove)?;

        if welcome_out.is_some() {
            return Err(Error::UnexpectedWelcome);
        }

        group
            .send_message(&api, &mls_message_out, &[*my_client_uuid])
            .await?;

        Ok(())
    }

    #[bridge]
    pub async fn leave_group(&self, group_uuid: Uuid) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::DBReference)?;
        let account_db = &global_data.database;
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            global_data.bearer.read().await.clone(),
        );
        let resource_fetcher = ResourceFetcher::new(api.clone(), account_db.clone());

        let (signature, _) = get_this_client_mls_resources(account_db, &mls_provider).await?;
        let mut group = BubbleGroup::new(
            MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider)
                .ok_or_else(|| Error::MLSGroupLoad)?,
        );

        let my_user_uuid = &global_data.user_uuid;
        let my_client_uuid = &global_data
            .client_uuid
            .read()
            .await
            .ok_or_else(|| Error::ReadClientUUID)?;

        // get all clients for our user with the exception of our own client
        let members_to_remove = group
            .get_group_members_by_user_uuid(my_user_uuid, &resource_fetcher)
            .await?
            .into_iter()
            .filter(|(uuid, _)| uuid != my_client_uuid)
            .map(|(_, index)| index)
            .collect::<Vec<_>>();

        // remove all members except our own client
        let (mls_message_out, welcome_out, _group_info) =
            group.remove_members(&mls_provider, &signature, &members_to_remove)?;

        if welcome_out.is_some() {
            // we do not support proposals so no proposals should exist
            return Err(Error::UnexpectedWelcome);
        }

        group
            .send_message(&api, &mls_message_out, &[*my_client_uuid])
            .await?;

        // finally we leave the group for our client
        let leave_message = group.leave_group(&mls_provider, &signature)?;

        group.send_message(&api, &leave_message, &[]).await?;

        group.save_if_needed(&mls_provider)?;

        Ok(())
    }
}
