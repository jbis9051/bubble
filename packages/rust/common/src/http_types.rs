use crate::base64::Base64;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateClient {
    pub signing_key: Base64,
    pub signature: Base64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateClient {
    pub signing_key: Base64,
    pub signature: Base64,
}

#[derive(Serialize, Deserialize)]
pub struct ReplaceKeyPackages {
    pub key_packages: Vec<Base64>,
}

#[derive(Serialize, Deserialize)]
pub struct KeyPackagePublic {
    pub key_package: Base64,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessage {
    pub client_uuids: Vec<String>,
    pub message: Base64,
}

#[derive(Serialize, Deserialize)]
pub struct CheckMessages {
    pub client_uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessagesResponse {
    pub messages: Vec<Base64>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub name: String,
    pub identity: Base64,
}

#[derive(Serialize, Deserialize)]
pub struct ConfirmEmail {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionTokenResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ForgotEmail {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordReset {
    pub password: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordResetCheck {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeEmail {
    pub new_email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteUser {
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateIdentity {
    pub identity: Base64,
}

#[derive(Serialize, Deserialize)]
pub struct PublicUser {
    pub uuid: String,
    pub username: String,
    pub name: String,
    pub identity: Base64,
}

#[derive(Serialize, Deserialize)]
pub struct PublicClient {
    pub user_uuid: String,
    pub uuid: String,
    pub signing_key: Base64,
    pub signature: Base64,
}

#[derive(Serialize, Deserialize)]
pub struct ClientsResponse {
    pub clients: Vec<PublicClient>,
}

#[derive(Serialize, Deserialize)]
pub struct RegisteredClientsResponse {
    pub uuid: String,
}
