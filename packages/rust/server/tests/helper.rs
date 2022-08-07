use axum_test_helper::{TestClient, TestResponse};

use bubble::models::user::User;
use bubble::router;

use bubble::routes::group::GroupName;
use bubble::types::DbPool;
use sqlx::postgres::{PgPoolOptions, PgRow};

use axum::http::StatusCode;
use bubble::models::confirmation::Confirmation;

use bubble::routes::user::{Confirm, CreateUser, SessionToken};

use std::future::Future;
use std::{env, thread};
use tokio::runtime::Runtime;
use uuid::Uuid;

#[macro_export]
macro_rules! cleanup {
    ($structdef: tt, |$dbarg:ident, $resourcesarg: ident| $code: expr) => {{
        #[derive(Default)]
        struct CleanupResources
            $structdef

        let cleanup_resources = CleanupResources::default();

        use sqlx::postgres::PgPoolOptions;
        use std::env;

        async fn cleanup($resourcesarg: CleanupResources) {
            let $dbarg = PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DB_URL").unwrap())
                .await
                .unwrap();
            $code
        }

        $crate::helper::Cleanup::new(Box::new(cleanup), cleanup_resources)
    }};

    (|$dbarg:ident| $code: expr) => {{
        use sqlx::postgres::PgPoolOptions;
        use std::env;

        async fn cleanup(_: ()) {
            let $dbarg = PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DB_URL").unwrap())
                .await
                .unwrap();
            $code
        }

        $crate::helper::Cleanup::new(Box::new(cleanup), ())
    }}
}

type CB<F, Resources> = Box<dyn Fn(Resources) -> F>;

pub struct Cleanup<F: 'static + Future + Send, Resources: Default> {
    cleanup: Option<CB<F, Resources>>,
    pub resources: Resources,
}

impl<F: 'static + Future + Send, Resources: Default> Cleanup<F, Resources> {
    pub fn new(cleanup: CB<F, Resources>, resources: Resources) -> Self {
        Self {
            cleanup: Some(cleanup),
            resources,
        }
    }
}
impl<F: 'static + Future + Send, Resources: Default> Drop for Cleanup<F, Resources> {
    fn drop(&mut self) {
        let cleanup = self.cleanup.take().unwrap();
        let resources = std::mem::take(&mut self.resources);
        let future = cleanup(resources);
        thread::spawn(|| {
            Runtime::new().unwrap().block_on(future);
        })
        .join();
    }
}

pub async fn start_server() -> (DbPool, TestClient) {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DB_URL").unwrap())
        .await
        .unwrap();

    let pool2 = pool.clone();

    let router = router::router(pool);

    (pool2, TestClient::new(router))
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

    let session_token: SessionToken = confirm_res.json().await;
    let token = Uuid::parse_str(&session_token.token).unwrap();

    Ok((token, user))
}

//Return Type: role_id refers to ID of user

pub async fn create_group(
    _db: &DbPool,
    client: &TestClient,
    group_name: &str,
    bearer: String,
) -> Result<TestResponse, sqlx::Error> {
    let group_name: String = group_name.parse().unwrap();
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
