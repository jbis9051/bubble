use crate::api::BubbleApi;
use crate::js_interface::{FrontendInstance, GlobalAccountData};
use crate::mls_provider::MlsProvider;
use crate::models::kv::{AccountKv, GlobalKv};
use crate::types::SIGNATURE_SCHEME;
use crate::Error;
use bridge_macro::bridge;
use common::base64;
use common::base64::Base64;
use common::http_types::{PublicUser, SessionTokenResponse};
use ed25519_dalek::{Keypair, SecretKey, Signer};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::OpenMlsCryptoProvider;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use std::fs;
use std::path::Path;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[bridge]
pub struct UserOut {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
    pub primary_client_uuid: Option<Uuid>,
    pub identity: Base64,
}

impl From<PublicUser> for UserOut {
    fn from(value: PublicUser) -> Self {
        Self {
            uuid: value.uuid,
            username: value.username,
            name: value.name,
            primary_client_uuid: value.primary_client_uuid,
            identity: value.identity,
        }
    }
}

impl FrontendInstance {
    #[bridge]
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

    #[bridge]
    pub async fn login(&self, username_or_email: String, password: String) -> Result<Uuid, Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        let res = api.login(username_or_email, password).await?;
        let user_uuid = self.login_with_token(res).await?;
        Ok(user_uuid)
    }

    async fn login_with_token(&self, res: SessionTokenResponse) -> Result<Uuid, Error> {
        let path = format!(
            "{}/accounts/{}.db",
            &self.static_data.data_directory, &res.user_uuid
        );
        if !Path::new(&path).exists() {
            // TODO: fs exist race condition
            return Err(Error::WrongDevice);
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

    #[bridge]
    pub async fn logout(&self) -> Result<(), Error> {
        GlobalKv::delete(&self.global_database, "current_account").await?;
        Ok(())
    }

    #[bridge]
    pub async fn forgot(&self, email: String) -> Result<(), Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        api.forgot(email).await?;
        Ok(())
    }

    #[bridge]
    pub async fn confirm(&self, token: Uuid) -> Result<Uuid, Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        let res = api.confirm(token).await?;
        let user_uuid = self.login_with_token(res).await?;
        Ok(user_uuid)
    }

    #[bridge]
    pub async fn forgot_confirm(&self, password: String, token: Uuid) -> Result<(), Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        api.forgot_confirm(password, token).await?;
        Ok(())
    }

    #[bridge]
    pub async fn forgot_check(&self, token: Uuid) -> Result<bool, Error> {
        let api = BubbleApi::new(self.static_data.domain.clone(), None);
        let res = api.forgot_check(token).await?;
        Ok(res)
    }

    #[bridge]
    pub async fn search(&self, query: String) -> Result<Vec<UserOut>, Error> {
        let global = self.account_data.read().await;
        let global_data = global.as_ref().ok_or_else(|| Error::NoGlobalAccountData)?;
        let api = BubbleApi::new(
            global_data.domain.clone(),
            Some(global_data.bearer.read().await.clone()),
        );
        let res = api.search(query).await?;
        let out = res.into_iter().map(|user| user.into()).collect();
        Ok(out)
    }
}
