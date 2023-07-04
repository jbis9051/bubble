use sqlx::types::Uuid;

pub async fn create_group(_clients: Vec<Uuid>) -> Result<(), ()> {
    Ok(())
}

pub async fn get_groups() -> Result<(), ()> {
    Ok(())
}

pub async fn add_member(_group_uuid: Uuid) -> Result<(), ()> {
    Ok(())
}

pub async fn remove_member(_group_uuid: Uuid) -> Result<(), ()> {
    Ok(())
}
