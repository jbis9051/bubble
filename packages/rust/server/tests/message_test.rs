use crate::helper::{start_server, TempDatabase};

use axum::http::StatusCode;

use bubble::routes::message::{CheckMessages, MessageRequest, MessagesReturned};

use uuid::Uuid;

use std::str::FromStr;

use bubble::routes::user::{Clients, CreateUser};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use openmls::prelude::*;
use openmls_rust_crypto::OpenMlsRustCrypto;

use bubble::routes::client::CreateClient;

use crate::crypto_helper::{PRIVATE, PUBLIC};
use base64::{engine::general_purpose, Engine as _};
use bubble::types::{Base64, SIGNATURE_SCHEME};

mod crypto_helper;
mod helper;

#[tokio::test]
async fn test_message() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "test_username".to_string(),
        password: "test_password".to_string(),
        name: "test_name".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);

    let res = client
        .get(&format!("/user/{}/clients", user.uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let payload: Clients = res.json().await;

    assert_eq!(payload.clients.len(), 0);

    // Create a Client
    let backend = &OpenMlsRustCrypto::default();
    let (_signature_privkey, signature_pubkey) = SignatureKeypair::new(SIGNATURE_SCHEME, backend)
        .unwrap()
        .into_tuple();

    let user_keypair = Keypair {
        public: PublicKey::from_bytes(PUBLIC).unwrap(),
        secret: SecretKey::from_bytes(PRIVATE).unwrap(),
    };

    let signature_of_signing_key = user_keypair.sign(signature_pubkey.as_slice());

    let create_client = CreateClient {
        signing_key: Base64(signature_pubkey.as_slice().to_vec()),
        signature: Base64(signature_of_signing_key.to_bytes().to_vec()),
    };

    let res = client
        .post("/client")
        .header("Authorization", bearer.clone())
        .json(&create_client)
        .send()
        .await;

    let client_uuid = Uuid::from_str(&res.text().await).unwrap();

    let request_messages = CheckMessages {
        client_uuid: client_uuid.to_string(),
    };
    let res = client
        .get("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_messages).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.json::<MessagesReturned>().await.messages.len(), 0);

    let message = MessageRequest {
        client_uuids: vec![client_uuid.to_string()],
        message: Base64(
            general_purpose::STANDARD
                .encode("test message")
                .into_bytes(),
        ),
    };

    let res = client
        .post("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .get("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_messages).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    let ret = res.json::<MessagesReturned>().await.messages;
    assert_eq!(ret.len(), 1);
    let decoded_message = general_purpose::STANDARD.decode(message.message.0);
    let decoded_message = match decoded_message {
        Ok(message) => message,
        Err(_) => panic!("failed to decode message"),
    };
    assert_eq!("test message".as_bytes().to_vec(), decoded_message);
}
#[tokio::test]
async fn negative_test_message() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;
    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "test_username".to_string(),
        password: "test_password".to_string(),
        name: "test_name".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);
    let res = client
        .get(&format!("/user/{}/clients", user.uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    let _payload: Clients = res.json().await;

    // Create a Client
    let backend = &OpenMlsRustCrypto::default();
    let (_signature_privkey, signature_pubkey) = SignatureKeypair::new(SIGNATURE_SCHEME, backend)
        .unwrap()
        .into_tuple();

    let user_keypair = Keypair {
        public: PublicKey::from_bytes(PUBLIC).unwrap(),
        secret: SecretKey::from_bytes(PRIVATE).unwrap(),
    };

    let signature_of_signing_key = user_keypair.sign(signature_pubkey.as_slice());

    let create_client = CreateClient {
        signing_key: Base64(signature_pubkey.as_slice().to_vec()),
        signature: Base64(signature_of_signing_key.to_bytes().to_vec()),
    };

    let res = client
        .post("/client")
        .header("Authorization", bearer.clone())
        .json(&create_client)
        .send()
        .await;
    let _client_uuid = Uuid::from_str(&res.text().await).unwrap();

    // //not a Uuid
    let message = MessageRequest {
        client_uuids: vec![69.to_string()],
        message: Base64("test message".as_bytes().to_vec()),
    };

    let res = client
        .post("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    //not an existing Uuid
    let message = MessageRequest {
        client_uuids: vec![Uuid::new_v4().to_string()],
        message: Base64(
            general_purpose::STANDARD
                .encode("test message")
                .into_bytes(),
        ),
    };

    let res = client
        .post("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    //the client does not exist

    let request_messages = CheckMessages {
        client_uuid: Uuid::new_v4().to_string(),
    };

    let res = client
        .get("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_messages).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    //the client belongs to a different user
    //make a second client
    let created_user2 = CreateUser {
        email: "test2@gmail.com".to_string(),
        username: "test_username2".to_string(),
        password: "test_password2".to_string(),
        name: "test_name2".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };

    let (token2, _user2) = helper::initialize_user(db.pool(), &client, &created_user2)
        .await
        .unwrap();
    let bearer2 = format!("Bearer {}", token2);

    let backend2 = &OpenMlsRustCrypto::default();
    let (_signature_privkey2, signature_pubkey2) =
        SignatureKeypair::new(SIGNATURE_SCHEME, backend2)
            .unwrap()
            .into_tuple();

    let user_keypair2 = Keypair {
        public: PublicKey::from_bytes(PUBLIC).unwrap(),
        secret: SecretKey::from_bytes(PRIVATE).unwrap(),
    };

    let signature_of_signing_key2 = user_keypair2.sign(signature_pubkey2.as_slice());

    let create_client2 = CreateClient {
        signing_key: Base64(signature_pubkey2.as_slice().to_vec()),
        signature: Base64(signature_of_signing_key2.to_bytes().to_vec()),
    };
    let res2 = client
        .post("/client")
        .header("Authorization", bearer2.clone())
        .json(&create_client2)
        .send()
        .await;

    let client_uuid2 = Uuid::from_str(&res2.text().await).unwrap();
    let request_messages2 = CheckMessages {
        client_uuid: client_uuid2.to_string(),
    };

    //first user's token is used here
    let res = client
        .get("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_messages2).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}
