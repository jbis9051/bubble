use crate::api::BubbleApi;
use crate::mls_provider::MlsProvider;
use crate::models::account::user::User;
use crate::models::kv::Kv;
use crate::types::{DbPool, SIGNATURE_SCHEME};
use common::base64;
use common::http_types::{PublicClient, PublicUser};
use ed25519_dalek::{PublicKey, Signature};
use openmls::prelude::{
    Credential, CredentialType, CredentialWithKey, SignaturePublicKey,
};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::OpenMlsCryptoProvider;
use uuid::Uuid;

pub async fn get_user_with_cache_check(
    uuid: &Uuid,
    api: &BubbleApi,
    db: &DbPool,
) -> Result<PublicUser, ()> {
    let local_user = User::try_from_uuid(db, uuid).await.unwrap();
    let api_user = api.get_user(uuid).await.unwrap();

    if let Some(user) = local_user {
        if user.identity != *api_user.identity {
            panic!("User identity mismatch in cache vs api");
        }
    }

    Ok(api_user)
}

pub async fn get_this_client_mls_resources(
    account_db: &DbPool,
    mls_provider: &MlsProvider,
) -> Result<(SignatureKeyPair, CredentialWithKey), ()> {
    let client_public = base64::deserialize(
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

pub async fn get_clients_authenticated(
    user_uuid: &Uuid,
    api: &BubbleApi,
    account_db: &DbPool,
) -> Result<Vec<PublicClient>, ()> {
    let user = get_user_with_cache_check(user_uuid, api, account_db)
        .await
        .unwrap();
    let user_key = PublicKey::from_bytes(&user.identity).unwrap();
    let clients = api.get_user_clients(user_uuid).await.unwrap();
    for client in &clients {
        let signature = Signature::from_bytes(&client.signature).unwrap();
        if user_key
            .verify_strict(&client.signing_key, &signature)
            .is_err()
        {
            panic!("bad client signature")
        }
    }
    Ok(clients)
}
