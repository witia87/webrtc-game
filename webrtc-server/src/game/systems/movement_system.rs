use linked_hash_map::LinkedHashMap;
use crate::messages::commons::Vector2;
use crate::game::systems::System;
use crate::messages::entities_updates::{EntityUpdate, PlayerPositionUpdatePayload};
use crate::messages::entities_updates::EntityUpdateType::PlayerPositionUpdate;
use crate::messages::player_actions::{MovePlayerActionPayload};
use prost::{DecodeError, Message as ProstMessage};

pub struct MovementSystem {
    players_positions: LinkedHashMap<u32, Vector2>,
}


impl MovementSystem {
    pub fn create() -> MovementSystem {
        let movement_system = MovementSystem { players_positions: LinkedHashMap::new() };
        movement_system
    }
}

impl System for MovementSystem {
    fn join_player(&mut self,
                   player_id: &u32) {
        self.players_positions.insert(player_id.clone(), Vector2 { x: 0f32, y: 0f32 });
    }

    fn apply_player_action(&mut self,
                           player_id: &u32,
                           player_action_payload_bytes: &Vec<u8>) -> Result<(), DecodeError> {
        match MovePlayerActionPayload::decode(player_action_payload_bytes.as_slice()) {
            Ok(player_action_payload) => {
                let current_position = self.players_positions.get(&player_id).unwrap();
                let direction = player_action_payload.direction.unwrap();

                self.players_positions[player_id] =
                    Vector2 {
                        x: current_position.x + direction.x,
                        y: current_position.y + direction.y,
                    };
                Ok(())
            }
            Err(err) => Err(err)
        }
    }

    fn remove_player(&mut self,
                     player_id: &u32) {
        self.players_positions.remove(player_id);
    }

    fn collect_entities_updates(&self) -> Vec<EntityUpdate> {
        let mut entities_updates = Vec::new();
        for player_id in self.players_positions.keys() {
            let new_position = self.players_positions.get(player_id).unwrap();
            entities_updates.push(EntityUpdate {
                entity_update_type: PlayerPositionUpdate as i32,
                entity_update_payload_bytes: PlayerPositionUpdatePayload
                {
                    new_position: Some(new_position.clone())
                }.encode_to_vec(),
            })
        };
        entities_updates
    }
}
