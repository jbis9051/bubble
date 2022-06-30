use axum::extract::Path;
use axum::routing::get;
use axum::handler::Handler;
use axum::Router;

pub fn router() -> Router {
    let app = Router::new()
        .route("/group/:name", get(create))
        //have to make unique
        .route("/group/:id", get(read))
        .route("/group/:id", get(read_users))
        .route("/group/:id/:new_users", get(add_users))
        .route("/group/:id/:users_to_delete", get(delete_users))
        .route("/group/:id/:name", get(change_name))
        .route("/group/:id", get(delete));
}

async fn create(Path(params): &str){
    let name = params.get("name");
    todo!();
}

async fn read(Path(params): &str){
    let group_id = params.get("id");
    todo!();
}

// unsure about multiple inputs
async fn read_users(Path(params): &str){
    let group_id = params.get("id");
    todo!();
}

async fn add_users(Path(params): &str){
    let group_id = params.get("id");
    let new_users: Vec<&str> = params.get(add_users);
    todo!();
}

async fn delete_users(Path(params): &str){
    let group_id = params.get("id");
    let users_to_delete: Vec<&str> = params.get(users_to_delete);
    todo!();
}

async fn change_name(Path(params): &str){
    let group_id = params.get("id");
    let name = params.get(name);
    todo!();
}

async fn delete(Path(params): &str){
    let group_id = params.get("id");
    todo!();
}