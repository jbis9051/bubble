use std::future::Future;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::handler::Handler;
use axum::{Json, Router};

pub fn router() -> Router {
    let app = Router::new()
        .route("/group/:name", post(create))
        .route("/group/:id", get(read))
        .route("/group/:id/:new_users", post(add_users))
        .route("/group/:id/:users_to_delete", post(delete_users))
        .route("/group/:id/:name", post(change_name))
        .route("/group/:id", delete(delete_group));
}


async fn create(Path(params): Path<String>){
    let name: String = params.get("name");

    // Call create in models
    // Return reply JSON
    todo!();
}

async fn read(Path(params): Path<String>){
    let group_id = params.get("id");
    todo!();
}

// unsure about multiple inputs
async fn read_users(Path(params): Path<String>){
    let group_id = params.get("id");
    todo!();
}

async fn add_users(Path((id, new_user_ids)): Path<(String, Vec<String>)>){
    let group_id = id.get("id");
    let new_users = new_user_ids.get("new_users");
    todo!();
}

async fn delete_users(Path((id, user_ids_to_delete)): Path<(String, Vec<String>)>){
    let group_id = id.get("id");
    let users_to_delete = user_ids_to_delete.get("users_to_delete");
    todo!();
}

async fn change_name(Path((id, new_name)): Path<(String, Vec<String>)>, Json(payload): Json<String>){
    let group_id = id.get("id");
    let name = new_name.get("name");
    //must resolve where normal rust or json is how requests replies sent
    let name_to_change = payload;
    todo!();
}

async fn delete_group(Path(params): fn(Path<String>)){
    let group_id = params.get("id");
    todo!();
}