use crate::api::BubbleApi;
use crate::application_message::Message;
use crate::helper::resource_fetcher::{ResourceError, ResourceFetcher};
use crate::mls_provider::MlsProvider;
use openmls::framing::MlsMessageOut;
use openmls::prelude::{InnerState, LeafNodeIndex, Member, MlsGroup, TlsSerializeTrait};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::types::Error;
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

pub struct BubbleGroup {
    group: MlsGroup,
}

impl BubbleGroup {
    pub fn new(group: MlsGroup) -> Self {
        Self { group }
    }
}

impl Deref for BubbleGroup {
    type Target = MlsGroup;

    fn deref(&self) -> &Self::Target {
        &self.group
    }
}

impl DerefMut for BubbleGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.group
    }
}

impl BubbleGroup {
    pub fn get_group_members(&self) -> Result<Vec<(Uuid, LeafNodeIndex)>, Error> {
        let members: Vec<Member> = self.group.members().collect();
        let mut client_uuids = Vec::with_capacity(members.len());
        for member in members {
            let client_uuid = Uuid::from_slice(member.credential.identity())?;
            client_uuids.push((client_uuid, member.index));
        }
        Ok(client_uuids)
    }

    pub async fn get_group_members_by_user_uuid(
        &self,
        user_uuid: &Uuid,
        resource_fetcher: &ResourceFetcher,
    ) -> Result<Vec<(Uuid, LeafNodeIndex)>, ResourceError> {
        let members = self.get_group_members();
        let mut out = Vec::with_capacity(members.len());
        for (client_uuid, index) in members {
            let client = resource_fetcher
                .get_client_partial_authentication(&client_uuid)
                .await?;
            if &client.user_uuid == user_uuid {
                out.push((client_uuid, index));
            }
        }
        Ok(out)
    }

    pub fn save_if_needed(&mut self, mls_provider: &MlsProvider) -> Result<(), Error> {
        if matches!(self.group.state_changed(), InnerState::Changed) {
            self.group.save(mls_provider)?
        }
        Ok(())
    }

    pub async fn send_message(
        &self,
        api: &BubbleApi,
        message: &MlsMessageOut,
        exclude: &[Uuid],
    ) -> Result<(), Error> {
        let members = self.get_group_members();
        let recipients = members
            .into_iter()
            .filter(|(uuid, _)| !exclude.contains(uuid))
            .map(|(uuid, _)| uuid)
            .collect::<Vec<_>>();
        let bytes = message.tls_serialize_detached().unwrap();
        api.send_message(recipients, bytes).await.unwrap();
        Ok(())
    }

    pub async fn send_application_message(
        &mut self,
        mls_provider: &MlsProvider,
        api: &BubbleApi,
        signer: &SignatureKeyPair,
        message: &Message,
    ) -> Result<(), ()> {
        let mls_message = serde_json::to_string(message).unwrap();
        let mls_message_bytes = mls_message.as_bytes();
        let mls_out = self
            .group
            .create_message(mls_provider, signer, mls_message_bytes)
            .unwrap();
        self.send_message(api, &mls_out, &[]).await.unwrap();
        Ok(())
    }
}
