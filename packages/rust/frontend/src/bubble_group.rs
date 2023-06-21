use crate::mls_provider::MlsProvider;
use openmls::prelude::{InnerState, LeafNodeIndex, Member, MlsGroup};
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
    pub fn get_group_members(&self) -> Vec<(Uuid, LeafNodeIndex)> {
        let members: Vec<Member> = self.group.members().collect();
        let mut client_uuids = Vec::with_capacity(members.len());
        for member in members {
            let client_uuid = Uuid::from_slice(member.credential.identity()).unwrap();
            client_uuids.push((client_uuid, member.index));
        }
        client_uuids
    }

    pub fn save_if_needed(&mut self, mls_provider: &MlsProvider) -> Result<(), ()> {
        if matches!(self.group.state_changed(), InnerState::Changed) {
            self.group.save(mls_provider).unwrap()
        }
        Ok(())
    }
}
