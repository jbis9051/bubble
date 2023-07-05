use crate::api::BubbleApi;
use crate::application_message::Message;
use crate::helper::bubble_group::BubbleGroup;
use crate::helper::helper::parse_identity;
use crate::js_interface::FrontendInstance;
use crate::mls_provider::MlsProvider;
use crate::models::account::group::Group;
use crate::models::account::inbox::Inbox;
use crate::models::account::location::Location;
use crate::types::{DbPool, MLS_GROUP_CONFIG};
use openmls::prelude::*;
use sqlx::types::chrono::{NaiveDateTime, Utc};

use uuid::Uuid;

impl FrontendInstance {
    // #[bridge]
    pub async fn receive_messages(&self) -> Result<(), crate::Error> {
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
            Inbox {
                id: 0,
                message: message.message.0,
                received_date: Utc::now().naive_utc(),
            }
            .create(account_db)
            .await
            .unwrap();
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
                    process_group_message(account_db, &inbox_message, &mls_provider, m.into())
                        .await
                        .unwrap();
                }
                MlsMessageInBody::PrivateMessage(m) => {
                    process_group_message(account_db, &inbox_message, &mls_provider, m.into())
                        .await
                        .unwrap();
                }
                MlsMessageInBody::Welcome(welcome) => {
                    let mut group = BubbleGroup::new(
                        MlsGroup::new_from_welcome(&mls_provider, &MLS_GROUP_CONFIG, welcome, None)
                            .unwrap(),
                    );
                    let group_id = Uuid::from_slice(group.group_id().as_slice()).unwrap();
                    Group {
                        id: 0,
                        uuid: group_id,
                        name: None,
                        image: None,
                    }
                    .create(account_db)
                    .await
                    .unwrap();

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

        async fn process_group_message(
            account_db: &DbPool,
            inbox_message: &Inbox,
            mls_provider: &MlsProvider,
            message: ProtocolMessage,
        ) -> Result<(), ()> {
            let mut group = BubbleGroup::new(
                MlsGroup::load(
                    &GroupId::from_slice(message.group_id().as_slice()),
                    mls_provider,
                )
                .unwrap(),
            );
            let group_message = group.process_message(mls_provider, message).unwrap();
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
                                location_date: NaiveDateTime::from_timestamp_millis(
                                    message.timestamp,
                                )
                                .unwrap(),
                                raw: inbox_message.message.clone(),
                                created_date: Default::default(),
                            }
                            .create(account_db)
                            .await
                            .unwrap();
                        }
                    }
                }
                ProcessedMessageContent::ProposalMessage(_) => {
                    panic!("unsupported message type: {:?}", content)
                }
                ProcessedMessageContent::ExternalJoinProposalMessage(_) => {
                    panic!("unsupported message type: {:?}", content)
                }
                ProcessedMessageContent::StagedCommitMessage(commit) => {
                    // TODO check if commit is valid
                    group.merge_staged_commit(mls_provider, *commit).unwrap();
                }
            }
            group.save_if_needed(mls_provider).unwrap();
            Ok(())
        }
    }
}
