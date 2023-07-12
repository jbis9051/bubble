use crate::mls_provider::MlsProvider;
use crate::models::kv::Kv;
use crate::types::{DbPool, SIGNATURE_SCHEME};
use common::base64;

use openmls::prelude::{Credential, CredentialType, CredentialWithKey, SignaturePublicKey};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::OpenMlsCryptoProvider;
use uuid::Uuid;

use crate::Error;

pub async fn get_this_client_mls_resources(
    user_uuid: &Uuid,
    client_uuid: &Uuid,
    account_db: &DbPool,
    mls_provider: &MlsProvider,
) -> Result<(SignatureKeyPair, CredentialWithKey), Error> {
    let client_public = base64::deserialize(
        // TODO: consider storing this directly in GlobalAccountData
        &Kv::get(account_db, "client_public_signature_key")
            .await?
            .ok_or_else(|| Error::ClientPublicSignatureNotFound)?,
    );
    let signature =
        SignatureKeyPair::read(mls_provider.key_store(), &client_public, SIGNATURE_SCHEME)
            .ok_or_else(|| Error::KeyStoreRead)?;
    let identity = format!("client_{}_{}", user_uuid, client_uuid);
    let credential = Credential::new(identity.into_bytes(), CredentialType::Basic)?;
    let credential_with_key = CredentialWithKey {
        credential,
        signature_key: SignaturePublicKey::from(signature.public()),
    };
    Ok((signature, credential_with_key))
}

#[derive(Debug, thiserror::Error)]
pub enum ParseIdentityError {}

pub fn parse_identity(identity: &[u8]) -> Result<(Uuid, Uuid), ParseIdentityError> {
    let identity = String::from_utf8(identity.to_vec()).expect("invalid identity");
    let parts: Vec<&str> = identity.split('_').collect();

    if parts.len() != 3 {
        panic!("invalid identity parts");
    }

    let user_uuid = Uuid::parse_str(parts[1]).expect("invalid user uuid");
    let client_uuid = Uuid::parse_str(parts[2]).expect("invalid client uuid");

    Ok((user_uuid, client_uuid))
}
