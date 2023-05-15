use crate::helper::{start_server, TempDatabase};

use std::str::FromStr;

use axum::http::StatusCode;

use bubble::routes::user::{Clients, CreateUser, PublicClient};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use openmls::prelude::*;
use openmls_rust_crypto::OpenMlsRustCrypto;

use uuid::Uuid;

use crate::crypto_helper::{PRIVATE, PUBLIC};

use bubble::routes::client::{CreateClient, KeyPackagePublic, ReplaceKeyPackages};

use bubble::types::Base64;

mod crypto_helper;
mod helper;

#[tokio::test]
async fn test_client_crud() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);

    // Ensure there are no clients
    let res = client
        .get(&format!("/user/{}/clients", user.uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let payload: Clients = res.json().await;

    assert_eq!(payload.clients.len(), 0);

    // Create a Client
    let signature_scheme = SignatureScheme::ED25519;
    let backend = &OpenMlsRustCrypto::default();
    let (_signature_privkey, signature_pubkey) = SignatureKeypair::new(signature_scheme, backend)
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

    assert_eq!(res.status(), StatusCode::CREATED);

    let client_uuid = Uuid::from_str(&res.text().await).unwrap();

    // Ensure the client is created
    let res = client
        .get(&format!("/user/{}/clients", user.uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let payload: Clients = res.json().await;

    assert_eq!(payload.clients.len(), 1);
    assert_eq!(payload.clients[0].user_uuid, user.uuid.to_string());
    assert_eq!(payload.clients[0].uuid, client_uuid.to_string());
    assert_eq!(
        payload.clients[0].signing_key.0,
        signature_pubkey.as_slice()
    );
    assert_eq!(
        payload.clients[0].signature.0,
        &signature_of_signing_key.to_bytes()
    );

    // Check that the client can be retrieved

    let res = client
        .get(&format!("/client/{}", client_uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let payload: PublicClient = res.json().await;

    assert_eq!(payload.user_uuid, user.uuid.to_string());
    assert_eq!(payload.uuid, client_uuid.to_string());
    assert_eq!(payload.signing_key.0, signature_pubkey.as_slice());
    assert_eq!(payload.signature.0, &signature_of_signing_key.to_bytes());

    // Update the Client with a new signing key

    let (_signature_privkey, signature_pubkey) = SignatureKeypair::new(signature_scheme, backend)
        .unwrap()
        .into_tuple();

    let signature_of_signing_key = user_keypair.sign(signature_pubkey.as_slice());

    let create_client = CreateClient {
        signing_key: Base64(signature_pubkey.as_slice().to_vec()),
        signature: Base64(signature_of_signing_key.to_bytes().to_vec()),
    };

    let res = client
        .patch(&format!("/client/{}", client_uuid))
        .header("Authorization", bearer.clone())
        .json(&create_client)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    // Ensure the client is updated

    let res = client
        .get(&format!("/client/{}", client_uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let payload: PublicClient = res.json().await;

    assert_eq!(payload.user_uuid, user.uuid.to_string());
    assert_eq!(payload.uuid, client_uuid.to_string());
    assert_eq!(payload.signing_key.0, signature_pubkey.as_slice());
    assert_eq!(payload.signature.0, &signature_of_signing_key.to_bytes());

    // Delete the Client

    let res = client
        .delete(&format!("/client/{}", client_uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    // Ensure the client is deleted

    let res = client
        .get(&format!("/user/{}/clients", user.uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let payload: Clients = res.json().await;

    assert_eq!(payload.clients.len(), 0);

    // Ensure the client cannot be retrieved

    let res = client
        .get(&format!("/client/{}", client_uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_key_packages() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);

    // Create a Client
    let signature_scheme = SignatureScheme::ED25519;
    let backend = &OpenMlsRustCrypto::default();
    let signature_keypair = SignatureKeypair::new(signature_scheme, backend).unwrap();
    let (_signature_privkey, signature_pubkey) = signature_keypair.clone().into_tuple();

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

    assert_eq!(res.status(), StatusCode::CREATED);

    let client_uuid = Uuid::from_str(&res.text().await).unwrap();

    // Ensure there are no key packages

    let res = client
        .get(&format!("/client/{}/key_package", client_uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    // Upload Key Packages

    let identity = format!("keypackage_{}_{}", user.uuid, client_uuid);
    let credential_bundle = CredentialBundle::from_parts(identity.into_bytes(), signature_keypair);
    let ciphersuites = Ciphersuite::MLS_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519;

    let mut key_packages = Vec::new();

    for _ in 0..5 {
        let key_package_bundle =
            KeyPackageBundle::new(&[ciphersuites], &credential_bundle, backend, vec![]).unwrap();
        key_packages.push(Base64(
            key_package_bundle
                .key_package()
                .clone()
                .tls_serialize_detached()
                .unwrap(),
        ));
    }

    let payload = ReplaceKeyPackages { key_packages };

    let res = client
        .post(&format!("/client/{}/key_packages", client_uuid))
        .header("Authorization", bearer.clone())
        .json(&payload)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    // Get a Key Package

    let res = client
        .get(&format!("/client/{}/key_package", client_uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let _: KeyPackagePublic = res.json().await;
}
