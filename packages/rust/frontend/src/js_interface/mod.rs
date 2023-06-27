use crate::export;
use crate::public::init::TokioThread;
use bridge_macro::bridge;
use sqlx::SqlitePool;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod group;
pub mod location;
//pub mod user;

#[derive(Debug)]
pub struct GlobalStaticData {
    pub data_directory: String,
    pub tokio: TokioThread,
}

#[derive(Debug)]
pub struct GlobalAccountData {
    pub database: SqlitePool,
    pub bearer: RwLock<String>,            // cached value
    pub domain: String,                    // cached value
    pub user_uuid: Uuid,                   // cached value
    pub client_uuid: RwLock<Option<Uuid>>, // cached value
}

pub struct FrontendInstance {
    pub(crate) static_data: GlobalStaticData,
    global_database: SqlitePool,
    account_data: RwLock<Option<GlobalAccountData>>,
}

impl FrontendInstance {
    pub fn new(
        static_data: GlobalStaticData,
        global_database: SqlitePool,
        account_data: Option<GlobalAccountData>,
    ) -> Self {
        Self {
            static_data,
            global_database,
            account_data: RwLock::new(account_data),
        }
    }
}

impl FrontendInstance {
    #[bridge]
    pub async fn multiply(&self, a: i32, b: i32) -> Result<i32, ()> {
        Ok(a * b)
    }
}

export!(
    FrontendInstance,
    multiply(a: i32, b: i32) -> Result<i32, ()>;
);
