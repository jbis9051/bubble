use std::collections::HashMap;
use axum::extract::Path;
use axum::routing::{get, post, delete};
use axum::handler::Handler;
use axum::Router;

pub fn router() {
    let user_routes = Router::new()
        .route("/user/signup", post(signup))
        .route("/user/signin/:email/:password", post(signin))
        .route("/user/signout/:token", post(signout))
        .route("/user/forgot/:email", post(forgot))
        .route("/user/forgot-confirm/:email/:password", post(forgot_confirm))
        .route("/user/change_email/:email", post(change_email))
        .route("/user/:password", delete(delete));
}

async fn signup(Path(params): Path<&str>) {
    let email = params.get("email");
    let username = params.get("username");
    let password = params.get("password");
    let phone = params.get("phone");
    let name = params.get("name");



    todo!();
}

async fn signin(Path(params): Path<HashMap<String, String>>) {
    let email = params.get("email");
    let password = params.get("password");


    todo!();
}

async fn signout(Path(params): &str) {
    let token = params.get("token");

    todo!();
}

async fn forgot(Path(params): &str) {
    let email = params.get("email");

    todo!();
}

async fn forgot_confirm(Path(params): &str) {
    let email = params.get("email");
    let password = params.get("password");

    todo!();
}

async fn change_email(Path(params): &str) {
    let new_email = params.get("email");

    todo!();
}

async fn delete(Path(params): &str) {
    let password = params.get("password");

    todo!();
}