use crate::helper::account_url;
use crate::models::kv::{AccountKv, GlobalKv};
use crate::{Error, GlobalAccountData};
use common::base64::Base64;
use common::http_types::{CreateUser, RegisteredClientsResponse};
use ed25519_dalek::Keypair;
use rand_core_2::OsRng;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::types::{chrono, Uuid};
use sqlx::Row;

use tokio::sync::RwLock;

//create a database with sqlite
//set database to
// update global db with user entry
// identity = key
//make a key and store in db
// send an api route to create a user
//from uuid create account db
//not updating global var

pub async fn register(
    username: String,
    password: String,
    name: String,
    email: String,
) -> Result<(), Error> {
    //create key pair and store as signing key for client
    let mut csprng = OsRng {};
    let account_key = Keypair::generate(&mut csprng);

    // api request
    let path = "/v1/user/register";
    let url = account_url(path).await;

    let created_user = CreateUser {
        email: email.clone(),
        username: username.clone(),
        password: password.clone(),
        name: name.clone(),
        identity: Base64(account_key.secret.to_bytes().to_vec()),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&created_user).unwrap())
        .send()
        .await
        .unwrap();
    let response: RegisteredClientsResponse = response.json().await.unwrap();
    let user_uuid = response.uuid;
    let client_uuid = Uuid::new_v4().to_string();

    let _global_db = crate::GLOBAL_DATABASE.get().unwrap();

    // create an account db based on up in migrations
    let account_db = SqlitePoolOptions::new()
        .connect("sqlite:accounts.db")
        .await?;
    sqlx::migrate!("./migrations/account")
        .run(&account_db)
        .await?;

    //assign new account_db to global account data var, clone when writing to it? or use a mutex
    let bearer = AccountKv::get(&account_db, "bearer").await?;
    let domain = AccountKv::get(&account_db, "domain").await?;

    if let Some(bearer) = bearer {
        let mut write = crate::GLOBAL_ACCOUNT_DATA.write().await;
        *write = Some(GlobalAccountData {
            bearer: RwLock::new(bearer),
            domain: domain.unwrap_or_default(),
            database: account_db.clone(),
        });
        drop(write);
    }

    // update account db
    sqlx::query("INSERT INTO user (uuid, name, identity, updated_date) VALUES ($1, $2, $3, $4);")
        .bind(&user_uuid)
        .bind(&name)
        .bind(&username)
        .bind(chrono::Utc::now().timestamp())
        .execute(&account_db)
        .await
        .unwrap();

    let user_id = sqlx::query("SELECT id FROM user WHERE uuid = $1")
        .bind(&user_uuid)
        .fetch_one(&account_db)
        .await
        .unwrap()
        .get::<i32, _>("id");

    sqlx::query(
        "INSERT INTO client (uuid, user_id, signing_key, validated_date) VALUES ($1, $2, $3, $4)",
    )
    .bind(&client_uuid)
    .bind(user_id)
    .bind(&account_key.public.to_bytes().to_vec())
    .bind(chrono::Utc::now().timestamp())
    .execute(&account_db)
    .await
    .unwrap();

    Ok(())
    //todo, remove unwraps()
}

pub async fn login(username: String, _password: String) -> Result<(), Error> {
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
    Ok(())
}

pub async fn logout() -> Result<(), Error> {
    let db = crate::GLOBAL_DATABASE.get().unwrap();

    GlobalKv::delete(db, "current_account").await.unwrap();
    Ok(())
}

pub async fn forgot(_email: String) -> Result<(), Error> {
    let db = crate::GLOBAL_DATABASE.get().unwrap();

    GlobalKv::set(db, "forgot_email", &_email).await.unwrap();

    let _response = reqwest::get("accounts/user/forgot")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    Ok(())
}
