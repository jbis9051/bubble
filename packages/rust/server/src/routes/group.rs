use std::future::Future;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::handler::Handler;
use axum::extract;
use axum::{Json, Router};
use crate::models::group;
use crate::models::group::Group;


pub fn router() -> Router {
    let app = Router::new()
        .route("/group/:name", post(create))
        .route("/group/:id", get(read))
        .route("/group/:id/:new_users", post(add_users))
        .route("/group/:id/:users_to_delete", post(delete_users))
        .route("/group/:id/:name", post(change_name))
        .route("/group/:id", delete(delete_group));
}

// Accept data -> deserialiable
// Return Data -> Serializable

//create and read functions
#[derive(Serialize)]
pub struct GroupInfo {
    id: i32,
    name: String,
    created: String,
}

//Respond with JSON: id, name, created_date

async fn create(Path(params): Path<String>) -> Json<GroupInfo> {
    let name = params.get("name");
    let group_response = Group.create(name);
    let new_group = GroupInfo {
        id: group_response.id,
        name: group_response.group_name,
        created: group_response.created,
    };
    Json(new_group)
}

// respond with JSON: id, name, created_date
async fn read(Path(params): Path<i32>) -> Json<GroupInfo> {
    let id = params.get("id");
    let group_response = Group.read(id);
    let new_group = GroupInfo {
        id: group_response.id,
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
async fn add_users(Path(id): Path<i32>, extract::Json(payload): extract::Json<UsersIDs>) {
    let group_id = id.get("id");
    let user_ids: Vec<i32> = payload.users;
    Group.add_users(group_id, user_ids);
}

//request JSON: vec<user_ids>
async fn delete_users(Path(id): Path<i32>, extract::Json(payload): extract::Json<UsersIDs>) {
    let group_id = id.get("id");
    let users_to_delete = payload.users;
    todo!();
}

#[derive(Serialize)]
pub struct NameChange {
    name: String,
}

//request json: name
async fn change_name(Path(id): Path<i32>, Json(payload): Json<String>) -> Json<NameChange> {
    let group_id = id.get("id");
    //must resolve where normal rust or json is how requests replies sent
    let name_to_change = payload.name;
    todo!();
}


//none, just id passed from path
async fn delete_group(Path(params): fn(Path<String>)) {
    let group_id = params.get("id");
    todo!();
}