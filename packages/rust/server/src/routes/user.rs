use std::collections::HashMap;
use std::io;
use axum::{Json};
use axum::extract::{Path};
use axum::routing::{post, delete};

use axum::Router;
use serde::de::Unexpected::Str;
use serde::Deserialize;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;

use crate::models::user;
use crate::models::user::User;


pub fn router() {
    let user_routes = Router::new()
        .route("/user/signup", post(signup))
        .route("/user/signup-confirm", post(signup_confirm))
        .route("/user/signin/:email/:password", post(signin))
        .route("/user/signout/:token", post(signout))
        .route("/user/forgot/:email", post(forgot))
        .route("/user/forgot-confirm/:email/:password", post(forgot_confirm))
        .route("/user/change_email/:email", post(change_email))
        .route("/user/:password", delete(delete_user));
}

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    username: String,
    password: String,
    phone: Option<String>,
    name: String,
}
async fn signup(conn: PoolConnection<Postgres>, Json(payload): Json<CreateUser>) -> Result<(), io::Error> {

    let user: User = User {
        id: 0,
        uuid: String::new(),
        username: payload.username,
        password: payload.password,
        profile_picture: String::new(),
        email: payload.email,
        phone: payload.phone,
        name: payload.name,
        created: String::new(),
    };
    User::signup(conn, &user);

    println!("Sending Email to {}", user.email);
    Ok(())
}

struct Confirm {
    link_id: String,
}
async fn signup_confirm(Json(payload): Json<Confirm>) -> Result<String, Error> {
    let link_id = payload.link_id;


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

async fn delete_user(Path(params): &str) {
    let password = params.get("password");

    todo!();
}