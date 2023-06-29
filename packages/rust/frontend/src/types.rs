use once_cell::sync::Lazy;
use openmls::group::MlsGroupConfig;
use openmls_traits::types::{Ciphersuite, SignatureScheme};
use sqlx::SqlitePool;

pub type DbPool = SqlitePool;

pub const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::ED25519;

pub const CIPHERSUITE: Ciphersuite =
    Ciphersuite::MLS_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519;

pub static MLS_GROUP_CONFIG: Lazy<MlsGroupConfig> = Lazy::new(|| {
    MlsGroupConfig::builder()
        .use_ratchet_tree_extension(true)
        .build()
});
