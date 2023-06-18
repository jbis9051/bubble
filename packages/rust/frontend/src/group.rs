/*
function get_groups(): Group[] {}
function create_group(): uuid {}
function add_member(group_uuid: uuid, user: uuid){}
function remove_member(group_uuid: uuid, user: uuid){}
function leave_group(group_uuid: uuid){}
 */

use crate::mls_provider::MlsProvider;
use crate::models::kv::Kv;
use crate::types::SIGNATURE_SCHEME;
use crate::GLOBAL_ACCOUNT_DATA;
use common::base64;
use openmls::credentials::CredentialType;
use openmls::group::MlsGroup;
use openmls::prelude::{
    Credential, CredentialWithKey, GroupId, MlsGroupConfig, SignaturePublicKey,
};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::OpenMlsCryptoProvider;
use uuid::Uuid;

pub async fn create_group() -> Result<Uuid, ()> {
    let global = GLOBAL_ACCOUNT_DATA.read().await;
    let account_db = &global.as_ref().unwrap().database;
    let mls_provider = MlsProvider::new(account_db.clone());
    let client_public = base64::deserialize(
        &Kv::get(account_db, "client_public_signature_key")
            .await
            .unwrap()
            .unwrap(),
    );
    let signature =
        SignatureKeyPair::read(mls_provider.key_store(), &client_public, SIGNATURE_SCHEME).unwrap();
    let credential = Credential::new(b"".to_vec(), CredentialType::Basic).unwrap();
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
    .unwrap();
    group.save(&mls_provider).unwrap();
    Ok(uuid)
}
