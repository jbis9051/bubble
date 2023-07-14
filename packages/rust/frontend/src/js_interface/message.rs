use crate::api::BubbleApi;
use crate::application_message::Message;
use crate::helper::bubble_group::BubbleGroup;
use crate::helper::helper::{get_this_client_mls_resources, parse_identity};
use crate::js_interface::FrontendInstance;
use crate::mls_provider::MlsProvider;
use crate::models::account::group::Group;
use crate::models::account::inbox::Inbox;
use crate::models::account::location::Location;
use crate::types::MLS_GROUP_CONFIG;
use crate::Error;
use openmls::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::types::chrono::{NaiveDateTime, Utc};

use crate::models::kv::AccountKv;

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct WaitingStagedCommit {
    pub commit_hash: Vec<u8>,
    pub commit_message_hash: Vec<u8>,
}

fn print_message(message: &Inbox) {
    let message = MlsMessageIn::tls_deserialize_exact(&message.message).unwrap();
    let content = message.extract();
    match content {
        MlsMessageInBody::PublicMessage(m) => {
            let a: ProtocolMessage = m.into();
            println!("public: {:?}", a.content_type());
        }
        MlsMessageInBody::PrivateMessage(m) => {
            let a: ProtocolMessage = m.into();
            println!("private: {:?}", a.content_type());
        }
        MlsMessageInBody::Welcome(_m) => {
            println!("welcome")
        }
        MlsMessageInBody::GroupInfo(_m) => {
            println!("GroupInfo")
        }
        MlsMessageInBody::KeyPackage(_m) => {
            println!("key_package")
        }
    }
}

impl FrontendInstance {
    //#[bridge]
    pub async fn receive_messages(&self) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().unwrap();
        let account_db = &global_data.database;
        let my_client_uuid = &global_data.client_uuid.read().await.unwrap();
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let messages = api.receive_messages(*my_client_uuid).await.unwrap();

        for message in messages {
            let mut inbox = Inbox {
                id: 0,
                message: message.message.0,
                server_received_date: NaiveDateTime::from_timestamp_millis(message.received_date)
                    .unwrap(),
                received_date: Utc::now().naive_utc(),
            };
            inbox.create(account_db).await.unwrap();
        }

