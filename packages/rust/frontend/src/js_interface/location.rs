use crate::api::BubbleApi;
use crate::application_message::{Location, Message};
use crate::helper::bubble_group::BubbleGroup;
use crate::helper::helper::get_this_client_mls_resources;
use crate::js_interface::FrontendInstance;
use crate::mls_provider::MlsProvider;
use crate::models::account::location::Location as LocationModel;
use bridge_macro::bridge;
use openmls::group::MlsGroup;
use openmls::prelude::GroupId;
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

impl FrontendInstance {
    #[bridge]
    pub async fn get_location(
        &self,
        group_uuid: Uuid,
        client: Uuid,
        before_timestamp: i64,
        amount: u32,
    ) -> Result<Vec<Location>, ()> {
        let global = self.account_data.read().await;
        let account_db = &global.as_ref().unwrap().database;
        let timestamp = NaiveDateTime::from_timestamp_millis(before_timestamp).unwrap();
        let locations = LocationModel::query(account_db, &group_uuid, &client, &timestamp, amount)
            .await
            .unwrap();

        Ok(locations
            .into_iter()
            .map(|location| Location {
                longitude: location.longitude,
                latitude: location.latitude,
                timestamp: location.location_date.timestamp_millis(),
            })
            .collect())
    }

    #[bridge]
    pub async fn get_num_location(
        &self,
        group_uuid: Uuid,
        client: Uuid,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<i64, ()> {
        let global = self.account_data.read().await;
        let account_db = &global.as_ref().unwrap().database;
        let from_timestamp = NaiveDateTime::from_timestamp_millis(from_timestamp).unwrap();
        let to_timestamp = NaiveDateTime::from_timestamp_millis(to_timestamp).unwrap();
        let locations = LocationModel::count_query(
            account_db,
            &group_uuid,
            &client,
            &from_timestamp,
            &to_timestamp,
        )
        .await
        .unwrap();

        Ok(locations)
    }

    #[bridge]
    pub async fn send_location(
        &self,
        group_uuid: Uuid,
        longitude: f64,
        latitude: f64,
        timestamp: i64,
    ) -> Result<(), ()> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().unwrap();
        let account_db = &global_data.database;
        let user_uuid = &global_data.user_uuid;
        let client_uuid = &global_data.client_uuid.read().await.unwrap();
        let mls_provider = MlsProvider::new(account_db.clone());
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let (signature, _) =
            get_this_client_mls_resources(user_uuid, client_uuid, account_db, &mls_provider)
                .await
                .unwrap();

        let mut group = BubbleGroup::new(
            MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider).unwrap(),
        );

        let message = Message::Location(Location {
            longitude,
            latitude,
            timestamp,
        });

        group
            .send_application_message(&mls_provider, &api, &signature, &message, &[*client_uuid])
            .await
            .unwrap();

        group.save_if_needed(&mls_provider).unwrap();
        Ok(())
    }
}
