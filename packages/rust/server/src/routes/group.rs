use crate::models::group::Group;
use crate::types::DbPool;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::Extension;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/:name", post(create))
        .route("/info/:id", get(read))
    // .route("/group/:id/new_users", post(add_users))
    // .route("/group/:id/delete_users", post(delete_users))
    // .route("/group/:id/name", post(change_name))
    // .route("/group/:id", delete(delete_group))
}

// Accept data -> deserialiable
// Return Data -> Serializable

//create and read functions
#[derive(Serialize)]
pub struct GroupInfo {
    uuid: String,
    name: String,
    created: String,
}

//Respond with JSON: id, name, created_date

async fn create(db: Extension<DbPool>, Path(name): Path<String>) -> Json<GroupInfo> {
    let mut group: Group = Group {
        id: 0,
        uuid: Uuid::new_v4(),
        group_name: name,
        created: NaiveDateTime::from_timestamp(0, 0),
        members: vec![],
    };

    Group::create(&db.0, &mut group).await.unwrap();
    let new_group = GroupInfo {
        uuid: group.uuid.to_string(),
        name: group.group_name,
        created: group.created.to_string(),
    };
    Json(new_group)
}

// respond with JSON: id, name, created_date
async fn read(db: Extension<DbPool>, Path(uuid): Path<String>) -> Json<GroupInfo> {
    // let uuid: = params.get("uuid");
    let mut group: Group = Group {
        id: 0,
        uuid: Uuid::new_v4(),
        group_name: String::new(),
        created: NaiveDateTime::from_timestamp(0, 0),
        members: vec![],
    };
    Group::read(&db.0, &mut group, uuid);
    let new_group = GroupInfo {
        uuid: group.uuid.to_string(),
        name: group.group_name,
        created: group.created.to_string(),
    };
    Json(new_group)
}

#[derive(Deserialize, Serialize)]
pub struct UsersIDs {
    users: Vec<i32>,
}

//request JSON: vec<user_ids>
// async fn add_users(db: Extension<DbPool>, Path(uuid): Path<String>, Json(payload): Json<UsersIDs>) {
//     let user_ids: &[i32] = &*payload.users;
//     Group::add_users(&db.0, uuid, user_ids);
// }

// //request JSON: vec<user_ids>
// async fn delete_users(
//     db: Extension<DbPool>,
//     Path(params): Path<String>,
//     extract::Json(payload): extract::Json<UsersIDs>,
// ) {
//     let group_id = params.get("uuid").to_string();
//     let users_to_delete = payload.users;
//     Group::delete_users(&db.0, group_id, users_to_delete);
// }
//
// #[derive(Serialize)]
// pub struct NameChange {
//     name: String,
// }
//
// //request json: name
// async fn change_name(
//     db: Extension<DbPool>,
//     Path(params): Path<String>,
//     extract::Json(payload): extract::Json<NameChange>,
// ) {
//     let group_id = params.get("uuid").to_string();
//     //must resolve where normal rust or json is how requests replies sent
//     let name_to_change = payload.name;
//     Group::change_name(&db.0, group_id, name_to_change);
// }
//
// //none, just id passed from path
// async fn delete_group(db: Extension<DbPool>, Path(params): Path<String>) {
//     let group_id = params.get("uuid").to_string();
//     Group::delete_group(&db.0, group_id);
// }
