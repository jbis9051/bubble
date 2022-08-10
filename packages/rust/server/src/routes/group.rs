use crate::extractor::AuthenticatedUser::AuthenticatedUser;
use crate::models::group::{Group, Role};
use crate::models::user::User;
use crate::types::DbPool;
use axum::extract::Path;
use axum::http::StatusCode;

use crate::routes::map_sqlx_err;
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
        .route("/:id/delete_users", post(delete_users))
        .route("/:id/name", patch(change_name))
        .route("/:id", delete(delete_group))
        .route("/:id/get_users", get(get_users))
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
        members: vec![],
    };

    group.create(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

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
    let uuid_converted: Uuid =
        Uuid::parse_str(&uuid).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    //UNSURE IF COLUMN INDEX OUT OF BOUNDS IS NECESSARY
    let group: Group = Group::from_uuid(&db.0, uuid_converted)
        .await
        .map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

    if user_role != Role::Admin {
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
) -> Result<StatusCode, StatusCode> {
    let uuid_converted: Uuid =
        Uuid::parse_str(&uuid).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut group = Group::from_uuid(&db.0, uuid_converted)
        .await
        .map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

    if user_role != Role::Admin {
        println!("User is not Admin of group");
        return Err(StatusCode::UNAUTHORIZED);
    }

    for i in &payload.users {
        let user_id: Uuid = Uuid::parse_str(i).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let user = User::from_uuid(&db.0, user_id)
            .await
            .map_err(map_sqlx_err)?;
        group.add_user(&db.0, user).await.map_err(map_sqlx_err)?;
    }
    Ok(StatusCode::OK)
}

async fn get_users(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, Json<UserID>), StatusCode> {
    let uuid_converted: Uuid =
        Uuid::parse_str(&uuid).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let group = Group::from_uuid(&db.0, uuid_converted)
        .await
        .map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

    if user_role != Role::Admin {
        println!("User is not Admin of group");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let users_in_group = UserID {
        users: group.get_users(&db).await.map_err(map_sqlx_err)?,
    };
    Ok((StatusCode::OK, Json(users_in_group)))
}

// //request JSON: vec<user_ids>
async fn delete_users(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<UserID>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut group = Group::from_uuid(&db.0, uuid).await.map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

    if user_role != Role::Admin {
        println!("User is not Admin of group");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // TODO Transaciton or something
    for member_uuid in &payload.users {
        let user_id = Uuid::parse_str(member_uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
        let user = User::from_uuid(&db.0, user_id)
            .await
            .map_err(map_sqlx_err)?;
        let user_role = group.role(&db, user.id).await.unwrap();
        if user_role == Role::Admin {
            println!("Please give another user the role of admin before leaving the group.");
            return Err(StatusCode::BAD_REQUEST);
        }
        group.delete_user(&db.0, user).await.map_err(map_sqlx_err)?;
    }

    Ok(StatusCode::OK)
}

//
#[derive(Deserialize, Serialize)]
pub struct NameChange {
    pub name: String,
}

//
// //request json: name
async fn change_name(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<NameChange>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut group = Group::from_uuid(&db.0, uuid).await.map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;

    if user_role != Role::Admin {
        println!("User is not Admin of group");
        return Err(StatusCode::UNAUTHORIZED);
    }
    //must resolve where normal rust or json is how requests replies sent
    let name_to_change: &str = &payload.name;
    group.group_name = name_to_change
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    group.update(&db).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

// //none, just id passed from path
async fn delete_group(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let group = Group::from_uuid(&db.0, uuid).await.map_err(map_sqlx_err)?;

    let user_role = group.role(&db.0, user.0.id).await.map_err(map_sqlx_err)?;
    if user_role != Role::Admin {
        println!("User is not Admin of group");
        return Err(StatusCode::UNAUTHORIZED);
    }

    group.delete(&db).await.map_err(map_sqlx_err)?;
    Ok(StatusCode::OK)
}
