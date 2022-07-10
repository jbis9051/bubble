use std::time::{SystemTime};
use crate::DbPool;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgRow, Postgres};
use sqlx::{Error, Row};
use futures::TryStreamExt;

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
    pub async fn create(db: &DbPool, group: &Group) {
        sqlx::query("INSERT INTO group($1, $2, $3, $4)")
            .bind(&group.id)
            .bind(&group.uuid)
            .bind(&group.group_name)
            .bind(&group.created)
            .execute(db)
            .await?;
    }
    //TENTATIVE, PoolConnection<Postgres> is not used
    pub async fn read(db: &DbPool, id: i32) -> Result<Group, Error> {
        let stream = sqlx::query("SELECT * FROM group WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;
        let group = call_user(row);
         let stream_join = sqlx::query("SELECT group_id FROM user_group WHERE group_id = $1")
             .bind(id)
             .fetch(&mut connection)
             .await?;
         while let Some(user_id)  = stream_join.try_next().await?{
             group.members.append(user_id.try_get("user_id"))?;
         }
        Ok(group)
    }
    pub fn call_user(row: PgRow) -> Group{
        Group {
            id: row.get("id"),
            uuid: row.get("uuid"),
            group_name: row.get("group_name"),
            created: row.get("created"),
            members: (),
        }
    }
    /*
    pub fn add_users( uuid: String, mut new_users: &[i32]) {
        /self.members.append(&mut new_user);

    }
    pub fn delete_users(uuid: String, users_to_delete: Vec<i32>) {

    }
    pub fn change_name(uuid: String, name: String) {

    }
    pub fn delete_group(uuid: String) {

    }*/
}