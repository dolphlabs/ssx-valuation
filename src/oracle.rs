use std::collections::BTreeMap;
use crate::{ClubId, PlayerId};

pub struct IdInternalizer {
    external_to_club: BTreeMap<u64, ClubId>,
    external_to_player: BTreeMap<u64, PlayerId>,
    next_club_id: u32,
    next_player_id: u32,
}

impl IdInternalizer {
    pub fn new() -> Self {
        Self {
            external_to_club: BTreeMap::new(),
            external_to_player: BTreeMap::new(),
            next_club_id: 0,
            next_player_id: 0,
        }
    }

    pub fn get_internal_club_id(&mut self, external_id: u64) -> ClubId {
        *self.external_to_club.entry(external_id).or_insert_with(|| {
            let id = ClubId(self.next_club_id);
            self.next_club_id += 1;
            id
        })
    }

    pub fn get_internal_player_id(&mut self, external_id: u64) -> PlayerId {
        *self.external_to_player.entry(external_id).or_insert_with(|| {
            let id = PlayerId(self.next_player_id);
            self.next_player_id += 1;
            id
        })
    }
}
