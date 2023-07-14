use serde::Serialize;
use crate::export;
use crate::public::init::TokioThread;
use bridge_macro::bridge;
use sqlx::SqlitePool;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod client;
pub mod group;
pub mod location;
pub mod message;
pub mod user;

#[derive(Debug)]
pub struct GlobalStaticData {
    pub data_directory: String,
    pub domain: String,
    pub tokio: TokioThread,
}

#[derive(Debug)]
pub struct GlobalAccountData {
    pub database: SqlitePool,
    pub bearer: RwLock<String>,
    // cached value
    pub domain: String,
    // cached value
    pub user_uuid: Uuid,
    // cached value
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

#[derive(Serialize)]
#[bridge]
pub struct AccountData {
    pub domain: String,
    pub user_uuid: Uuid,
    pub client_uuid: Option<Uuid>,
}

#[derive(Serialize)]
#[bridge]
pub struct Status {
    pub domain: String,
    pub data_directory: String,
    pub account_data: Option<AccountData>,
}

impl FrontendInstance {
    #[bridge]
    pub async fn status(&self) -> Result<Status, ()> {
        let account_data = self.account_data.read().await;


        let account_data_out= if let Some(account_data) = account_data.as_ref() {
            Some(AccountData {
                domain: account_data.domain.clone(),
                user_uuid: account_data.user_uuid,
                client_uuid: *account_data.client_uuid.read().await,
            })
        } else {
            None
        };

        Ok(Status {
            domain: self.static_data.domain.clone(),
            data_directory: self.static_data.data_directory.clone(),
            account_data: account_data_out,
        })
    }
}

use crate::application_message::Location;
use crate::js_interface::group::Group;
use crate::js_interface::user::UserOut;
use crate::Error;

export!(
    FrontendInstance,
    status() -> Result<Status, ()>;
    // user
    register(
        username: String,
        password: String,
        name: String,
        email: String
    ) -> Result<(), Error>;
    login(username_or_email: String, password: String) -> Result<Uuid, Error>;
    logout() -> Result<(), Error>;
    // group
    get_groups() -> Result<Vec<Group>, Error>;
    create_group() -> Result<Uuid, Error>;
    add_member(group_uuid: Uuid, user_uuid: Uuid) -> Result<(), Error>;
    remove_member(group_uuid: Uuid, user_uuid: Uuid) -> Result<(), Error>;
    leave_group(group_uuid: Uuid) -> Result<(), Error>;
    // message
    receive_messages() -> Result<(), Error>;
    // location
    get_location(
        group_uuid: Uuid,
        client: Uuid,
        before_timestamp: i64,
        amount: u32
    ) -> Result<Vec<Location>, ()>;
    get_num_location(
        group_uuid: Uuid,
        client: Uuid,
        from_timestamp: i64,
        to_timestamp: i64
    ) -> Result<i64, ()>;
    send_location(
        group_uuid: Uuid,
        longitude: f64,
        latitude: f64,
        timestamp: i64
    ) -> Result<(), ()>;
    // clients
    replace_key_packages() -> Result<(), Error>;
    search(query: String) -> Result<Vec<UserOut>, Error>;
);
