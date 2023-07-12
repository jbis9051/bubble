use crate::api::BubbleApi;
use crate::application_message::Message;
use crate::helper::helper::parse_identity;

use crate::mls_provider::MlsProvider;
use crate::Error;
use openmls::framing::MlsMessageOut;
use openmls::prelude::{GroupId, InnerState, LeafNodeIndex, Member, MlsGroup, TlsSerializeTrait};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::OpenMlsCryptoProvider;

use std::ops::{Deref, DerefMut};
use uuid::Uuid;

pub struct BubbleGroup {
    group: MlsGroup,
    group_uuid: Uuid,
}

impl BubbleGroup {
    pub fn new(group: MlsGroup) -> Self {
        let group_uuid = Uuid::from_slice(group.group_id().as_slice()).unwrap();
        Self { group, group_uuid }
    }

    pub fn new_from_uuid(
        group_uuid: &Uuid,
        mls_provider: &impl OpenMlsCryptoProvider,
    ) -> Option<Self> {
        MlsGroup::load(&GroupId::from_slice(group_uuid.as_ref()), mls_provider).map(Self::new)
    }

    pub fn group_uuid(&self) -> Uuid {
        self.group_uuid
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

pub struct BubbleMember {
    pub index: LeafNodeIndex,
    pub user_uuid: Uuid,
    pub client_uuid: Uuid,
}

impl BubbleGroup {
    pub fn get_group_members(&self) -> Result<Vec<BubbleMember>, Error> {
        let members: Vec<Member> = self.group.members().collect();
        let mut client_uuids = Vec::with_capacity(members.len());
        for member in members {
            // "client_{user_uuid}_{client_uuid}"
            let identity = member.credential.identity();
            let (user_uuid, client_uuid) = parse_identity(identity).unwrap();

            client_uuids.push(BubbleMember {
                index: member.index,
                user_uuid,
                client_uuid,
            });
        }
        Ok(client_uuids)
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
        exclude_client: &[Uuid],
    ) -> Result<(), Error> {
        let members = self.get_group_members()?;
        let recipients = members
            .into_iter()
            .map(|m| m.client_uuid)
            .filter(|uuid| !exclude_client.contains(uuid))
            .collect::<Vec<_>>();
        let bytes = message.tls_serialize_detached()?;
        api.send_message(recipients, bytes, self.group_uuid).await?;
        Ok(())
    }

    pub async fn send_application_message(
        &mut self,
        mls_provider: &MlsProvider,
        api: &BubbleApi,
        signer: &SignatureKeyPair,
        message: &Message,
        exclude_client: &[Uuid],
    ) -> Result<(), Error> {
        let mls_message = serde_json::to_string(message)?;
        let mls_message_bytes = mls_message.as_bytes();
        let mls_out = self
            .group
            .create_message(mls_provider, signer, mls_message_bytes)?;
        self.send_message(api, &mls_out, exclude_client).await?;
        Ok(())
    }
}
