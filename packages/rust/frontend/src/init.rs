use crate::models::kv::{AccountKv, GlobalKv};
use crate::platform::DevicePromise;
use crate::promise::Promise;
use crate::{Error, GlobalAccountData, GlobalStaticData, GLOBAL_STATIC_DATA};
use serde_json::json;
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;
use std::{sync, thread};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::oneshot::Sender;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct TokioThread {
    pub handle: Handle,
    pub shutdown: Sender<()>,
}

impl TokioThread {
    pub fn spawn() -> Self {
        let (handle_send, handle_recv) = sync::mpsc::channel();
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

        thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            handle_send.send(runtime.handle().clone()).unwrap();
            runtime.block_on(async {
                shutdown_rx.await.unwrap();
            });
        });

        let handle = handle_recv.recv().unwrap();

        Self {
            handle,
            shutdown: shutdown_tx,
        }
    }
}

pub fn init(promise: DevicePromise, data_directory: String) -> Result<(), Error> {
    let tokio_thread = TokioThread::spawn();
    /*

    tokio_thread.handle.block_on(promisify::<(), Error>(promise, async move {
        //init_async(&data_directory).await?;
        Ok(())
    }));*/

    let global_data = GlobalStaticData {
        data_directory,
        tokio: tokio_thread,
    };

    GLOBAL_STATIC_DATA
        .set(global_data)
        .map_err(|_| Error::GlobalAlreadyInitialized)?;

    promise.resolve(&json!({"status": true, "value": null}).to_string());

    Ok(())
}

pub async fn init_async(data_directory: &str) -> Result<(), Error> {
    let database =
        SqlitePool::connect(&format!("sqlite:{}/global.db?mode=rwc", &data_directory)).await?;
    sqlx::migrate!("./migrations/global").run(&database).await?;

    crate::GLOBAL_DATABASE
        .set(database.clone())
        .map(|_| ())
        .map_err(|_| Error::GlobalAlreadyInitialized)?;

    let current_account = GlobalKv::get(&database, "current_account").await?;

    if let Some(current_account) = current_account {
        let path = format!("{}/accounts/{}.db", &data_directory, &current_account);

        if !Path::new(&path).exists() {
            GlobalKv::delete(&database, "current_account")
                .await
                .unwrap();
            return Ok(());
        }

        let account_database = SqlitePool::connect(&format!("sqlite:{}", &path)).await?;

        sqlx::migrate!("./migrations/account")
            .run(&account_database)
            .await?;

        let bearer = AccountKv::get(&account_database, "bearer").await?;
        let domain = AccountKv::get(&account_database, "domain").await?;
        let client_uuid = {
            let client_uuid = AccountKv::get(&account_database, "client_uuid").await?;
            if let Some(client_uuid) = client_uuid {
                Some(
                    Uuid::from_str(&client_uuid)
                        .map_err(|err| Error::UuidParseError("client_uuid", err))?,
                )
            } else {
                None
            }
        };

        if let Some(bearer) = bearer {
            let mut write = crate::GLOBAL_ACCOUNT_DATA.write().await;
            *write = Some(GlobalAccountData {
                bearer: RwLock::new(bearer),
                domain: domain.unwrap_or_default(),
                user_uuid: Uuid::from_str(&current_account)
                    .map_err(|err| Error::UuidParseError("current_account", err))?,
                database: account_database,
                client_uuid: RwLock::new(client_uuid),
            });
            drop(write);
        }
    }

    Ok(())
}
