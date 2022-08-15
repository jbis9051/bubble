use axum_test_helper::{TestClient, TestResponse};
use std::borrow::Borrow;
use std::env;

use bubble::models::user::User;
use bubble::router;

use bubble::routes::group::GroupName;
use bubble::types::DbPool;
use sqlx::postgres::{PgPoolOptions, PgRow};

use axum::http::StatusCode;
use bubble::models::confirmation::Confirmation;

use bubble::routes::user::{ChangeEmail, Confirm, CreateUser, SessionToken, SignInJson};

use bubble::models::session::Session;
use sqlx::migrate::MigrateDatabase;
use sqlx::Postgres;
use uuid::Uuid;

pub struct TempDatabase {
    pool: DbPool,
    _db_url: String,
}

impl TempDatabase {
    pub async fn new() -> Self {
        let db_url = format!("{}/{}", env::var("DB_URL").unwrap(), Uuid::new_v4());
        Postgres::create_database(&db_url).await.unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();

        sqlx::migrate!("../db/migrations").run(&pool).await.unwrap();

        Self {
            pool,
            _db_url: db_url,
        }
    }

    pub fn pool(&self) -> &DbPool {
        &self.pool
    }
}

pub async fn start_server(pool: DbPool) -> TestClient {
    let router = router::router(pool);

    TestClient::new(router)
}

// For user authentication testing only
pub async fn signup_user(
    db: &DbPool,
    client: &TestClient,
    user_in: &CreateUser,
) -> Result<(User, Uuid), StatusCode> {
    let res = client
        .post("/user/signup")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(user_in).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let user = User::from_username(db, &user_in.username).await.unwrap();
    let link_id = Confirmation::filter_user_id(db, user.id).await.unwrap()[0].link_id;

    Ok((user, link_id))
}

// For user authentication testing only
pub async fn signup_confirm_user(
    db: &DbPool,
    client: &TestClient,
    confirm: &Confirm,
    user_in: &User,
) -> Result<(User, Uuid), StatusCode> {
    let res = client
        .post("/user/signup-confirm")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(confirm).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let user = User::from_id(db, user_in.id).await.unwrap();
    let token: SessionToken = res.json().await;
    Ok((user, Uuid::parse_str(&token.token).unwrap()))
}

pub async fn signin_user(
    _db: &DbPool,
    client: &TestClient,
    user: &User,
) -> Result<Uuid, StatusCode> {
    let signin = SignInJson {
        email: user.email.clone().unwrap(),
        password: user.password.clone(),
    };
    let res = client
        .post("/user/signin")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&signin).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let token: SessionToken = res.json().await;
    Ok(Uuid::parse_str(&token.token).unwrap())
}

pub async fn signout_user(
    _db: &DbPool,
    client: &TestClient,
    session: &Session,
) -> Result<(), StatusCode> {
    let token = SessionToken {
        token: session.token.to_string(),
    };
    let res = client
        .delete("/user/signout")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&token).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    Ok(())
}

pub async fn change_email(
    db: &DbPool,
    client: &TestClient,
    change: &ChangeEmail,
) -> Result<Uuid, StatusCode> {
    let res = client
        .post("/user/change-email")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&change).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let confirmation: Confirmation = sqlx::query("SELECT * FROM confirmation WHERE email = $1;")
        .bind(&change.new_email)
        .fetch_one(db)
        .await
        .unwrap()
        .borrow()
        .into();

    Ok(confirmation.link_id)
}

pub async fn change_email_confirm(
    db: &DbPool,
    client: &TestClient,
    confirm: &Confirm,
) -> Result<(User, Uuid), StatusCode> {
    let res = client
        .post("/user/change-email-confirm")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&confirm).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let token: SessionToken = res.json().await;
    let uuid = Uuid::parse_str(&token.token).map_err(|_| StatusCode::BAD_REQUEST)?;
    let user = User::from_session(db, uuid).await.unwrap();

    Ok((user, Uuid::parse_str(&token.token).unwrap()))
}

// Anyone testing should use this one
pub async fn initialize_user(
    db: &DbPool,
    client: &TestClient,
    user_in: &CreateUser,
) -> Result<(Uuid, User), StatusCode> {
    let signup_res = client
        .post("/user/signup")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(user_in).unwrap())
        .send()
        .await;
    assert_eq!(signup_res.status(), StatusCode::CREATED);

    let user = User::from_username(db, &user_in.username).await.unwrap();
    let confirmations = Confirmation::filter_user_id(db, user.id).await.unwrap();
    let confirmation = &confirmations[0];

    let confirm_res = client
        .post("/user/signup-confirm")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&Confirm {
                link_id: confirmation.link_id.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;
    assert_eq!(confirm_res.status(), StatusCode::CREATED);

    let user = User::from_id(db, user.id).await.unwrap();
    let session_token: SessionToken = confirm_res.json().await;
    let token = Uuid::parse_str(&session_token.token).unwrap();

    Ok((token, user))
}

//Return Type: role_id refers to ID of user

pub async fn create_group(
    _db: &DbPool,
    client: &TestClient,
    group_name: String,
    bearer: String,
) -> Result<TestResponse, sqlx::Error> {
    let res = client
        .post("/group/create")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&GroupName { name: group_name }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    Ok(res)
}

pub async fn get_user_group(
    db: &DbPool,
    group_id: i32,
    user_id: i32,
) -> Result<PgRow, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM user_group WHERE group_id = $1 AND user_id = $2;")
        .bind(group_id)
        .bind(user_id)
        .fetch_one(db)
        .await?;
    Ok(row)
}
