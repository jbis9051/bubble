use crate::api::BubbleApi;
use common::base64::Base64;
use common::http_types::{
    CreateClient, CreateClientResponse, KeyPackagePublic, PublicClient, ReplaceKeyPackages,
};
use openmls::prelude::{KeyPackage, KeyPackageIn};
use tls_codec::{Deserialize, Serialize};

use uuid::Uuid;

impl BubbleApi {
    pub async fn request_key_package(
        &self,
        client_uuid: &Uuid,
    ) -> Result<KeyPackageIn, reqwest::Error> {
        let key_package: KeyPackagePublic = self
            .client
            .get(&format!(
                "{}/v1/client/{}/key_package",
                self.domain, client_uuid
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let key_package = key_package.key_package;
        let key_package_in = KeyPackageIn::tls_deserialize(&mut key_package.as_slice()).unwrap();
        Ok(key_package_in)
    }

    pub async fn replace_key_packages(
        &self,
        client_uuid: &Uuid,
        key_packages: Vec<KeyPackage>,
    ) -> Result<(), reqwest::Error> {
        let _res = self
            .client
            .post(&format!(
                "{}/v1/client/{}/key_packages",
                self.domain, client_uuid
            ))
            .json(&ReplaceKeyPackages {
                key_packages: key_packages
                    .into_iter()
                    .map(|k| Base64(k.tls_serialize_detached().unwrap()))
                    .collect(),
            })
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_client(&self, client_uuid: &Uuid) -> Result<PublicClient, reqwest::Error> {
        let client: PublicClient = self
            .client
            .get(&format!("{}/v1/client/{}", self.domain, client_uuid))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(client)
    }

    pub async fn create_client(
        &self,
        signing_key: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<Uuid, reqwest::Error> {
        let res: CreateClientResponse = self
            .client
            .post(&format!("{}/v1/client", self.domain))
            .json(&CreateClient {
                signing_key: Base64(signing_key),
                signature: Base64(signature),
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res.client_uuid)
    }
}
