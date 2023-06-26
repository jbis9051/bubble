use crate::mls_provider::MlsProvider;
use crate::models::kv::Kv;
use crate::types::{DbPool, SIGNATURE_SCHEME};
use common::base64;
use openmls::prelude::{Credential, CredentialType, CredentialWithKey, SignaturePublicKey};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::OpenMlsCryptoProvider;

pub async fn get_this_client_mls_resources(
    account_db: &DbPool,
    mls_provider: &MlsProvider,
) -> Result<(SignatureKeyPair, CredentialWithKey), ()> {
    let client_public = base64::deserialize(
        // TODO: consider storing this directly in GlobalAccountData
        &Kv::get(account_db, "client_public_signature_key")
            .await
            .unwrap()
            .unwrap(),
    );
    let signature =
        SignatureKeyPair::read(mls_provider.key_store(), &client_public, SIGNATURE_SCHEME).unwrap();
    let credential = Credential::new(client_public, CredentialType::Basic).unwrap();
    let credential_with_key = CredentialWithKey {
        credential,
        signature_key: SignaturePublicKey::from(signature.public()),
    };
    Ok((signature, credential_with_key))
}
