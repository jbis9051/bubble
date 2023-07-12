use crate::services::email::EmailService;

use openmls::prelude::{Ciphersuite, SignatureScheme};

use sqlx::{Pool, Postgres};

use std::sync::Arc;

pub const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::ED25519;

pub const CIPHERSUITES: Ciphersuite =
    Ciphersuite::MLS_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519;

pub type DbPool = Pool<Postgres>;

pub type EmailServiceArc = Arc<(dyn EmailService + Send + Sync)>;
