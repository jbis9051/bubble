use crate::api::BubbleApi;
use crate::models::account::client::Client;
use crate::models::account::user::User;
use crate::types::DbPool;
use common::http_types::{PublicClient, PublicUser};
use ed25519_dalek::{PublicKey, Signature};
use uuid::Uuid;

/// Fetches various resources using various authentication procedures.
///
/// **Full-Authentication**: The resource is authenticated against both the local cache and the API. TOFU is used if the resource is not in the cache. Full authentication requires a network request.
///
/// **Partial-Authentication**: The resource is authenticated against the local cache. TOFU is used if the resource is not in the cache.
///
/// **No-Authentication**: The resource is not authenticated.
pub struct ResourceFetcher {
    api: BubbleApi,
    account_db: DbPool,
}

#[derive(Debug, thiserror::Error)]
pub enum ResourceError {
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("cache does not match api: {0:?} {1:?}")]
    CacheDoesNotMatchApi(ResourceType, Uuid),
    #[error("invalid client signature: {0:?}")]
    InvalidClientSignature(Uuid),
    #[error("signature error: {0}")]
    Signature(#[from] ed25519_dalek::SignatureError),
}

#[derive(Debug)]
pub enum ResourceType {
    User,
    Client,
}

impl ResourceFetcher {
    pub fn new(api: BubbleApi, account_db: DbPool) -> Self {
        Self { api, account_db }
    }

    /// Retrieve the Clients for a given User. The Clients are retrieved from the API and authenticated with full-authentication.
    pub async fn get_clients_full_authentication(
        &self,
        user_uuid: &Uuid,
    ) -> Result<Vec<PublicClient>, ResourceError> {
        let user = self.get_user_full_authentication(user_uuid).await?;
        let clients = self.api.get_user_clients(user_uuid).await?;
        self.authenticate_clients_against_user_identity(&user.identity, &clients)?;

        for client in clients.clone() {
            let mut client: Client = client.into();
            Client::delete_by_uuid(&self.account_db, &client.uuid).await?;
            client.create(&self.account_db).await?;
        }

        Ok(clients)
    }

    pub fn authenticate_clients_against_user_identity<'a>(
        &self,
        user_identity: &[u8],
        clients: impl IntoIterator<Item = &'a PublicClient>,
    ) -> Result<(), ResourceError> {
        let user_key = PublicKey::from_bytes(user_identity)?;
        for client in clients {
            let signature = Signature::from_bytes(&client.signature)?;
            if user_key
                .verify_strict(&client.signing_key, &signature)
                .is_err()
            {
                return Err(ResourceError::InvalidClientSignature(client.uuid));
            }
        }
        Ok(())
    }

    /// Retrieve a user. The user's identity is authenticated with full-authentication.
    pub async fn get_user_full_authentication(
        &self,
        user_uuid: &Uuid,
    ) -> Result<PublicUser, ResourceError> {
        let local_user = User::try_from_uuid(&self.account_db, user_uuid)
            .await?;
        let api_user = self.api.get_user(user_uuid).await?;

        if let Some(cache_user) = local_user {
            if cache_user.identity != *api_user.identity {
                return Err(ResourceError::CacheDoesNotMatchApi(
                    ResourceType::User,
                    *user_uuid,
                ));
            }
        }

        Ok(api_user)
    }

    /// Retrieve a user. The user's identity is authenticated with partial-authentication.
    pub async fn get_user_partial_authentication(
        &self,
        user_uuid: &Uuid,
    ) -> Result<User, ResourceError> {
        let local_user = User::try_from_uuid(&self.account_db, user_uuid)
            .await?;
        if let Some(cache_user) = local_user {
            return Ok(cache_user);
        }
        let api_user = self.api.get_user(user_uuid).await?;
        let mut user: User = api_user.into();
        user.create(&self.account_db).await?;
        Ok(user)
    }

    /// Retrieve the Clients for a given User. The Clients are retrieved from the API and authenticated with partial-authentication.
    pub async fn get_clients_partial_authentication(
        &self,
        user_uuid: &Uuid,
    ) -> Result<Vec<PublicClient>, ResourceError> {
        let user = self.get_user_partial_authentication(user_uuid).await?;
        let clients = self.api.get_user_clients(user_uuid).await?;
        self.authenticate_clients_against_user_identity(&user.identity, &clients)?;
        Ok(clients)
    }

    /// Retrieves a Client from a UUID. The Client is retrieved from the database. If the Client is in the database, no further checks are performed. If the Client is not in the database, the Client is retrieved from the API using partial-authentication.
    pub async fn get_client_partial_authentication(
        &self,
        client_uuid: &Uuid,
    ) -> Result<Client, ResourceError> {
        let local_client = Client::try_from_uuid(&self.account_db, client_uuid)
            .await?;
        if let Some(cache_client) = local_client {
            return Ok(cache_client);
        }
        let api_client = self.api.get_client(client_uuid).await?;
        let user = self
            .get_user_partial_authentication(&api_client.user_uuid)
            .await?;
        self.authenticate_clients_against_user_identity(&user.identity, &[api_client.clone()])?;
        let mut client: Client = api_client.into();
        client.create(&self.account_db).await?;
        Ok(client)
    }
}
