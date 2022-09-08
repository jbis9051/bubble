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
        let row_id_in: i32 = row.get("role_id");
        Member {
            id: row.get("id"),
            user_id: row.get("user_id"),
            group_id: row.get("group_id"),
            role_id: Role::try_from(row_id_in as u8).unwrap(),
            created: row.get("created"),
        }
    }
}

impl Member {
    //should return instead groups vec,
    //one just gets group_ids and returns vector
    //another gets just admin size
    //members size

    pub async fn group_id(db: &DbPool, user_id: i32) -> Result<Vec<PgRow>, sqlx::Error> {
        let groups = sqlx::query("SELECT group_id FROM member WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(db)
            .await?;
        Ok(groups)
    }

    pub async fn admin_num(db: &DbPool, group_id: i32) -> Result<Vec<PgRow>, sqlx::Error> {
        let admin_size =
            sqlx::query("SELECT user_id FROM member WHERE group_id = $1 AND role_id = $2")
                .bind(group_id)
                .bind(Role::Admin as i32)
                .fetch_all(db)
                .await?;
        Ok(admin_size)
    }

    pub async fn all_members_in_group(
        db: &DbPool,
        group_id: i32,
    ) -> Result<Vec<PgRow>, sqlx::Error> {
        let members_size = sqlx::query("SELECT * FROM member WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(db)
            .await?;
        Ok(members_size)
    }

    pub async fn delete_all_by_user_id(db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM member WHERE user_id = $1")
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
