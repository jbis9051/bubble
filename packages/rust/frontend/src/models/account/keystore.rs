use crate::types::DbPool;
use openmls_traits::key_store::MlsEntityId;
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;

pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

pub enum InternalMlsEntityId {
    SignatureKeyPair,
    HpkePrivateKey,
    KeyPackage,
    PskBundle,
    EncryptionKeyPair,
    GroupState,
}

impl InternalMlsEntityId {
    pub fn as_str(&self) -> &'static str {
        match self {
            InternalMlsEntityId::SignatureKeyPair => "signature_key_pair",
            InternalMlsEntityId::HpkePrivateKey => "hpke_private_key",
            InternalMlsEntityId::KeyPackage => "key_package",
            InternalMlsEntityId::PskBundle => "psk_bundle",
            InternalMlsEntityId::EncryptionKeyPair => "encryption_key_pair",
            InternalMlsEntityId::GroupState => "group_state",
        }
    }
}

impl From<&str> for InternalMlsEntityId {
    fn from(value: &str) -> Self {
        match value {
            "signature_key_pair" => Self::SignatureKeyPair,
            "hpke_private_key" => Self::HpkePrivateKey,
            "key_package" => Self::KeyPackage,
            "psk_bundle" => Self::PskBundle,
            "encryption_key_pair" => Self::EncryptionKeyPair,
            "group_state" => Self::GroupState,
            _ => panic!("Invalid MlsEntityId"),
        }
    }
}

impl From<MlsEntityId> for InternalMlsEntityId {
    fn from(value: MlsEntityId) -> Self {
        match value {
            MlsEntityId::SignatureKeyPair => Self::SignatureKeyPair,
            MlsEntityId::HpkePrivateKey => Self::HpkePrivateKey,
            MlsEntityId::KeyPackage => Self::KeyPackage,
            MlsEntityId::PskBundle => Self::PskBundle,
            MlsEntityId::EncryptionKeyPair => Self::EncryptionKeyPair,
            MlsEntityId::GroupState => Self::GroupState,
        }
    }
}

pub struct KeyStore {
    pub id: i32,
    pub key: Key,
    pub value: Value,
    pub type_name: InternalMlsEntityId,
    pub created: NaiveDateTime,
}

impl From<&SqliteRow> for KeyStore {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            key: row.get("key"),
            value: row.get("value"),
            type_name: row.get::<&str, _>("type_name").into(),
            created: row.get("created"),
        }
    }
}

impl KeyStore {
    pub async fn get(db: &DbPool, key: &[u8]) -> Result<Option<Value>, sqlx::Error> {
        sqlx::query("SELECT value FROM keystore WHERE key = $1;")
            .bind(key)
            .fetch_optional(db)
            .await
            .map(|row| row.map(|row| row.get("value")))
    }

    pub async fn set(
        db: &DbPool,
        key: &[u8],
        value: &[u8],
        type_name: InternalMlsEntityId,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM keystore WHERE key = $1; INSERT INTO kv (key, value, type_name) VALUES ($1,$2, $3);")
            .bind(key)
            .bind(value)
            .bind(type_name.as_str())
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn delete(db: &DbPool, key: &[u8]) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM keystore WHERE key = $1;")
            .bind(key)
            .execute(db)
            .await?;
        Ok(())
    }
}
