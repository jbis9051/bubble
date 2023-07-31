use crate::js_interface::{FrontendInstance, GlobalAccountData, GlobalStaticData};
use crate::models::kv::{AccountKv, GlobalKv};
use crate::platform::{get_default_domain, DevicePromise};
use crate::public::promise::promisify;
use crate::{Error, VIRTUAL_MEMORY};
use bridge_macro::bridge;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::{sync, thread};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::oneshot::Sender;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct TokioThread {
    pub handle: Handle,
    pub shutdown: Option<Sender<()>>,
}

impl Drop for TokioThread {
    fn drop(&mut self) {
        self.shutdown.take().map(|s| s.send(()));
    }
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
            shutdown: Some(shutdown_tx),
        }
    }
}

#[bridge]
#[derive(Deserialize, Serialize)]
pub struct InitOptions {
    pub data_directory: String,
    pub force_new: bool,
}

pub fn init(promise: DevicePromise, json: String) -> Result<(), Error> {
    /*use oslog::OsLogger;
    OsLogger::new("com.bubble.app")
           .level_filter(LevelFilter::Trace)
           .init()
           .unwrap();*/
    let options: InitOptions = serde_json::from_str(&json)?;
    let tokio_thread = TokioThread::spawn();
    let handle = tokio_thread.handle.clone();
    handle.block_on(promisify::<usize, Error>(promise, async move {
        if !options.force_new {
            let instance = VIRTUAL_MEMORY
                .clone_iter()
                .position(|m| m.static_data.data_directory == options.data_directory);
            if let Some(instance) = instance {
                return Ok(instance);
            }
        }
        let frontend_instance =
            create_frontend_instance(options.data_directory, tokio_thread).await?;
        let address = VIRTUAL_MEMORY.push(Arc::new(frontend_instance));
        Ok(address)
    }));

    Ok(())
}

pub async fn create_frontend_instance(
    data_directory: String,
    tokio_thread: TokioThread,
) -> Result<FrontendInstance, Error> {
    let (pool, account_data) = init_async(&data_directory).await?;
    let domain = GlobalKv::get(&pool, "domain").await?.unwrap();
    let global_data = GlobalStaticData {
        data_directory,
        domain,
        tokio: tokio_thread,
    };
    let frontend_instance = FrontendInstance::new(global_data, pool, account_data);
    Ok(frontend_instance)
}

pub async fn init_async(
    data_directory: &str,
) -> Result<(SqlitePool, Option<GlobalAccountData>), Error> {
    let database =
        SqlitePool::connect(&format!("sqlite:{}/global.db?mode=rwc", &data_directory)).await?;
    sqlx::migrate!("./migrations/global").run(&database).await?;

    let domain = GlobalKv::get(&database, "domain").await?;

    if domain.is_none() {
        GlobalKv::set(&database, "domain", get_default_domain()).await?;
    }

    let current_account = GlobalKv::get(&database, "current_account").await?;

    if let Some(current_account) = current_account {
        // we are logged in (or at least "current_account" exists)
        let path = format!("{}/accounts/{}.db", &data_directory, &current_account);

        if !Path::new(&path).exists() {
            GlobalKv::delete(&database, "current_account")
                .await
                .unwrap();
            return Ok((database, None));
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
            return Ok((
                database,
                Some(GlobalAccountData {
                    bearer: RwLock::new(bearer),
                    domain: domain.unwrap_or_default(),
                    user_uuid: Uuid::from_str(&current_account)
                        .map_err(|err| Error::UuidParseError("current_account", err))?,
                    database: account_database,
                    client_uuid: RwLock::new(client_uuid),
                }),
            ));
        }
    }

    Ok((database, None))
}
