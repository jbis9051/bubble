mod api;
mod application_message;
mod helper;
mod js_interface;
mod mls_provider;
mod models;
mod platform;
mod public;
mod types;
mod virtual_memory;

use std::sync::Arc;
// export all platform specific functions
pub use platform::export::*;

use crate::js_interface::FrontendInstance;

use crate::virtual_memory::VirtualMemory;

use once_cell::sync::Lazy;
use openmls::prelude::{AddMembersError, LeaveGroupError, RemoveMembersError};

use crate::helper::resource_fetcher::ResourceError;
use serde::{Serialize, Serializer};
use serde_json::json;
use sqlx::migrate::MigrateError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("sqlx migrate error: {0}")]
    SqlxMigrate(#[from] MigrateError),
    #[error("global oneshot initialized, you probably called init twice")]
    GlobalAlreadyInitialized,
    #[error("unable to parse uuid for field '{0}': {1}")]
    UuidParseError(&'static str, uuid::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("signature error: {0}")]
    Signature(#[from] ed25519_dalek::SignatureError),
    #[error("credential error: {0}")]
    Credential(#[from] openmls::prelude::CredentialError),
    #[error("tls_codec error: {0}")]
    TLS(#[from] tls_codec::Error),
    #[error("keystore error: {0}")]
    KeyStore(#[from] mls_provider::keystore::SqlxError),
    #[error("could not get account db reference")]
    DBReference,
    #[error("uuid error: {0}")]
    Uuid(#[from] uuid::Error),
    #[error("no client_public_signature found in kv table")]
    ClientPublicSignatureNotFound,
    #[error("could not read signature key pair from key store")]
    KeyStoreRead,
    #[error("identity mismatch in cache vs api")]
    IdentityMismatch,
    #[error("serde_json error {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("mls create message error: {0}")]
    CreateMessage(#[from] openmls::prelude::CreateMessageError),
    #[error("mls new group error")]
    MLSNewGroup(#[from] openmls::prelude::NewGroupError<mls_provider::keystore::SqlxError>),
    #[error("resource error: {0}")]
    Resource(#[from] ResourceError),
    #[error("add members error: {0}")]
    AddMembers(#[from] AddMembersError<mls_provider::keystore::SqlxError>),
    #[error("mls group loaded nothing")]
    MLSGroupLoad,
    #[error("remove members error: {0}")]
    RemoveMembers(#[from] RemoveMembersError<mls_provider::keystore::SqlxError>),
    #[error("leave group error: {0}")]
    LeaveGroup(#[from] LeaveGroupError),
    #[error("welcome message exists when it shouldn't")]
    UnexpectedWelcome,
    #[error("could not read client_uuid")]
    ReadClientUUID,

    #[error("don't know what to return for this error yet")]
    TestingError,
}

impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = json!({
            "message": self.to_string()
        });
        value.serialize(serializer)
    }
}

pub static VIRTUAL_MEMORY: Lazy<VirtualMemory<Arc<FrontendInstance>>> =
    Lazy::new(VirtualMemory::new);
