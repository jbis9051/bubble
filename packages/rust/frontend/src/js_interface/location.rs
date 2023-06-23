use crate::api::BubbleApi;
use crate::application_message::{Location, Message};
use crate::helper::bubble_group::BubbleGroup;
use crate::helper::helper::get_this_client_mls_resources;
use crate::mls_provider::MlsProvider;
use crate::models::account::location::Location as LocationModel;
use crate::GLOBAL_ACCOUNT_DATA;
use bridge_macro::bridge;
use openmls::group::MlsGroup;
use openmls::prelude::GroupId;
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[bridge]
pub async fn get_location(
    group_uuid: Uuid,
    client: Uuid,
    before_timestamp: i64,
    amount: u32,
) -> Result<Vec<Location>, ()> {
    let global = &GLOBAL_ACCOUNT_DATA.read().await;
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
    group_uuid: Uuid,
    client: Uuid,
    from_timestamp: i64,
    to_timestamp: i64,
) -> Result<i64, ()> {
    let global = &GLOBAL_ACCOUNT_DATA.read().await;
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
    group_uuid: Uuid,
    longitude: f64,
    latitude: f64,
    timestamp: i64,
) -> Result<(), ()> {
    let global = GLOBAL_ACCOUNT_DATA.read().await;
    let global_data = global.as_ref().unwrap();
    let account_db = &global_data.database;
    let mls_provider = MlsProvider::new(account_db.clone());
    let api = BubbleApi::new(
        global_data.domain.clone(),
        global_data.bearer.read().await.clone(),
    );
    let (signature, _) = get_this_client_mls_resources(account_db, &mls_provider)
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
        .send_application_message(&mls_provider, &api, &signature, &message)
        .await
        .unwrap();

    group.save_if_needed(&mls_provider).unwrap();
    Ok(())
}
