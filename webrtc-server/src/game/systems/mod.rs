pub mod movement_system;

use prost::DecodeError;
use crate::messages::entities_updates::EntitiesUpdate;

pub trait System {
    fn join_player(&mut self,
                   player_id: &u32);

    fn apply_player_action(&mut self,
                           player_id: &u32,
                           player_action: &Vec<u8>) -> Result<(), DecodeError>;

    fn remove_player(&mut self,
                     player_id: &u32);

    fn collect_entities_updates(&self) -> EntitiesUpdate;
}
