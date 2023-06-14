use sqlx::types::Uuid;

use openmls::prelude::*;
use openmls_rust_crypto::OpenMlsRustCrypto;

pub async fn create_group(clients: Vec<Uuid>) -> Result<(), ()> {
    Ok(())
}

pub async fn get_groups() -> Result<(), ()> {
    Ok(())
}

pub async fn add_member(group_uuid: Uuid) -> Result<(), ()> {
    Ok(())
}

pub async fn remove_member(group_uuid: Uuid) -> Result<(), ()> {
    Ok(())
}
