use crate::api::BubbleApi;
use common::http_types::{ClientsResponse, PublicClient, PublicUser};
use uuid::Uuid;

impl BubbleApi {
    pub async fn get_user(&self, uuid: &Uuid) -> Result<PublicUser, reqwest::Error> {
        let user: PublicUser = self
            .client
            .get(&format!("{}/v1/users/{}", self.domain, uuid))
            .send()
            .await?
            .json()
            .await?;
        Ok(user)
    }

    pub async fn get_user_clients(&self, uuid: &Uuid) -> Result<Vec<PublicClient>, reqwest::Error> {
        let clients: ClientsResponse = self
            .client
            .get(&format!("{}/v1/users/{}/clients", self.domain, uuid))
            .send()
            .await?
            .json()
            .await?;
        Ok(clients.clients)
    }
}
