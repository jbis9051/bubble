use crate::models::kv::{AccountKv, GlobalKv};
use crate::{Error, GlobalAccountData, GlobalStaticData, GLOBAL_STATIC_DATA};
use sqlx::SqlitePool;
use std::path::Path;
use std::{sync, thread};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::oneshot::Sender;
use tokio::sync::RwLock;

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

pub fn init(data_directory: String) -> Result<(), Error> {
    let tokio_thread = TokioThread::spawn();

    tokio_thread.handle.block_on(init_async(&data_directory))?;

    let global_data = GlobalStaticData {
        data_directory,
        tokio: tokio_thread,
    };

    GLOBAL_STATIC_DATA
        .set(global_data)
        .map(|_| ())
        .map_err(|_| Error::GlobalAlreadyInitialized)?;

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

        if let Some(bearer) = bearer {
            let mut write = crate::GLOBAL_ACCOUNT_DATA.write().await;
            *write = Some(GlobalAccountData {
                bearer: RwLock::new(bearer),
                database: account_database,
            });
            drop(write);
        }
    }

    Ok(())
}
