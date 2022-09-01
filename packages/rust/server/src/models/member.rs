use crate::types::DbPool;
use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;

use crate::models::group::Group;
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
    pub async fn delete_all_by_user_id(db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
        let groups = sqlx::query("SELECT group_id FROM member WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(db)
            .await?;

        //checking for edge cases
        let members_size_len = groups.len();
        for i in groups {
            let group_id: i32 = i.get("group_id");

            //cannot delete group if user to delete is only admin
            let admin_size =
                sqlx::query("SELECT user_id FROM member WHERE group_id = $1 AND role_id = $2")
                    .bind(group_id)
                    .bind(Role::Admin as i32)
                    .fetch_all(db)
                    .await?;
            if admin_size.len() == 1 {
                let admin_id: i32 = admin_size[0].get("user_id");
                if admin_id == user_id && members_size_len != 1 {
                    //TODO Preferable to return an error from routes, not models, that is not sqlx and includes the id of the groups in quesetion
                    return Err(sqlx::Error::Protocol(
                        "There must be at least one Admin in the group.".to_string(),
                    ));
                }
            }

            //deletes group if user is the only user in the group
            let members_size = sqlx::query("SELECT * FROM member WHERE group_id = $1")
                .bind(group_id)
                .fetch_all(db)
                .await?;
            if members_size.len() == 1 {
                let group = Group::from_id(db, group_id).await?;
                group.delete(db).await?;
            }
        }

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
