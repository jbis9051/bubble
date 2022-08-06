use crate::extractor::AuthenticatedUser::AuthenticatedUser;
use crate::models::group::{Group, Role};
use crate::models::user::User;
use crate::types::DbPool;
use axum::extract::Path;
use axum::http::StatusCode;

use axum::routing::{delete, get, patch, post};
use axum::Extension;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/create", post(create))
        .route("/:id", get(read))
        .route("/:id/new_users", post(add_users))
        .route("/:id/delete_users", delete(delete_users))
        .route("/:id/name", patch(change_name))
        .route("/:id", delete(delete_group))
}

// Accept data -> deserialiable
// Return Data -> Serializable

//create and read functions
#[derive(Default, Deserialize, Serialize)]
pub struct GroupInfo {
    pub uuid: String,
    pub name: String,
    pub created: String,
}

#[derive(Deserialize, Serialize)]
pub struct GroupName {
    pub name: String,
    //token string
}
//Respond with JSON: id, name, created_date

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
        members: Vec::new(),
    };

    match group.create(&db.0, user.0.id).await {
        Ok(_) => (),
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let new_group = GroupInfo {
        uuid: group.uuid.to_string(),
        name: group.group_name,
        created: group.created.to_string(),
    };

    Ok((StatusCode::CREATED, Json(new_group)))
}

// respond with JSON: id, name, created_date
async fn read(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, Json<GroupInfo>), StatusCode> {
    let uuid_converted: Uuid = match Uuid::parse_str(&uuid) {
        Ok(uuid_converted) => uuid_converted,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    //UNSURE IF COLUMN INDEX OUT OF BOUNDS IS NECESSARY
    let group: Group = match Group::from_uuid(&db.0, uuid_converted).await {
        Ok(group) => group,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(error);
        }
    };
    let user_role: i32 = match group.role(&db.0, user.0.id).await {
        Ok(group) => group as i32,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(error);
        }
    };

    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        return Err(StatusCode::UNAUTHORIZED);
    }

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

//request JSON: vec<user_ids>
async fn add_users(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<UserID>,
    user: AuthenticatedUser,
) -> StatusCode {
    let uuid_converted: Uuid = match Uuid::parse_str(&uuid) {
        Ok(uuid_converted) => uuid_converted,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    let mut group = match Group::from_uuid(&db.0, uuid_converted).await {
        Ok(group) => group,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    let user_role: i32 = match group.role(&db.0, user.0.id).await {
        Ok(group) => group as i32,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        return StatusCode::UNAUTHORIZED;
    }
    for i in &payload.users {
        let user_id: Uuid = match Uuid::parse_str(i) {
            Ok(user_id) => user_id,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };
        let user = match User::from_uuid(&db.0, user_id).await {
            Ok(user) => user,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };
        match group.add_user(&db.0, user).await {
            Ok(_) => (),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };
    }
    StatusCode::OK
}

// //request JSON: vec<user_ids>
async fn delete_users(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<UserID>,
    user: AuthenticatedUser,
) -> StatusCode {
    let uuid_converted: Uuid = match Uuid::parse_str(&uuid) {
        Ok(uuid_converted) => uuid_converted,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    let mut group = match Group::from_uuid(&db.0, uuid_converted).await {
        Ok(group) => group,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    let user_role: i32 = match group.role(&db.0, user.0.id).await {
        Ok(group) => group as i32,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        return StatusCode::UNAUTHORIZED;
    }
    for i in &payload.users {
        let user_id: Uuid = match Uuid::parse_str(i) {
            Ok(user_id) => user_id,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };
        let user = match User::from_uuid(&db.0, user_id).await {
            Ok(user) => user,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };
        group.delete_user(&db.0, user);
    }
    StatusCode::OK
}

//
#[derive(Deserialize, Serialize)]
pub struct NameChange {
    name: String,
}

//
// //request json: name
async fn change_name(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<NameChange>,
    user: AuthenticatedUser,
) -> StatusCode {
    let group_id: Uuid = match Uuid::parse_str(&uuid) {
        Ok(group_id) => group_id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    let mut group = match Group::from_uuid(&db, group_id).await {
        Ok(group) => group,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    let user_role: i32 = match group.role(&db.0, user.0.id).await {
        Ok(group) => group as i32,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        return StatusCode::UNAUTHORIZED;
    }
    //must resolve where normal rust or json is how requests replies sent
    let name_to_change: &str = &payload.name;
    group.group_name = match name_to_change.parse() {
        Ok(name) => name,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    match group.update(&db).await {
        Ok(_) => (),
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    StatusCode::OK
}

// //none, just id passed from path
async fn delete_group(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    user: AuthenticatedUser,
) -> StatusCode {
    let group_id: Uuid = match Uuid::parse_str(&uuid) {
        Ok(group_id) => group_id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    let group = match Group::from_uuid(&db, group_id).await {
        Ok(group) => group,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    let user_role: i32 = match group.role(&db.0, user.0.id).await {
        Ok(group) => group as i32,
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        return StatusCode::UNAUTHORIZED;
    }

    match group.delete(&db).await {
        Ok(_) => (),
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return error;
        }
    };
    StatusCode::OK
}
