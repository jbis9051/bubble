use crate::types::DbPool;

use sqlx::postgres::PgRow;
use sqlx::types::chrono;

//use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use sqlx::Row;

// Based on up.sql
pub struct Group {
    //ids and uuids are mutually unique
    pub id: i32,
    pub uuid: Uuid,
    pub group_name: String,
    pub created: chrono::NaiveDateTime,
    pub members: Vec<i32>,
}

pub enum Role {
    Parent = 0,
    Child = 1,
}

#[derive(sqlx::FromRow)]
pub struct UserID {
    id: i32,
}

//Retarded issues with future
// pub async fn get_group_id(db: &DbPool, uuid: &str) -> i32{
//     let mut groupID: (i32, ) = sqlx::query_as("SELECT id FROM group WHERE uuid = $1")
//         .bind(uuid)
//         .fetch_one(db)
//         .await;
//     groupID.0
// }

pub fn get_group_by_row(row: &PgRow) -> Group {
    Group {
        id: row.get("id"),
        uuid: row.get("uuid"),
        group_name: row.get("group_name"),
        created: row.get("created"),
        members: Vec::new(),
    }
}

// CRUD functions
impl Group {
    pub async fn get_group_by_id(db: &DbPool, id: i32) -> Result<Group, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM \"group\" WHERE id = $1")
            .bind(id)
            .fetch_one(db)
            .await?;
        let group = get_group_by_row(&row);
        Ok(group)
    }

    pub async fn create(db: &DbPool, group: &mut Group) -> Result<(), sqlx::Error> {
        let row =
            sqlx::query("INSERT INTO \"group\" (uuid, group_name) VALUES ($1, $2) RETURNING *;")
                .bind(&group.uuid)
                .bind(&group.group_name)
                .fetch_one(db)
                .await?;
        let group_db = get_group_by_row(&row);
        group.created = group_db.created;
        Ok(())
    }

    //     //TENTATIVE, PoolConnection<Postgres> is not used
    pub async fn read(db: &DbPool, _group: &mut Group, uuid: Uuid) -> Result<(), sqlx::Error> {
        let row = sqlx::query("SELECT * FROM group WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(db)
            .await?;
        let _group = get_group_by_row(&row);
        // let stream_join = sqlx::query("SELECT group_id FROM user_group WHERE group_id = $1")
        //     .bind(id)
        //     .fetch(&mut connection)
        //     .await?;
        // while let Some(user_id) = stream_join.try_next().await? {
        //     group.members.append(user_id.try_get("user_id"))?;
        // }
        Ok(())
    }

    //Roles: Owner, User
    pub async fn add_users(db: &DbPool, uuid: Uuid, new_users: &[i32]) -> Result<(), sqlx::Error> {
        for i in new_users {
            let user_id: (i32,) = sqlx::query_as("SELECT id FROM user WHERE uuid = $1")
                .bind(i)
                .fetch_one(db)
                .await?;
            let group_id: (i32,) = sqlx::query_as("SELECT id FROM group WHERE uuid = $1")
                .bind(uuid)
                .fetch_one(db)
                .await?;
            sqlx::query(
                "INSERT INTO user_group (user_id, group_id, role_id)
                    VALUES ($1, $2, $3);",
            )
            .bind(user_id.0)
            .bind(group_id.0)
            //TODO! Enum is not working
            .bind(1)
            .execute(db)
            .await?;
        }
        //must perform inner join with user_group afterwards
        Ok(())
    }

    pub async fn delete_users(
        db: &DbPool,
        uuid: Uuid,
        users_to_delete: &[i32],
    ) -> Result<(), sqlx::Error> {
        let group_id: (i32,) = sqlx::query_as("SELECT id FROM group WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(db)
            .await?;

        for i in users_to_delete {
            let user_id: (i32,) = sqlx::query_as("SELECT id FROM user WHERE uuid = $1")
                .bind(i)
                .fetch_one(db)
                .await?;
            sqlx::query("DELETE FROM user_group WHERE user_id = $1 && group_id = $2")
                .bind(user_id.0)
                .bind(group_id.0)
                .execute(db)
                .await?;
        }
        Ok(())
    }

    pub async fn change_name(db: &DbPool, uuid: Uuid, name: &str) -> Result<(), sqlx::Error> {
        let group_id: (i32,) = sqlx::query_as("SELECT id FROM group WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(db)
            .await?;

        sqlx::query("UPDATE group SET group_name = $1 WHERE id = $2")
            .bind(name)
            .bind(group_id.0)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn delete_group(db: &DbPool, uuid: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM group WHERE uuid = $1")
            .bind(uuid)
            .execute(db)
            .await?;
        Ok(())
    }
}
