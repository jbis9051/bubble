/*
function get_groups(): Group[] {}
function create_group(): uuid {}
function add_member(group_uuid: uuid, user: uuid){}
function remove_member(group_uuid: uuid, user: uuid){}
function leave_group(group_uuid: uuid){}
 */

use crate::api::BubbleApi;
use crate::helper::get_user_with_cache_check;
use crate::mls_provider::MlsProvider;
use crate::models::kv::Kv;
use crate::types::SIGNATURE_SCHEME;
use crate::Error;
use crate::GLOBAL_ACCOUNT_DATA;
use common::base64;
use ed25519_dalek::{PublicKey, Signature};
use openmls::credentials::CredentialType;
use openmls::group::MlsGroup;
use openmls::prelude::{
    Credential, CredentialWithKey, GroupId, MlsGroupConfig, SignaturePublicKey,
};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::OpenMlsCryptoProvider;
use std::str::FromStr;
use uuid::Uuid;

pub async fn create_group() -> Result<Uuid, Error> {
    let global = &GLOBAL_ACCOUNT_DATA.read().await;
    let account_db = &global.as_ref().ok_or_else(|| Error::TestingError)?.database;

    let mls_provider = MlsProvider::new(account_db.clone());
    let client_public = base64::deserialize(
        &Kv::get(account_db, "client_public_signature_key")
            .await
            .map_err(|_| Error::TestingError)?
            .ok_or_else(|| Error::TestingError)?,
    );
    let signature =
        SignatureKeyPair::read(mls_provider.key_store(), &client_public, SIGNATURE_SCHEME)
            .ok_or_else(|| Error::TestingError)?;
    let credential =
        Credential::new(client_public, CredentialType::Basic).map_err(|_| Error::TestingError)?;
    let credential_with_key = CredentialWithKey {
        credential,
        signature_key: SignaturePublicKey::from(signature.public()),
    };
    let uuid = Uuid::new_v4();
    let mut group = MlsGroup::new_with_group_id(
        &mls_provider,
        &signature,
        &MlsGroupConfig::default(),
        GroupId::from_slice((uuid).as_ref()),
        credential_with_key,
    )
    .map_err(|_| Error::TestingError)?;
    group.save(&mls_provider).map_err(|_| Error::TestingError)?;
    Ok(uuid)
}

pub async fn add_member(group_uuid: Uuid, user_uuid: Uuid) -> Result<(), ()> {
    let global = GLOBAL_ACCOUNT_DATA.read().await;
    let global_data = global.as_ref().unwrap();
    let account_db = &global_data.database;
    let mls_provider = MlsProvider::new(account_db.clone());

    let client_public = base64::deserialize(
        &Kv::get(account_db, "client_public_signature_key")
            .await
            .unwrap()
            .unwrap(),
    );
    let signature =
        SignatureKeyPair::read(mls_provider.key_store(), &client_public, SIGNATURE_SCHEME).unwrap();

    let mut group =
        MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), &mls_provider).unwrap();
    let api = BubbleApi::new(
        global_data.domain.clone(),
        global_data.bearer.read().await.clone(),
    );
    let user = get_user_with_cache_check(&user_uuid, &api, account_db)
        .await
        .unwrap();
    let user_key = PublicKey::from_bytes(&user.identity).unwrap();
    let clients = api.get_user_clients(&user_uuid).await.unwrap();
    for client in &clients {
        let signature = Signature::from_bytes(&client.signature).unwrap();
        if user_key
            .verify_strict(&client.signing_key, &signature)
            .is_err()
        {
            panic!("bad client signature")
        }
    }
    let mut key_packages = Vec::with_capacity(clients.len());
    for client in &clients {
        let client_uuid = Uuid::from_str(&client.uuid).unwrap();
        let key_package = api.request_key_package(&client_uuid).await.unwrap();
        key_packages.push(key_package);
    }
    let (_mls_message_out, _welcome_out, _group_info) = group
        .add_members(&mls_provider, &signature, &key_packages)
        .unwrap();
    todo!()
}
