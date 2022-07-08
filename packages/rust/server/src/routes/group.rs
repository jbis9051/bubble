use crate::models::group::Group;
use axum::extract;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Serialize, Deserialize};
use axum::routing::delete;

pub fn router() -> Router {
    let app = Router::new()
        .route("/group/:name", post(create))
        .route("/group/:id", get(read))
        .route("/group/:id/new_users", post(add_users))
        .route("/group/:id/users_to_delete", post(delete_users))
        .route("/group/:id/name", post(change_name))
        .route("/group/:id", delete(delete_group));
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

async fn create(Path(params): Path<String>) -> Json<GroupInfo> {
    let name: String = params.get("name").to_string();
    let group_response = Group::create(name);
    let new_group = GroupInfo {
        uuid: group_response.uuid,
        name: group_response.group_name,
        created: group_response.created,
    };
    Json(new_group)
}

// respond with JSON: id, name, created_date
async fn read(Path(params): Path<String>) -> Json<GroupInfo> {
    let uuid = params.get("id").to_string();
    let group_response = Group::read(uuid);
    let new_group = GroupInfo {
        uuid: group_response.uuid,
        name: group_response.group_name,
        created: group_response.created,
    };
    Json(new_group)
}

#[derive(Deserialize)]
pub struct UsersIDs {
    users: Vec<i32>,
}

//request JSON: vec<user_ids>
async fn add_users(Path(id): Path<String>, extract::Json(payload): extract::Json<UsersIDs>) {
    let group_id = id.get("id").to_string();
    let user_ids: &[i32] = &*payload.users;
    Group::add_users(group_id, user_ids);
}

//request JSON: vec<user_ids>
async fn delete_users(Path(params): Path<String>, extract::Json(payload): extract::Json<UsersIDs>) {
    let group_id = params.get("id").to_string();
    let users_to_delete = payload.users;
    Group::delete_users(group_id, users_to_delete);
}

#[derive(Serialize)]
pub struct NameChange {
    name: String,
}

//request json: name
async fn change_name(
    Path(params): Path<String>,
    extract::Json(payload): extract::Json<NameChange>,
) {
    let group_id = params.get("id").to_string();
    //must resolve where normal rust or json is how requests replies sent
    let name_to_change = payload.name;
    Group::change_name(group_id, name_to_change);
}

//none, just id passed from path
async fn delete_group(Path(params): Path<String>) {
    let group_id = params.get("id").to_string();
    Group::delete_group(group_id);
}
