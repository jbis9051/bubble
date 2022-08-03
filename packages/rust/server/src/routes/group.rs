use crate::extractor::AuthenticatedUser::AuthenticatedUser;
use crate::models::group::Group;
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
#[derive(Deserialize, Serialize)]
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
        id: 1,
        uuid: Uuid::new_v4(),
        group_name: payload.name,
        created: NaiveDateTime::from_timestamp(0, 0),
        members: vec![],
    };

    Group::create(&db.0, &mut group, &user).await.unwrap();
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
    // let uuid: = params.get("uuid");
    let uuid_converted: Uuid = Uuid::parse_str(&uuid).unwrap();
    let mut group: Group = Group {
        id: 0,
        uuid: Uuid::new_v4(),
        group_name: String::new(),
        created: NaiveDateTime::from_timestamp(0, 0),
        members: vec![],
    };
    Group::read(&db.0, &mut group, uuid_converted, &user);
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
// #[derive(Deserialize, Serialize)]
// pub struct users_id {
//     pub users: Vec<Uuid>,
//}
//Hi, so the issue is t ihe uuid now, where on line 96, we gotta make the data type uuid

//wait its easier to just convert string to uuid
//request JSON: vec<user_ids>
async fn add_users(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<UserID>,
    user: AuthenticatedUser,
) {
    let uuid_converted: Uuid = Uuid::parse_str(&uuid).unwrap();
    for i in &payload.users {
        let user_id: Uuid = Uuid::parse_str(i).unwrap();
        Group::add_user(&db.0, uuid_converted, user_id, &user);
    }
}

// //request JSON: vec<user_ids>
async fn delete_users(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<UserID>,
    user: AuthenticatedUser,
) {
    let group_id: Uuid = Uuid::parse_str(&uuid).unwrap();
    for i in &payload.users {
        let user_id: Uuid = Uuid::parse_str(i).unwrap();
        Group::delete_user(&db.0, group_id, user_id, &user);
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
    //must resolve where normal rust or json is how requests replies sent
    let name_to_change: &str = &payload.name;
    Group::change_name(&db.0, group_id, name_to_change, &user);
}

// //none, just id passed from path
async fn delete_group(db: Extension<DbPool>, Path(uuid): Path<String>, user: AuthenticatedUser) {
    let group_id: Uuid = Uuid::parse_str(&uuid).unwrap();
    Group::delete_group(&db.0, group_id, &user);
}
