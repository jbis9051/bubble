use crate::api::BubbleApi;
use crate::models::account::user::User;
use crate::types::DbPool;
use common::http_types::PublicUser;
use uuid::Uuid;

pub async fn get_user_with_cache_check(
    uuid: &Uuid,
    api: &BubbleApi,
    db: &DbPool,
) -> Result<PublicUser, ()> {
    let local_user = User::try_from_uuid(db, uuid).await.unwrap();
    let api_user = api.get_user(uuid).await.unwrap();

    if let Some(user) = local_user {
        if user.identity != *api_user.identity {
            panic!("User identity mismatch in cache vs api");
        }
    }

    Ok(api_user)
}
