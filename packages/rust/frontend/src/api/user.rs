use crate::api::BubbleApi;
use common::base64::Base64;
use common::http_types::{
    ClientsResponse, CreateUser, CreateUserResponse, Login, PublicClient, PublicUser,
    SessionTokenResponse,
};

use uuid::Uuid;

impl BubbleApi {
    pub async fn register(
        &self,
        email: String,
        username: String,
        password: String,
        name: String,
        identity: Vec<u8>,
    ) -> Result<Uuid, reqwest::Error> {
        let res = self
            .client
            .post(&format!("{}/v1/user/register", self.domain))
            .json(&CreateUser {
                email,
                username,
                password,
                name,
                identity: Base64(identity),
            })
            .send()
            .await?;

        if res.error_for_status_ref().is_err() {
            panic!("Error registering user: {:?}", res.text().await?);
        }

        let res: CreateUserResponse = res.error_for_status()?.json().await?;
        Ok(res.user_uuid)
    }

    pub async fn login(
        &self,
        username_or_email: String,
        password: String,
    ) -> Result<SessionTokenResponse, reqwest::Error> {
        let res: SessionTokenResponse = self
            .client
            .post(&format!("{}/v1/user/session", self.domain))
            .json(&Login {
                username_or_email,
                password,
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }

    pub async fn get_user(&self, uuid: &Uuid) -> Result<PublicUser, reqwest::Error> {
        let user: PublicUser = self
            .client
            .get(&format!("{}/v1/user/{}", self.domain, uuid))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(user)
    }

    pub async fn get_user_clients(&self, uuid: &Uuid) -> Result<Vec<PublicClient>, reqwest::Error> {
        let clients: ClientsResponse = self
            .client
            .get(&format!("{}/v1/user/{}/clients", self.domain, uuid))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(clients.clients)
    }
}
