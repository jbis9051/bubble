use crate::models::kv::GlobalKv;

use sqlx::types::{chrono, Uuid};
use sqlx::Row;

pub async fn register(username: String, password: String, name: String) {
    // Global Database = actual global database?
    let db = crate::GLOBAL_DATABASE.get().unwrap();
    let user_uuid = Uuid::new_v4().to_string();
    let client_uuid = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO user (uuid, name, identity, updated_date) VALUES ($1, $2, $3, $4);")
        .bind(&user_uuid)
        .bind(&name)
        .bind(&username)
        .bind(chrono::Utc::now().timestamp())
        .execute(db)
        .await
        .unwrap();

    let user_id = sqlx::query("SELECT id FROM user WHERE uuid = $1")
        .bind(&user_uuid)
        .fetch_one(db)
        .await
        .unwrap()
        .get::<i32, _>("id");

    // signing_key from client?
    sqlx::query(
        "INSERT INTO client (uuid, user_id, signing_key, validated_date) VALUES ($1, $2, $3, $4)",
    )
    .bind(&client_uuid)
    .bind(user_id)
    .bind(&password)
    .bind(Option::<i64>::None)
    .execute(db)
    .await
    .unwrap();

    //maybe we need user_data??????
    let _user_data = GlobalKv::get(db, &format!("user:{}", user_id))
        .await
        .unwrap();
}

pub async fn login(username: String, _password: String) {
    let db = crate::GLOBAL_DATABASE.get().unwrap();
    let _user_id = sqlx::query("SELECT FROM user WHERE identity = $1")
        .bind(&username)
        .fetch_one(db)
        .await
        .unwrap();
    let _real_password = sqlx::query("SELECT password FROM user WHERE identity = $1")
        .bind(&username)
        .fetch_one(db)
        .await
        .unwrap();
    // if _password == _real_password {
    //     //verify password somehow????? Idk how rn I'm tired I have to wake up 6am fuckkkkkkk
    // }
}

pub async fn logout() {
    let db = crate::GLOBAL_DATABASE.get().unwrap();

    GlobalKv::delete(db, "current_account").await.unwrap();
}

pub async fn forgot(_email: String) {
    //?
    let db = crate::GLOBAL_DATABASE.get().unwrap();

    GlobalKv::set(db, "forgot_email", &_email).await.unwrap();

    let _response = reqwest::get("accounts/user/forgot")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}
