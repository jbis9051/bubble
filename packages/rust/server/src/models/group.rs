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

pub struct UserGroup {
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
    pub group_ids: Vec<UserGroup>,
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
    pub async fn delete_user_groups(db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_group WHERE user_id = $1")
            .bind(user_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn role(&self, db: &DbPool, user_id: i32) -> Result<Role, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM user_group WHERE group_id = $1 AND user_id = $2;")
            .bind(self.id)
            .bind(user_id)
            .fetch_one(db)
            .await?;
        let role_id: i32 = row.get("role_id");
        let role_enum = match Role::try_from(role_id as u8) {
            Ok(role_enum) => role_enum,
            Err(_) => {
                return Err(sqlx::Error::TypeNotFound {
                    type_name: "Role".to_owned(),
                })
            }
        };
        Ok(role_enum)
    }

    pub async fn members(&self, db: &DbPool) -> Result<Vec<User>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM user_group WHERE group_id = $1 ")
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
            "INSERT INTO user_group (user_id, group_id, role_id)
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
        sqlx::query("DELETE FROM user_group WHERE group_id = $1")
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
            "INSERT INTO user_group (user_id, group_id, role_id)
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

        let rows = sqlx::query("DELETE FROM user_group WHERE user_id = $1 RETURNING *;")
            .bind(user_id)
            .fetch_all(&mut tx)
            .await?;
        for row in rows {
            let group_id: i32 = row.get("group_id");
            let groups = sqlx::query("SELECT * FROM user_group WHERE group_id = $1;")
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
        let table_size = sqlx::query("SELECT * FROM user_group WHERE group_id = $1")
            .bind(self.id)
            .fetch_all(db)
            .await?;

        if table_size.len() == 1 {
            Group::delete(self, db);
        } else {
            sqlx::query("DELETE FROM user_group WHERE user_id = $1 AND group_id = $2")
                .bind(user.id)
                .bind(self.id)
                .execute(db)
                .await?;
        }

        Ok(())
    }
}
