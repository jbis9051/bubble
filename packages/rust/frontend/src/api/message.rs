use crate::api::BubbleApi;
use common::base64::Base64;
use common::http_types::SendMessage;
use reqwest::Error;
use uuid::Uuid;

impl BubbleApi {
    pub async fn send_message(
        &self,
        client_uuids: Vec<Uuid>,
        message: Vec<u8>,
    ) -> Result<(), Error> {
        let message = Base64(message);
        let message = SendMessage {
            client_uuids,
            message,
        };
        self.client
            .post(&format!("{}/v1/messages", self.domain))
            .json(&message)
            .send()
            .await?;
        Ok(())
    }
}
