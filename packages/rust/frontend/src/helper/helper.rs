use crate::mls_provider::MlsProvider;
use crate::models::kv::Kv;
use crate::types::{DbPool, SIGNATURE_SCHEME};
use common::base64;
use common::http_types::{PublicClient, PublicUser};
use ed25519_dalek::{PublicKey, Signature};
use openmls::prelude::{Credential, CredentialType, CredentialWithKey, SignaturePublicKey};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::OpenMlsCryptoProvider;
use uuid::Uuid;

use crate::Error;

pub async fn get_user_with_cache_check(
    uuid: &Uuid,
    api: &BubbleApi,
    db: &DbPool,
) -> Result<PublicUser, Error> {
    let local_user = User::try_from_uuid(db, uuid).await?;
    let api_user = api.get_user(uuid).await?;

    if let Some(user) = local_user {
        if user.identity != *api_user.identity {
            return Err(Error::IdentityMismatch);
        }
    }

    Ok(api_user)
}

pub async fn get_this_client_mls_resources(
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
    let credential = Credential::new(client_public, CredentialType::Basic)?;
    let credential_with_key = CredentialWithKey {
        credential,
        signature_key: SignaturePublicKey::from(signature.public()),
    };
    Ok((signature, credential_with_key))
}

pub async fn get_clients_authenticated(
    user_uuid: &Uuid,
    api: &BubbleApi,
    account_db: &DbPool,
) -> Result<Vec<PublicClient>, Error> {
    let user = get_user_with_cache_check(user_uuid, api, account_db).await?;
    let user_key = PublicKey::from_bytes(&user.identity)?;
    let clients = api.get_user_clients(user_uuid).await?;
    for client in &clients {
        let signature = Signature::from_bytes(&client.signature)?;
        user_key.verify_strict(&client.signing_key, &signature)?;
    }
    Ok(clients)
}
