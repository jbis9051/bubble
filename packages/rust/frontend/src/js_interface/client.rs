use crate::api::BubbleApi;
use crate::helper::helper::get_this_client_mls_resources;
use crate::js_interface::FrontendInstance;
use crate::mls_provider::MlsProvider;
use crate::types::CIPHERSUITE;
use crate::Error;
use openmls::prelude::{
    Credential, CredentialType, CredentialWithKey, CryptoConfig, KeyPackage, ProtocolVersion,
    SignaturePublicKey,
};

impl FrontendInstance {
    pub async fn replace_key_packages(&self) -> Result<(), Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::NoGlobalAccountData)?;
        let account_db = &global_data.database;
        let user_uuid = global_data.user_uuid;
        let client_uuid = global_data.client_uuid.read().await.unwrap();
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let mls_provider = MlsProvider::new(account_db.clone());
        let (signature, _) =
            get_this_client_mls_resources(&client_uuid, account_db, &mls_provider).await?;

        let identity = format!("keypackage_{}_{}", user_uuid, client_uuid);
        let credential = Credential::new(identity.into_bytes(), CredentialType::Basic)?;
        let public = SignaturePublicKey::from(signature.public());

        let num_key_packages = 100;

        let mut key_packages = Vec::with_capacity(num_key_packages);

        for _ in 0..num_key_packages {
            let key_package = KeyPackage::builder()
                .build(
                    CryptoConfig {
                        ciphersuite: CIPHERSUITE,
                        version: ProtocolVersion::default(),
                    },
                    &mls_provider,
                    &signature,
                    CredentialWithKey {
                        credential: credential.clone(),
                        signature_key: public.clone(),
                    },
                )
                .unwrap();
            key_packages.push(key_package);
        }

        api.replace_key_packages(&client_uuid, key_packages).await?;

        Ok(())
    }
}
