mod keystore;

use crate::mls_provider::keystore::MlsKeyStoreProvider;
use openmls_rust_crypto::RustCrypto;
use openmls_traits::OpenMlsCryptoProvider;
use sqlx::SqlitePool;

pub struct MlsProvider {
    crypto: RustCrypto,
    key_store: MlsKeyStoreProvider,
}

impl MlsProvider {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            crypto: RustCrypto::default(),
            key_store: MlsKeyStoreProvider::new(pool),
        }
    }
}

impl OpenMlsCryptoProvider for MlsProvider {
    type CryptoProvider = RustCrypto;
    type RandProvider = RustCrypto;
    type KeyStoreProvider = MlsKeyStoreProvider;

    fn crypto(&self) -> &Self::CryptoProvider {
        &self.crypto
    }

    fn rand(&self) -> &Self::RandProvider {
        &self.crypto
    }

    fn key_store(&self) -> &Self::KeyStoreProvider {
        &self.key_store
    }
}
