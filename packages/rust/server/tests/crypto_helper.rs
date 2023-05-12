use ed25519_dalek::Keypair;
use openmls::prelude::*;

use rand_core_2::OsRng;

// some tests just need a public and private ed25519 keypair, so we cache a pair here for speed, otherwise we generate new keypairs as needed
pub const PUBLIC: &[u8] = &[
    185, 244, 25, 9, 115, 194, 167, 64, 181, 44, 148, 222, 61, 46, 254, 235, 42, 155, 163, 213,
    124, 123, 34, 151, 245, 184, 6, 116, 111, 18, 97, 190,
];
pub const PRIVATE: &[u8] = &[
    212, 139, 203, 143, 152, 23, 140, 184, 49, 125, 44, 89, 240, 71, 172, 95, 65, 11, 227, 156, 25,
    116, 77, 0, 82, 26, 52, 35, 39, 21, 80, 84,
];

pub fn generate_ed25519_keypair() -> Keypair {
    let mut csprng = OsRng {};
    Keypair::generate(&mut csprng)
}

pub fn generate_credential_bundle(
    identity: Vec<u8>,
    credential_type: CredentialType,
    signature_algorithm: SignatureScheme,
    backend: &impl OpenMlsCryptoProvider,
) -> Result<Credential, CredentialError> {
    let credential_bundle =
        CredentialBundle::new(identity, credential_type, signature_algorithm, backend)?;
    Ok(credential_bundle.into_parts().0)
}
