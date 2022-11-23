use linked_hash_map::LinkedHashMap;
use crate::comms::players_store::PlayersData;
use crate::messages::entities_updates::EntityUpdate;

pub mod world;
pub mod systems;

pub struct RoundInput {
    pub players_data: PlayersData,
    pub players_actions: LinkedHashMap<u32, Vec<u8>>,
}

pub struct RoundOutput {
    pub players_to_disconnect: Vec<u32>,
    pub entities_updates: Vec<EntityUpdate>,
}
