use crate::extractor::AuthenticatedUser::AuthenticatedUser;
use crate::models::group::{Group, Role};
use crate::models::user::User;
use crate::types::DbPool;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::delete;
use axum::routing::{get, post};
use axum::Extension;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use std::process;

pub fn router() -> Router {
    Router::new()
        .route("/create", post(create))
        .route("/:id", get(read))
        .route("/:id/new_users", post(add_users))
        .route("/:id/delete_users", post(delete_users))
        .route("/:id/name", post(change_name))
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
) -> (StatusCode, Json<GroupInfo>) {
    let mut group: Group = Group {
        id: Default::default(),
        uuid: Uuid::new_v4(),
        group_name: payload.name,
        created: NaiveDateTime::from_timestamp(0, 0),
        members: Vec::new(),
    };
    //authorization
    group.create(&db.0, user.0.id).await.unwrap();

    let new_group = GroupInfo {
        uuid: group.uuid.to_string(),
        name: group.group_name,
        created: group.created.to_string(),
    };

    (StatusCode::CREATED, Json(new_group))
}

// respond with JSON: id, name, created_date
async fn read(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    user: AuthenticatedUser,
) -> Json<GroupInfo> {
    let uuid_converted: Uuid = Uuid::parse_str(&uuid).unwrap();
    let group: Group = Group::from_uuid(&db.0, uuid_converted).await.unwrap();
    let user_role = group.get_role(&db.0, user.0.id).await.unwrap();
    //might change with a error code returned instead
    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        process::exit(1)
    }
    //Group::read(&db.0, &mut group, uuid_converted, &user).await.unwrap();

    let new_group = GroupInfo {
        uuid: group.uuid.to_string(),
        name: group.group_name,
        created: group.created.to_string(),
    };
    Json(new_group)
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
) {
    let uuid_converted: Uuid = Uuid::parse_str(&uuid).unwrap();
    let mut group = Group::from_uuid(&db.0, uuid_converted).await.unwrap();
    let user_role = group.get_role(&db.0, user.0.id).await.unwrap();
    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        process::exit(1)
    }
    for i in &payload.users {
        let user_id: Uuid = Uuid::parse_str(i).unwrap();
        let user = User::get_by_uuid(&db.0, user_id).await.unwrap();
        group.add_user(&db.0, user);
    }
}

// //request JSON: vec<user_ids>
async fn delete_users(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<UserID>,
    user: AuthenticatedUser,
) {
    let uuid_converted: Uuid = Uuid::parse_str(&uuid).unwrap();
    let mut group = Group::from_uuid(&db.0, uuid_converted).await.unwrap();
    let user_role = group.get_role(&db.0, user.0.id).await.unwrap();
    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        process::exit(1)
    }
    for i in &payload.users {
        let user_id: Uuid = Uuid::parse_str(i).unwrap();
        let user = User::get_by_uuid(&db.0, user_id).await.unwrap();
        group.delete_user(&db.0, user);
    }
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
) {
    let group_id: Uuid = Uuid::parse_str(&uuid).unwrap();
    let mut group = Group::from_uuid(&db, group_id).await.unwrap();
    let user_role = group.get_role(&db.0, user.0.id).await.unwrap();
    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        process::exit(1)
    }
    //must resolve where normal rust or json is how requests replies sent
    let name_to_change: &str = &payload.name;
    group.group_name = name_to_change.parse().unwrap();
    group.update(&db).await.unwrap();
}

// //none, just id passed from path
async fn delete_group(db: Extension<DbPool>, Path(uuid): Path<String>, user: AuthenticatedUser) {
    let group_id: Uuid = Uuid::parse_str(&uuid).unwrap();
    let group = Group::from_uuid(&db, group_id).await.unwrap();
    let user_role = group.get_role(&db.0, user.0.id).await.unwrap();
    if user_role != Role::Admin as i32 {
        println!("User is not Admin of group");
        process::exit(1)
    }

    group.delete(&db).await.unwrap();
}
