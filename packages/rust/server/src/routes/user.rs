use axum::extract::Path;
use axum::routing::post;
use axum::{Extension, Json};
use std::collections::HashMap;

use crate::DbPool;
use axum::Router;
use serde::Deserialize;

use uuid::Uuid;

use crate::models::user::User;

pub fn router() -> Router {
    Router::new().route("/signup", post(signup))
    /*   .route("/signup-confirm", post(signup_confirm))
    .route("/signin", post(signin))
    .route("/signout/:token", post(signout))
    .route("/forgot", post(forgot))
    .route("/forgot-confirm", post(forgot_confirm))
    .route("/change_email", post(change_email))
    .route("", delete(delete_user))*/
}

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    username: String,
    password: String,
    phone: Option<String>,
    name: String,
}
async fn signup(db: Extension<DbPool>, Json(payload): Json<CreateUser>) {
    let user: User = User {
        id: 0,
        uuid: Uuid::new_v4().to_string(),
        username: payload.username,
        password: payload.password,
        profile_picture: None,
        email: payload.email,
        phone: payload.phone,
        name: payload.name,
        created: String::new(),
    };
    let _link_id = match User::signup(&db.0, &user).await {
        Ok(link_id) => link_id,
        _ => return,
    };

    println!("Sending Email to {}", user.email);
}

struct Confirm {
    link_id: String,
}
/*
async fn signup_confirm(
    conn: PoolConnection<Postgres>,
    Json(payload): Json<Confirm>,
) -> Result<User, io::Error> {
    let link_id = payload.link_id;

    Ok(())
}*/

async fn signin(Path(params): Path<HashMap<String, String>>) {
    let _email = params.get("email");
    let _password = params.get("password");

    todo!();
}
/*
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
*/
