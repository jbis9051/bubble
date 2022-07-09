use std::time::{SystemTime};
use crate::DbPool;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgRow, Postgres};
use sqlx::{Error, Row};
// Based on up.sql
pub struct Group {
    //ids and uuids are mutually unique
    pub id: i32,
    pub uuid: String,
    pub group_name: String,
    pub created: String,
    pub members: Vec<i32>,
}

// CRUD functions
impl Group {
    pub async fn create(db: &DbPool, group: &Group){
        sqlx::query("INSERT INTO group($1, $2, $3, $4)")
            .bind(&group.id)
            .bind(&group.uuid)
            .bind(&group.group_name)
            .bind(&group.created)
            .execute(db)
            .await?;
    }/*
    pub fn read(mut connection: PoolConnection<Postgres>, uuid: String) {

    }
    pub fn add_users(&mut self, uuid: String, mut new_users: &[i32]) {
        *self.members.append(&mut new_user);
    }
    pub fn delete_users(uuid: String, users_to_delete: Vec<i32>) {

    }
    pub fn change_name(uuid: String, name: String) {

    }
    pub fn delete_group(uuid: String) {

    }*/
}