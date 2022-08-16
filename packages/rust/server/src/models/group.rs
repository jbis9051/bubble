use crate::types::DbPool;

use std::borrow::Borrow;

use crate::models::user::User;
use sqlx::postgres::PgRow;

use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use sqlx::Row;

pub struct Group {
    pub id: i32,
    pub uuid: Uuid,
    pub group_name: String,
    pub created: NaiveDateTime,
}

pub struct Members {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub role_id: Role,
    pub created: NaiveDateTime,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Role {
    Admin = 0,
    Member = 1,
}

impl TryFrom<u8> for Role {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Role::Admin),
            1 => Ok(Role::Member),
            _ => Err(()),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct UserIDs {
    pub user_ids: Vec<String>,
}

#[derive(sqlx::FromRow)]
pub struct GroupIDs {
    pub group_ids: Vec<Members>,
}

impl From<&PgRow> for Group {
    fn from(row: &PgRow) -> Self {
        Group {
            id: row.get("id"),
            uuid: row.get("uuid"),
            group_name: row.get("group_name"),
            created: row.get("created"),
        }
    }
}

impl Group {
    pub async fn delete_members(db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM members WHERE user_id = $1")
            .bind(user_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn role(&self, db: &DbPool, user_id: i32) -> Result<Role, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM member WHERE group_id = $1 AND user_id = $2;")
            .bind(self.id)
            .bind(user_id)
            .fetch_one(db)
            .await?;
        let role_id: i32 = row.get("role_id");
        //TODO
        let role_enum = Role::try_from(role_id as u8).unwrap();
        Ok(role_enum)
    }

    pub async fn members(&self, db: &DbPool) -> Result<Vec<User>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM member WHERE group_id = $1 ")
            .bind(self.id)
            .fetch_all(db)
            .await?;
        let mut users_in_group: Vec<User> = vec![];
        for i in row {
            let user_to_be_added = User::from_id(db, i.get("id")).await?;
            users_in_group.push(user_to_be_added);
        }
        Ok(users_in_group)
    }

    pub async fn from_uuid(db: &DbPool, uuid: &Uuid) -> Result<Group, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"group\" WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn from_id(db: &DbPool, id: i32) -> Result<Group, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"group\" WHERE id = $1")
            .bind(id)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn create(&mut self, db: &DbPool, user: &User) -> Result<(), sqlx::Error> {
        let tx = db.begin().await?;
        let row =
            sqlx::query("INSERT INTO \"group\" (uuid, group_name) VALUES ($1, $2) RETURNING *;")
                .bind(&self.uuid)
                .bind(&self.group_name)
                .fetch_one(db)
                .await?;
        self.id = row.get("id");
        self.created = row.get("created");
        sqlx::query(
            "INSERT INTO member (user_id, group_id, role_id)
                    VALUES ($1, $2, $3);",
        )
        .bind(user.id)
        .bind(self.id)
        .bind(Role::Admin as i32)
        .execute(db)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn update(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE \"group\" SET group_name = $1 WHERE id = $2")
            .bind(&self.group_name)
            .bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        let mut tx = db.begin().await?;
        sqlx::query("DELETE FROM member WHERE group_id = $1")
            .bind(self.id)
            .execute(&mut tx)
            .await?;
        sqlx::query("DELETE FROM location_group WHERE group_id = $1")
            .bind(self.id)
            .execute(&mut tx)
            .await?;
        sqlx::query("DELETE FROM \"group\" WHERE id = $1")
            .bind(self.id)
            .execute(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn add_user(&mut self, db: &DbPool, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO member (user_id, group_id, role_id)
                    VALUES ($1, $2, $3);",
        )
        .bind(user.id)
        .bind(self.id)
        .bind(Role::Member as i32)
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn delete_user_by_user_id(db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
        let mut tx = db.begin().await?;

        let rows = sqlx::query("DELETE FROM member WHERE user_id = $1 RETURNING *;")
            .bind(user_id)
            .fetch_all(&mut tx)
            .await?;
        for row in rows {
            let group_id: i32 = row.get("group_id");
            let groups = sqlx::query("SELECT * FROM member WHERE group_id = $1;")
                .bind(group_id)
                .fetch_all(&mut tx)
                .await?;
            if groups.is_empty() {
                sqlx::query("DELETE * FROM location_group WHERE group_id = $1;")
                    .bind(group_id)
                    .execute(&mut tx)
                    .await?;
                sqlx::query("DELETE * FROM \"group\" id = $1;")
                    .bind(group_id)
                    .execute(&mut tx)
                    .await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_user(&mut self, db: &DbPool, user: User) -> Result<(), sqlx::Error> {
        let tx = db.begin().await?;

        let members_size = sqlx::query("SELECT * FROM member WHERE group_id = $1")
            .bind(self.id)
            .fetch_all(db)
            .await?;

        let admin_size =
            sqlx::query("SELECT user_id FROM member WHERE group_id = $1 AND role_id = $2")
                .bind(self.id)
                .bind(Role::Admin as i32)
                .fetch_all(db)
                .await?;
        if admin_size.len() == 1 {
            let admin_id: i32 = admin_size[0].get("user_id");
            if admin_id == user.id && members_size.len() != 1 {
                return Err(sqlx::Error::Protocol(
                    "There must be at least one Admin in the group.".to_string(),
                ));
            }
        }

        if members_size.len() == 1 {
            Group::delete(self, db).await?;
        } else {
            sqlx::query("DELETE FROM member WHERE user_id = $1 AND group_id = $2")
                .bind(user.id)
                .bind(self.id)
                .execute(db)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
