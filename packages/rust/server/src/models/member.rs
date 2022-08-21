use crate::types::DbPool;
use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;

use std::borrow::Borrow;

pub struct Member {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub role_id: Role,
    pub created: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Role {
    Admin = 0x00,
    Member = 0x01,
}

impl TryFrom<u8> for Role {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Role::Admin),
            0x01 => Ok(Role::Member),
            _ => Err(()),
        }
    }
}
impl Role {
    fn to_int(self) -> u8 {
        match self {
            Role::Admin => 0x00,
            Role::Member => 0x01,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct GroupIDs {
    pub group_ids: Vec<Member>,
}

impl From<&PgRow> for Member {
    fn from(row: &PgRow) -> Self {
        Member {
            id: row.get("id"),
            user_id: row.get("user_id"),
            group_id: row.get("group_id"),
            role_id: Role::try_from((row.get::<f32, &str>("role_id")) as u8).unwrap(),
            created: row.get("created"),
        }
    }
}

impl Member {
    //TODO check if admin and if group is empty
    pub async fn delete_all_by_user_id(db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM members WHERE user_id = $1")
            .bind(user_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query(
            "INSERT INTO member (user_id, group_id, role_id)
                    VALUES ($1, $2, $3) RETURNING *;",
        )
        .bind(self.user_id)
        .bind(self.group_id)
        .bind(self.role_id.to_int() as i32)
        .fetch_one(db)
        .await?
        .borrow()
        .into();
        Ok(())
    }
}
