pub mod movement_system;

use crate::messages::entities_updates::EntityUpdate;

pub trait System {
    fn join_player(&mut self,
                   player_id: &u32);

    fn apply_player_action(&mut self,
                           player_id: &u32,
                           player_action_payload: &Vec<u8>);

    fn remove_player(&mut self,
                     player_id: &u32);

    fn collect_entities_updates(&self) -> Vec<EntityUpdate>;
}