        self.process_messages().await;
        Ok(())
    }

    async fn process_messages(&self) {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().unwrap();
        let account_db = &global_data.database;
        let mls_provider = MlsProvider::new(account_db.clone());
        let messages = Inbox::all(account_db).await.unwrap();
        let messages = {
            let mut out = Vec::with_capacity(messages.len());
            for message in messages {
                let message = match MlsMessageIn::tls_deserialize_exact(&message.message) {
                    Ok(m) => (m, message),
                    Err(_) => continue, // TODO: consider logging error
                };
                out.push(message);
            }
            out
        };
        for (message, inbox_message) in messages {
            let body = message.extract();
            match body {
                MlsMessageInBody::PublicMessage(m) => {
                    self.process_group_message(&inbox_message, m.into())
                        .await
                        .unwrap();
                }
                MlsMessageInBody::PrivateMessage(m) => {
                    self.process_group_message(&inbox_message, m.into())
                        .await
                        .unwrap();
                }
                MlsMessageInBody::Welcome(welcome) => {
                    let mut group = BubbleGroup::new(
                        MlsGroup::new_from_welcome(&mls_provider, &MLS_GROUP_CONFIG, welcome, None)
                            .unwrap(),
                    );
                    let group_id = Uuid::from_slice(group.group_id().as_slice()).unwrap();

                    let exists = Group::from_uuid(account_db, group_id).await.unwrap();

                    if exists.is_none() {
                        Group {
                            id: 0,
                            uuid: group_id,
                            name: None,
                            image: None,
                            updated_at: NaiveDateTime::default(),
                        }
                        .create(account_db)
                        .await
                        .unwrap();
                    }

                    group.save_if_needed(&mls_provider).unwrap();
                }
                MlsMessageInBody::GroupInfo(_) => continue,
                MlsMessageInBody::KeyPackage(_) => continue,
            }

            // TODO: process message

            Inbox::delete_by_id(account_db, inbox_message.id)
                .await
                .unwrap();
        }
    }

    async fn process_group_message(
        &self,
        inbox_message: &Inbox,
        message: ProtocolMessage,
    ) -> Result<(), ()> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().unwrap();
        let account_db = &global_data.database;
        let mls_provider = MlsProvider::new(account_db.clone());
        let user_uuid = &global_data.user_uuid;
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let mut group = BubbleGroup::new(
            MlsGroup::load(
                &GroupId::from_slice(message.group_id().as_slice()),
                &mls_provider,
            )
            .unwrap(),
        );

        if message.content_type() != ContentType::Application && group.epoch() > message.epoch() {
            /* println!(
                "skipping message with epoch {} as we are at epoch {}",
                message.epoch(),
                group.epoch()
            );*/
            return Ok(());
        }

        if let Some(pending_commit) = group.pending_commit() {
            // we need to check if this is our own message
            // retrieve the last commit from the db
            let last_commit = AccountKv::get(
                account_db,
                &format!("waiting_staged_commit_{}", group.group_uuid()),
            )
            .await
            .unwrap();
            if let Some(last_commit) = last_commit {
                let last_commit: WaitingStagedCommit = serde_json::from_str(&last_commit).unwrap();
                // first we hash the raw message
                let commit_hash =
                    Sha256::digest(serde_json::to_vec(&pending_commit).unwrap()).to_vec();
                let commit_message_hash = Sha256::digest(&inbox_message.message).to_vec();
                if last_commit.commit_hash == commit_hash
                    && last_commit.commit_message_hash == commit_message_hash
                {
                    group.merge_pending_commit(&mls_provider).unwrap();
                    group.save_if_needed(&mls_provider).unwrap();
                    return Ok(());
                }
            }
        }

        let group_message = group.process_message(&mls_provider, message).unwrap();
        let (_, client_uuid) = parse_identity(group_message.credential().identity()).unwrap();
        let content = group_message.into_content();
        match content {
            ProcessedMessageContent::ApplicationMessage(app) => {
                let message: Message = serde_json::from_slice(&app.into_bytes()).unwrap();
                match message {
                    Message::Location(message) => {
                        Location {
                            id: 0,
                            client_uuid,
                            group_uuid: group.group_uuid(),
                            longitude: message.longitude,
                            latitude: message.latitude,
                            location_date: NaiveDateTime::from_timestamp_millis(message.timestamp)
                                .unwrap(),
                            raw: inbox_message.message.clone(),
                            created_date: Default::default(),
                        }
                        .create(account_db)
                        .await
                        .unwrap();
                    }
                    Message::GroupStatus(status) => {
                        let mut group = Group::from_uuid(account_db, group.group_uuid())
                            .await
                            .unwrap()
                            .unwrap();
                        if inbox_message.server_received_date > group.updated_at {
                            group.name = status.name;
                            group.image = status.image.map(|i| i.0);
                            group.updated_at = Utc::now().naive_utc();
                            group.update(account_db).await.unwrap();
                        }
                    }
                }
            }
            ProcessedMessageContent::ProposalMessage(m) => {
                let (signature, _) = get_this_client_mls_resources(
                    user_uuid,
                    &client_uuid,
                    account_db,
                    &mls_provider,
                )
                .await
                .unwrap();

                // whenever we receive a proposal, we store it in the pending proposals, commit, and then send the commit to the group including ourselves
                // we wait for the DS to send it back to us before merging
                group.store_pending_proposal(*m);

                let (commit, _welcome, _group_info) = group
                    .commit_to_pending_proposals(&mls_provider, &signature)
                    .unwrap();

                // the above should have created a pending commit
                // let's retrieve it
                let staged = group.pending_commit().unwrap();

                // hash the commit
                let commit_hash = Sha256::digest(serde_json::to_vec(staged).unwrap()).to_vec();
                // hash the commit message
                let commit_message_hash =
                    Sha256::digest(commit.tls_serialize_detached().unwrap()).to_vec();
                let waiting = WaitingStagedCommit {
                    commit_hash,
                    commit_message_hash,
                };

                let members = group.get_group_members().unwrap();
                let removed_client_uuids = staged
                    .remove_proposals()
                    .map(|p| p.remove_proposal().removed())
                    .map(|i| members.iter().find(|m| m.index == i).unwrap().client_uuid)
                    .collect::<Vec<_>>();

                // store them in the db, keyed by the group uuid
                AccountKv::set(
                    account_db,
                    &format!("waiting_staged_commit_{}", group.group_uuid()),
                    &serde_json::to_string(&waiting).unwrap(),
                )
                .await
                .unwrap();

                // finally, send the commit to the group
                group
                    .send_message(&api, &commit, &removed_client_uuids)
                    .await
                    .unwrap();
            }
            ProcessedMessageContent::ExternalJoinProposalMessage(_) => {
                panic!("unsupported message type: {:?}", content)
            }
            ProcessedMessageContent::StagedCommitMessage(commit) => {
                // TODO check if commit is valid
                group.merge_staged_commit(&mls_provider, *commit).unwrap();
            }
        }
        group.save_if_needed(&mls_provider).unwrap();
        Ok(())
    }
}
