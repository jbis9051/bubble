use crate::models::account::keystore::{InternalMlsEntityId, KeyStore};
use openmls::prelude::OpenMlsKeyStore;
use openmls_traits::key_store::MlsEntity;
use sqlx::SqlitePool;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::thread;
use tokio::runtime::{Builder, Handle};

pub struct MlsKeyStoreProvider {
    db: SqlitePool,
}

impl MlsKeyStoreProvider {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

#[derive(Debug)]
pub struct SqlxError(sqlx::Error);

impl From<sqlx::Error> for SqlxError {
    fn from(e: sqlx::Error) -> Self {
        Self(e)
    }
}

impl PartialEq for SqlxError {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string() // TODO: This is not a good way to compare errors
    }
}

impl Display for SqlxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Error for SqlxError {}

impl OpenMlsKeyStore for MlsKeyStoreProvider {
    type Error = SqlxError;

    fn store<V: MlsEntity>(&self, k: &[u8], v: &V) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        let internal_id: InternalMlsEntityId = V::ID.into();
        let v = serde_json::to_vec(v).unwrap();
        let _handle = Handle::current();
        let db = self.db.clone();
        let k = k.to_vec();
        thread::spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async move {
                KeyStore::set(&db, &k, &v, internal_id).await.unwrap();
            });
        })
        .join()
        .unwrap();
        Ok(())
    }

    fn read<V: MlsEntity>(&self, k: &[u8]) -> Option<V>
    where
        Self: Sized,
    {
        let db = self.db.clone();
        let k = k.to_vec();
        let v: Option<V> = thread::spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async move { KeyStore::get(&db, &k).await })
        })
        .join()
        .unwrap()
        .unwrap()
        .map(|v| serde_json::from_slice(&v).unwrap());
        v
    }

    fn delete<V: MlsEntity>(&self, k: &[u8]) -> Result<(), Self::Error> {
        Ok(Handle::current().block_on(KeyStore::delete(&self.db, k))?)
    }
}
