use crate::api::BubbleApi;
use crate::js_interface::{FrontendInstance, GlobalAccountData};
use crate::mls_provider::MlsProvider;
use crate::models::kv::{AccountKv, GlobalKv};
use crate::types::SIGNATURE_SCHEME;
use crate::Error;
use common::base64;
use common::http_types::SessionTokenResponse;
use ed25519_dalek::{Keypair, SecretKey, Signer};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::OpenMlsCryptoProvider;
use rand_core::OsRng;
use sqlx::sqlite::SqlitePoolOptions;
use std::fs;
use std::path::Path;
use tokio::sync::RwLock;
use uuid::Uuid;

// create a database with sqlite
// set database to update global db with user entry
// identity = key
// make a key and store in db
// send an api route to create a user from uuid create account db
// not updating global var
// `domain`, `bearer`, `client_uuid`
impl FrontendInstance {
    pub async fn register(
        &self,
        username: String,
        password: String,
        name: String,
        email: String,
    ) -> Result<(), Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        let mut csprng = OsRng {};
        let user_keys = Keypair::generate(&mut csprng);
        let public = user_keys.public.to_bytes().to_vec();
        let user_uuid = api
            .register(email, username, password, name, public.clone())
            .await?;
        fs::create_dir_all(format!("{}/accounts", &self.static_data.data_directory)).unwrap();
        let path = format!(
            "{}/accounts/{}.db",
            &self.static_data.data_directory, &user_uuid
        );
        let account_db = SqlitePoolOptions::new()
            .connect(&format!("sqlite:{}?mode=rwc", path))
            .await?;
        sqlx::migrate!("./migrations/account")
            .run(&account_db)
            .await?;
        AccountKv::set(
            &account_db,
            "user_private_key",
            &base64::serialize(user_keys.secret.to_bytes().as_ref()),
        )
        .await?;
        AccountKv::set(&account_db, "domain", &self.static_data.domain).await?;
        Ok(())
    }

    pub async fn login(&self, username_or_email: String, password: String) -> Result<Uuid, Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        let res = api.login(username_or_email, password).await?;
        let user_uuid = self.login_with_token(res).await?;
        Ok(user_uuid)
    }

    pub async fn login_with_token(&self, res: SessionTokenResponse) -> Result<Uuid, Error> {
        let path = format!(
            "{}/accounts/{}.db",
            &self.static_data.data_directory, &res.user_uuid
        );
        if !Path::new(&path).exists() {
            // TODO: fs exist race condition
            panic!("logging into an account on a device other than the one it was created on is not supported yet");
        }
        let account_db = SqlitePoolOptions::new()
            .connect(&format!("sqlite:{}", path))
            .await?;

        AccountKv::set(&account_db, "bearer", &res.bearer.to_string()).await?;
        GlobalKv::set(
            &self.global_database,
            "current_account",
            &res.user_uuid.to_string(),
        )
        .await?;

        let domain = AccountKv::get(&account_db, "domain").await?.unwrap();

        // we've logged in, now if needed, we must create a client

        let client_uuid = AccountKv::get(&account_db, "client_uuid").await?;

        if let Some(client_uuid) = client_uuid {
            let mut guard = self.account_data.write().await;
            *guard = Some(GlobalAccountData {
                database: account_db,
                bearer: RwLock::new(res.bearer.to_string()),
                domain,
                user_uuid: res.user_uuid,
                client_uuid: RwLock::new(Some(Uuid::parse_str(&client_uuid).unwrap())),
            });
            // client already exists
            return Ok(res.user_uuid);
        }

        // we must create a client

        let user_private_key = base64::deserialize(
            &AccountKv::get(&account_db, "user_private_key")
                .await
                .unwrap()
                .unwrap(),
        );
        let user_private_key = SecretKey::from_bytes(&user_private_key).unwrap();

        let user_keypair = Keypair {
            public: (&user_private_key).into(),
            secret: user_private_key,
        };

        let client_signature_keypair = SignatureKeyPair::new(SIGNATURE_SCHEME).unwrap();

        let signature_of_signing_key = user_keypair.sign(client_signature_keypair.public());

        let api = BubbleApi::new(
            self.static_data.domain.clone(),
            Some(res.bearer.to_string()),
        );

        let client_uuid = api
            .create_client(
                client_signature_keypair.public().to_vec(),
                signature_of_signing_key.to_bytes().to_vec(),
            )
            .await?;

        let mls_provider = MlsProvider::new(account_db.clone());

        AccountKv::set(&account_db, "client_uuid", &client_uuid.to_string()).await?;
        AccountKv::set(
            &account_db,
            "client_public_signature_key",
            &base64::serialize(client_signature_keypair.public()),
        )
        .await?;

        client_signature_keypair.store(mls_provider.key_store())?;

        let mut guard = self.account_data.write().await;
        *guard = Some(GlobalAccountData {
            database: account_db,
            bearer: RwLock::new(res.bearer.to_string()),
            domain,
            user_uuid: res.user_uuid,
            client_uuid: RwLock::new(Some(client_uuid)),
        });
        Ok(res.user_uuid)
    }

    pub async fn logout(&self) -> Result<(), Error> {
        GlobalKv::delete(&self.global_database, "current_account").await?;
        Ok(())
    }

    pub async fn forgot(&self, email: String) -> Result<(), Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        api.forgot(email).await?;
        Ok(())
    }

    pub async fn confirm(&self, token: Uuid) -> Result<Uuid, Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        let res = api.confirm(token).await?;
        let user_uuid = self.login_with_token(res).await?;
        Ok(user_uuid)
    }

    pub async fn forgot_confirm(&self, password: String, token: Uuid) -> Result<(), Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        api.forgot_confirm(password, token).await?;
        Ok(())
    }
    //different because its an endpoint?

    pub async fn forgot_check(&self, token: Uuid) -> Result<bool, Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        let res = api.forgot_check(token).await;
        Ok(res.is_ok())
    }
}
