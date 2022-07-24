use crate::types::DbPool;

use sqlx::postgres::PgRow;
use sqlx::types::chrono;

use crate::extractor::AuthenticatedUser::AuthenticatedUser;
use sqlx::types::Uuid;
use sqlx::Row;

pub struct Group {
    pub id: i32,
    pub uuid: Uuid,
    pub group_name: String,
    pub created: chrono::NaiveDateTime,
    pub members: Vec<Uuid>,
}

#[repr(u8)]
pub enum Role {
    Admin = 0,
    Child = 1,
}

#[derive(sqlx::FromRow)]
pub struct UserID {
    id: i32,
}

pub fn get_group_by_row(row: &PgRow) -> Group {
    Group {
        id: row.get("id"),
        uuid: row.get("uuid"),
        group_name: row.get("group_name"),
        created: row.get("created"),
        members: Vec::new(),
    }
}

pub async fn from_uuid(db: &DbPool, uuid: Uuid) -> Result<Group, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM \"group\" WHERE uuid = $1")
        .bind(uuid)
        .fetch_one(db)
        .await?;
    let group = get_group_by_row(&row);
    Ok(group)
}

pub async fn authorize_user(db: &DbPool, user_id: i32) -> Result<bool, sqlx::Error> {
    let role_id: (i32,) = sqlx::query_as("SELECT role_id FROM user_group WHERE id = $1")
        .bind(user_id)
        .fetch_one(db)
        .await?;
    Ok(role_id.0 == (Role::Admin as i32))
}

impl Group {
    //duplication necesssary for testing, must eventually resolve
    async fn from_uuid(db: &DbPool, uuid: Uuid) -> Result<Group, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM \"group\" WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(db)
            .await?;
        let group = get_group_by_row(&row);
        Ok(group)
    }

    pub async fn from_id(db: &DbPool, id: i32) -> Result<Group, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM \"group\" WHERE id = $1")
            .bind(id)
            .fetch_one(db)
            .await?;
        let group = get_group_by_row(&row);
        Ok(group)
    }

    pub async fn create(
        db: &DbPool,
        group: &mut Group,
        AuthenticatedUser(user): &AuthenticatedUser,
    ) -> Result<(), sqlx::Error> {
        if !authorize_user(db, user.id).await.unwrap() {
            return Ok(());
        };
        let row =
            sqlx::query("INSERT INTO \"group\" (uuid, group_name) VALUES ($1, $2) RETURNING *;")
                .bind(&group.uuid)
                .bind(&group.group_name)
                .fetch_one(db)
                .await?;
        let group_db = get_group_by_row(&row);
        group.created = group_db.created;
        sqlx::query(
            "INSERT INTO user_group (user_id, group_id, role_id)
                    VALUES ($1, $2, $3);",
        )
        .bind(user.id)
        .bind(group.id)
        .bind(Role::Admin as i32)
        .execute(db)
        .await?;
        Ok(())
    }

    //TODO read_users could be a new function to return only user list or other public information for non parents
    pub async fn read(
        db: &DbPool,
        _group: &mut Group,
        uuid: Uuid,
        AuthenticatedUser(user): &AuthenticatedUser,
    ) -> Result<(), sqlx::Error> {
        if !authorize_user(db, user.id).await.unwrap() {
            return Ok(());
        };
        // changes reference to group, does not use from_id
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
    pub async fn add_user(
        db: &DbPool,
        uuid: Uuid,
        new_users: Uuid,
        AuthenticatedUser(user): &AuthenticatedUser,
    ) -> Result<(), sqlx::Error> {
        if !authorize_user(db, user.id).await.unwrap() {
            return Ok(());
        };
        let _group = from_uuid(db, uuid).await?;
        let user_id: (i32,) = sqlx::query_as("SELECT id FROM user WHERE uuid = $1")
            .bind(new_users)
            .fetch_one(db)
            .await?;
        sqlx::query(
            "INSERT INTO user_group (user_id, group_id, role_id)
                    VALUES ($1, $2, $3);",
        )
        .bind(user_id.0)
        .bind(_group.id)
        .bind(Role::Admin as i32)
        .execute(db)
        .await?;

        //must perform inner join with user_group afterwards
        Ok(())
    }

    pub async fn delete_user(
        db: &DbPool,
        uuid: Uuid,
        user_to_delete: Uuid,
        AuthenticatedUser(user): &AuthenticatedUser,
    ) -> Result<(), sqlx::Error> {
        if !authorize_user(db, user.id).await.unwrap() {
            return Ok(());
        };
        let group = from_uuid(db, uuid).await?;

        let user_id: (i32,) = sqlx::query_as("SELECT id FROM user WHERE uuid = $1")
            .bind(user_to_delete)
            .fetch_one(db)
            .await?;
        sqlx::query("DELETE FROM user_group WHERE user_id = $1 && group_id = $2")
            .bind(user_id.0)
            .bind(group.id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn change_name(
        db: &DbPool,
        uuid: Uuid,
        name: &str,
        AuthenticatedUser(user): &AuthenticatedUser,
    ) -> Result<(), sqlx::Error> {
        if !authorize_user(db, user.id).await.unwrap() {
            return Ok(());
        };
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

    pub async fn delete_group(
        db: &DbPool,
        uuid: Uuid,
        AuthenticatedUser(user): &AuthenticatedUser,
    ) -> Result<(), sqlx::Error> {
        if !authorize_user(db, user.id).await.unwrap() {
            return Ok(());
        };
        sqlx::query("DELETE FROM group WHERE uuid = $1")
            .bind(uuid)
            .execute(db)
            .await?;
        Ok(())
    }
}
