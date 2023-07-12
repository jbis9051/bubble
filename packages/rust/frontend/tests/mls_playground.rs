use openmls::prelude::{config::CryptoConfig, *};
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls_traits::{signatures::Signer, types::SignatureScheme};

fn generate_credential(
    identity: Vec<u8>,
    credential_type: CredentialType,
    signature_algorithm: SignatureScheme,
    backend: &OpenMlsRustCrypto,
) -> (CredentialWithKey, SignatureKeyPair) {
    // ANCHOR: create_basic_credential
    let credential = Credential::new(identity, credential_type).unwrap();
    // ANCHOR_END: create_basic_credential
    // ANCHOR: create_credential_keys
    let signature_keys = SignatureKeyPair::new(signature_algorithm).unwrap();
    signature_keys.store(backend.key_store()).unwrap();
    // ANCHOR_END: create_credential_keys

    (
        CredentialWithKey {
            credential,
            signature_key: signature_keys.to_public_vec().into(),
        },
        signature_keys,
    )
}

fn generate_key_package(
    ciphersuite: Ciphersuite,
    credential_with_key: CredentialWithKey,
    _extensions: Extensions,
    backend: &OpenMlsRustCrypto,
    signer: &impl Signer,
) -> KeyPackage {
    // ANCHOR: create_key_package
    // Create the key package
    KeyPackage::builder()
        // .key_package_extensions(extensions)
        .build(
            CryptoConfig::with_default_version(ciphersuite),
            backend,
            signer,
            credential_with_key,
        )
        .unwrap()
    // ANCHOR_END: create_key_package
}

#[tokio::test]
pub async fn mls_playground() {
    //use simple_logger::SimpleLogger;
    //SimpleLogger::new().init().unwrap();

    let backend = &OpenMlsRustCrypto::default();
    let ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519;
    // Generate credentials with keys
    let (alice_credential, alice_signature_keys) = generate_credential(
        "Alice".into(),
        CredentialType::Basic,
        ciphersuite.signature_algorithm(),
        backend,
    );

    let (bob_credential, bob_signature_keys) = generate_credential(
        "Bob".into(),
        CredentialType::Basic,
        ciphersuite.signature_algorithm(),
        backend,
    );

    // Generate KeyPackages
    let bob_key_package = generate_key_package(
        ciphersuite,
        bob_credential,
        Extensions::default(),
        backend,
        &bob_signature_keys,
    );

    // ANCHOR: mls_group_config_example
    let mls_group_config = MlsGroupConfig::builder()
        .crypto_config(CryptoConfig::with_default_version(ciphersuite))
        .use_ratchet_tree_extension(true)
        .build();
    // ANCHOR_END: mls_group_config_example

    // ANCHOR: alice_create_group
    let group_id = GroupId::from_slice(b"123e4567e89b");

    let mut alice_group = MlsGroup::new_with_group_id(
        backend,
        &alice_signature_keys,
        &mls_group_config,
        group_id,
        alice_credential,
    )
    .expect("An unexpected error occurred.");

    // === Alice adds Bob ===
    // ANCHOR: alice_adds_bob
    let (mls_message_out, welcome, group_info) = alice_group
        .add_members(backend, &alice_signature_keys, &[bob_key_package])
        .expect("Could not add members.");
    // ANCHOR_END: alice_adds_bob

    // Suppress warning
    let _mls_message_out = mls_message_out;
    let _group_info = group_info;

    alice_group
        .merge_pending_commit(backend)
        .expect("error merging pending commit");

    // Check that the group now has two members
    assert_eq!(alice_group.members().count(), 2);

    // Check that Alice & Bob are the members of the group
    let members = alice_group.members().collect::<Vec<Member>>();
    assert_eq!(members[0].credential.identity(), b"Alice");
    assert_eq!(members[1].credential.identity(), b"Bob");

    // ANCHOR: bob_joins_with_welcome
    let ser = welcome.tls_serialize_detached().unwrap();
    let welcome: Welcome = match MlsMessageIn::tls_deserialize_exact(ser).unwrap().extract() {
        MlsMessageInBody::Welcome(w) => w,
        _ => panic!("fuck"),
    };
    let mut bob_group = MlsGroup::new_from_welcome(
        backend,
        &mls_group_config,
        welcome,
        None, // We use the ratchet tree extension, so we don't provide a ratchet tree here
    )
    .expect("Error joining group from Welcome");
    // ANCHOR_END: bob_joins_with_welcome

    let message = alice_group
        .create_message(backend, &alice_signature_keys, b"Hi")
        .unwrap();

    // process bob
    let message = MlsMessageIn::tls_deserialize_exact(message.tls_serialize_detached().unwrap())
        .unwrap()
        .extract();
    let message = match message {
        MlsMessageInBody::PrivateMessage(a) => a,
        _ => panic!("uck"),
    };

    let out = alice_group
        .process_message(backend, message)
        .expect("TODO: panic message");
    return;
    // send bob
    let message = bob_group
        .create_message(backend, &bob_signature_keys, b"Bye")
        .unwrap();

    // process alice
    let message = MlsMessageIn::tls_deserialize_exact(message.tls_serialize_detached().unwrap())
        .unwrap()
        .extract();
    let message = match message {
        MlsMessageInBody::PrivateMessage(a) => a,
        _ => panic!("uck"),
    };

    let out = alice_group
        .process_message(backend, message)
        .expect("TODO: panic message");

    // bob leave

    let proposal = bob_group.leave_group(backend, &bob_signature_keys).unwrap();
    let message = MlsMessageIn::tls_deserialize_exact(proposal.tls_serialize_detached().unwrap())
        .unwrap()
        .extract();
    let message = match message {
        MlsMessageInBody::PrivateMessage(a) => a,
        _ => panic!("uck"),
    };

    let processed = alice_group.process_message(backend, message).unwrap();
    let content = processed.into_content();
    let proposal = match content {
        ProcessedMessageContent::ProposalMessage(p) => p,
        _ => panic!("fuic"),
    };

    alice_group.store_pending_proposal(*proposal);
    let (commit, _, _) = alice_group
        .commit_to_pending_proposals(backend, &alice_signature_keys)
        .unwrap();
    let message = MlsMessageIn::tls_deserialize_exact(commit.tls_serialize_detached().unwrap())
        .unwrap()
        .extract();
    let message = match message {
        MlsMessageInBody::PrivateMessage(a) => a,
        _ => panic!("uck"),
    };

    let processed = alice_group.process_message(backend, message).unwrap();
    let content = processed.into_content();
    let commit = match content {
        ProcessedMessageContent::StagedCommitMessage(c) => c,
        _ => panic!("fuic"),
    };

    alice_group.merge_staged_commit(backend, *commit).unwrap();
}
