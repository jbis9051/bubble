use crate::types::CIPHERSUITE;
use openmls::prelude::*;
use openmls_traits::signatures::Signer;
use openmls_traits::OpenMlsCryptoProvider;

pub fn generate_key_package<KeyStore: OpenMlsKeyStore>(
    backend: &impl OpenMlsCryptoProvider<KeyStoreProvider = KeyStore>,
    signer: &impl Signer,
    credential_with_key: CredentialWithKey,
) -> Result<KeyPackage, KeyPackageNewError<KeyStore::Error>> {
    KeyPackage::builder().build(
        CryptoConfig {
            ciphersuite: CIPHERSUITE,
            version: ProtocolVersion::default(),
        },
        backend,
        signer,
        credential_with_key,
    )
}
