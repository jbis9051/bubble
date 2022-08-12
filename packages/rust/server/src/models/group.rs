use crate::types::DbPool;

use sqlx::postgres::PgRow;
use sqlx::types::chrono;

use crate::models::user::User;

use sqlx::types::Uuid;
use sqlx::Row;

pub struct Group {
    pub id: i32,
    pub uuid: Uuid,
    pub group_name: String,
    pub created: chrono::NaiveDateTime,
    pub members: Vec<Uuid>,
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

impl Group {
    pub async fn role(&self, db: &DbPool, user_id: i32) -> Result<Role, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM user_group WHERE group_id = $1 AND user_id = $2;")
            .bind(self.id)
            .bind(user_id)
            .fetch_one(db)
            .await?;
        let role_id: i32 = row.get("role_id");
        let role_enum = match Role::try_from(role_id as u8) {
            Ok(role_enum) => role_enum,
            Err(_) => return Err(sqlx::Error::RowNotFound),
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
            let user_to_be_added = match User::from_id(db, i.get("id")).await {
                Ok(user_to_be_added) => user_to_be_added,
                Err(_) => return Err(sqlx::Error::RowNotFound),
            };
            users_in_group.push(user_to_be_added);
        }
        Ok(users_in_group)
    }

    fn from_row(row: &PgRow) -> Group {
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
        let group = Self::from_row(&row);
        Ok(group)
    }

    pub async fn from_id(db: &DbPool, id: i32) -> Result<Group, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM \"group\" WHERE id = $1")
            .bind(id)
            .fetch_one(db)
            .await?;
        let group = Self::from_row(&row);
        Ok(group)
    }

    pub async fn create(&mut self, db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
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
        .bind(user_id)
        .bind(self.id)
        .bind(Role::Admin as i32)
        .execute(db)
        .await?;
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
        sqlx::query("DELETE FROM \"user_group\" WHERE group_id = $1")
            .bind(self.id)
            .execute(db)
            .await?;
        sqlx::query("DELETE FROM \"group\" WHERE id = $1")
            .bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn add_user(&mut self, db: &DbPool, user: User) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_group (user_id, group_id, role_id)
                    VALUES ($1, $2, $3);",
        )
        .bind(user.id)
        .bind(self.id)
        .bind(Role::Member as i32)
        .execute(db)
        .await?;
        self.members.push(user.uuid);
        Ok(())
    }

    pub async fn delete_user(&mut self, db: &DbPool, user: User) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_group WHERE user_id = $1 AND group_id = $2")
            .bind(user.id)
            .bind(self.id)
            .execute(db)
            .await?;
        self.members.retain(|uuid| uuid != &user.uuid);
        Ok(())
    }
}
