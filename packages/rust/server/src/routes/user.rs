use axum::extract::Path;
use axum::routing::get;
use axum::routing::post;
use axum::handler::Handler;
use axum::Router;

pub fn router() -> Router {
    let app = Router::new()
        .route("/user/signup/:email/:username/:password/:phone/:name", post(signup))
        .route("/user/signin/:email/:password", get(signin))
        .route("/user/signout/:token", get(signout))
        .route("/user/forgot/:email", get(forgot))
        .route("/user/forgot-confirm/:email/:password", get(forgot_confirm))
        .route("/user/change_email/:email", get(change_email))
        .route("/user/:password", get(delete));
}

async fn signup(Path(params): &str) {
    let email = params.get("email");
    let username = params.get("username");
    let password = params.get("password");
    let phone = params.get("phone");
    let name = params.get("name");



    todo!();
}

async fn signin(Path(params): &str) {
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