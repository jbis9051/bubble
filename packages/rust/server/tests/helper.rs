use axum_test_helper::TestClient;
use std::borrow::Borrow;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

use bubble::models::user::User;
use bubble::router;

use bubble::types::{Base64, DbPool, SIGNATURE_SCHEME};
use sqlx::postgres::PgPoolOptions;

use axum::http::StatusCode;
use bubble::models::confirmation::Confirmation;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use openmls::prelude::SignatureKeypair;
use openmls_rust_crypto::OpenMlsRustCrypto;

use bubble::routes::user::{ChangeEmail, Confirm, CreateUser, Login, SessionToken};

use bubble::models::session::Session;
use bubble::routes::client::CreateClient;
use bubble::services::email::EmailService;
use bubble::services::email::PrinterEmailService;

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
    let email_service = Arc::new(PrinterEmailService::default());
    let router = router::router(pool, email_service);

    TestClient::new(router)
}

// For user authentication testing only
pub async fn register(
    db: &DbPool,
    client: &TestClient,
    user_in: &CreateUser,
) -> Result<(User, Uuid), StatusCode> {
    let res = client
        .post("/user/register")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(user_in).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let user = User::from_username(db, &user_in.username).await.unwrap();
    let link_id = Confirmation::filter_user_id(db, user.id).await.unwrap()[0].token;

    Ok((user, link_id))
}

// For user authentication testing only
pub async fn confirm_user(
    db: &DbPool,
    client: &TestClient,
    confirm: &Confirm,
    user_in: &User,
) -> Result<(User, Uuid), StatusCode> {
    let res = client
        .patch("/user/confirm")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(confirm).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let user = User::from_id(db, user_in.id).await.unwrap();
    let token: SessionToken = res.json().await;
    Ok((user, Uuid::parse_str(&token.token).unwrap()))
}

pub async fn login(_db: &DbPool, client: &TestClient, login: &Login) -> Result<Uuid, StatusCode> {
    let res = client
        .post("/user/session")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&login).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let token: SessionToken = res.json().await;
    Ok(Uuid::parse_str(&token.token).unwrap())
}

pub async fn logout(
    _db: &DbPool,
    client: &TestClient,
    session: &Session,
) -> Result<(), StatusCode> {
    let token = SessionToken {
        token: session.token.to_string(),
    };
    let bearer = format!("Bearer {}", token.token);
    let res = client
        .delete("/user/session")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&token).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    Ok(())
}

pub async fn change_email(
    db: &DbPool,
    client: &TestClient,
    change: &ChangeEmail,
    session: &Session,
) -> Result<Uuid, StatusCode> {
    let bearer = format!("Bearer {}", session.token);
    let res = client
        .post("/user/email")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&change).unwrap())
        .header("Authorization", bearer)
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

    Ok(confirmation.token)
}

// Anyone testing should use this one
pub async fn initialize_user(
    db: &DbPool,
    client: &TestClient,
    user_in: &CreateUser,
) -> Result<(Uuid, User), StatusCode> {
    let (user, link_id) = register(db, client, user_in).await.unwrap();
    let (user, token) = confirm_user(
        db,
        client,
        &Confirm {
            token: link_id.to_string(),
        },
        &user,
    )
    .await
    .unwrap();

    Ok((token, user))
}

pub async fn create_client(
    public: &[u8],
    private: &[u8],
    bearer: &str,
    client: &TestClient,
) -> (SignatureKeypair, Uuid) {
    let backend = &OpenMlsRustCrypto::default();
    let signature_keypair = SignatureKeypair::new(SIGNATURE_SCHEME, backend).unwrap();
    let (_signature_privkey, signature_pubkey) = signature_keypair.clone().into_tuple();

    let user_keypair = Keypair {
        public: PublicKey::from_bytes(public).unwrap(),
        secret: SecretKey::from_bytes(private).unwrap(),
    };

    let signature_of_signing_key = user_keypair.sign(signature_pubkey.as_slice());

    let create_client = CreateClient {
        signing_key: Base64(signature_pubkey.as_slice().to_vec()),
        signature: Base64(signature_of_signing_key.to_bytes().to_vec()),
    };

    let res = client
        .post("/client")
        .header("Authorization", bearer)
        .json(&create_client)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);

    let client_uuid = Uuid::from_str(&res.text().await).unwrap();

    (signature_keypair, client_uuid)
}
