use crate::extractor::authenticated_user::AuthenticatedUser;
use crate::models::group::Group;
use crate::models::member::Role;
use crate::models::user::User;
use crate::types::DbPool;
use axum::extract::Path;
use axum::http::StatusCode;

use crate::models::member::Member;
use crate::routes::map_sqlx_err;
use axum::routing::{delete, get, patch, post};
use axum::Extension;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/:id", get(read))
        .route("/:id/users", post(add_users))
        .route("/:id/users", delete(delete_users))
        .route("/:id/name", patch(change_name))
        .route("/:id", delete(delete_group))
        .route("/:id/users", get(members))
}

#[derive(Default, Deserialize, Serialize)]
pub struct GroupInfo {
    pub uuid: String,
    pub name: String,
    pub created: String,
}

#[derive(Deserialize, Serialize)]
pub struct GroupName {
    pub name: String,
}

async fn create(
    db: Extension<DbPool>,
    Json(payload): Json<GroupName>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, Json<GroupInfo>), StatusCode> {
    let mut group: Group = Group {
        id: Default::default(),
        uuid: Uuid::new_v4(),
        group_name: payload.name,
        created: NaiveDateTime::from_timestamp(0, 0),
    };

    group.create(&db.0, &user.0).await.map_err(map_sqlx_err)?;

    let new_group = GroupInfo {
        uuid: group.uuid.to_string(),
        name: group.group_name,
        created: group.created.to_string(),
    };

    Ok((StatusCode::CREATED, Json(new_group)))
}

async fn read(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    _: AuthenticatedUser,
) -> Result<(StatusCode, Json<GroupInfo>), StatusCode> {
    let uuid_converted: Uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;

    let group: Group = Group::from_uuid(&db.0, &uuid_converted)
        .await
        .map_err(map_sqlx_err)?;

    let new_group = GroupInfo {
        uuid: group.uuid.to_string(),
        name: group.group_name,
        created: group.created.to_string(),
    };
    Ok((StatusCode::OK, Json(new_group)))
}

#[derive(Deserialize, Serialize)]
pub struct UserID {
    pub users: Vec<String>,
}

async fn add_users(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<UserID>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid_converted: Uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;

    let group = Group::from_uuid(&db.0, &uuid_converted)
        .await
        .map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

    if user_role != Role::Admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    for i in &payload.users {
        let user_id: Uuid = Uuid::parse_str(i).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let user = User::from_uuid(&db.0, &user_id)
            .await
            .map_err(map_sqlx_err)?;
        let mut member = Member {
            id: 0,
            user_id: user.id,
            group_id: group.id,
            role_id: Role::Member,
            created: NaiveDateTime::from_timestamp(0, 0),
        };
        member.create(&db.0).await.map_err(map_sqlx_err)?;
    }
    Ok(StatusCode::OK)
}

async fn members(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, Json<UserID>), StatusCode> {
    let uuid_converted: Uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;

    let group = Group::from_uuid(&db.0, &uuid_converted)
        .await
        .map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

    if user_role != Role::Admin {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let users_in_group = group.members(&db).await.map_err(map_sqlx_err)?;
    let mut user_uuids_in_group = UserID { users: vec![] };
    user_uuids_in_group.users = users_in_group
        .iter()
        .map(|user| user.uuid.to_string())
        .collect::<Vec<String>>();

    Ok((StatusCode::OK, Json(user_uuids_in_group)))
}

async fn delete_users(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<UserID>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut group = Group::from_uuid(&db.0, &uuid).await.map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

    if user_role != Role::Admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    for member_uuid in &payload.users {
        let user_id = Uuid::parse_str(member_uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
        let user = User::from_uuid(&db.0, &user_id)
            .await
            .map_err(map_sqlx_err)?;
        group.delete_user(&db.0, user).await.map_err(map_sqlx_err)?;
    }

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Serialize)]
pub struct NameChange {
    pub name: String,
}

async fn change_name(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<NameChange>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut group = Group::from_uuid(&db.0, &uuid).await.map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

    if user_role != Role::Admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let name_to_change: &str = &payload.name;
    group.group_name = name_to_change
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    group.update(&db).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

async fn delete_group(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;

    let group = Group::from_uuid(&db.0, &uuid).await.map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;
    if user_role != Role::Admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    group.delete(&db).await.map_err(map_sqlx_err)?;
    Ok(StatusCode::OK)
}
