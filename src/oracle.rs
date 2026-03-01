use crate::{ClubId, PlayerId};

pub struct IdInternalizer {
    // No longer needs to store mappings as we use stable IDs directly
}

impl IdInternalizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_internal_club_id(&mut self, external_id: u64) -> ClubId {
        ClubId(external_id as u32)
    }

    pub fn get_internal_player_id(&mut self, external_id: u64) -> PlayerId {
        PlayerId(external_id as u32)
    }
}
