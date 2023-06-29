use crate::js_interface::{FrontendInstance, GlobalAccountData, GlobalStaticData};
use crate::models::kv::{AccountKv, GlobalKv};
use crate::platform::DevicePromise;
use crate::public::promise::promisify;
use crate::{Error, VIRTUAL_MEMORY};
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
    let handle = tokio_thread.handle.clone();

    handle.block_on(promisify::<usize, Error>(promise, async move {
        let (pool, account_data) = init_async(&data_directory).await?;
        let global_data = GlobalStaticData {
            data_directory,
            tokio: tokio_thread,
        };
        let frontend_instance = FrontendInstance::new(global_data, pool, account_data);
        let address = VIRTUAL_MEMORY.push(Arc::new(frontend_instance));
        Ok(address)
    }));

    Ok(())
}

pub async fn init_async(
    data_directory: &str,
) -> Result<(SqlitePool, Option<GlobalAccountData>), Error> {
    let database =
        SqlitePool::connect(&format!("sqlite:{}/global.db?mode=rwc", &data_directory)).await?;
    sqlx::migrate!("./migrations/global").run(&database).await?;

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
