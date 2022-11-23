use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;
use crate::messages::commands::{Command};
use crate::game::systems::movement_system::MovementSystem;
use crate::game::systems::System;
use crate::messages::notifications::WorldStateUpdate;
use crate::messages::player_actions::{PlayerActionType};

pub struct World {
    systems_map: HashMap<PlayerActionType, Box<dyn System>>,
}

impl World {
    pub fn create() -> World {
        let mut systems_map: HashMap<_, Box<(dyn System)>> = HashMap::new();
        systems_map.insert(PlayerActionType::Move,
                           Box::new(MovementSystem::create()));
        let world = World { systems_map };
        world
    }

    pub fn update(&self,
                  commands: &LinkedHashMap<u32, Command>)
                  -> WorldStateUpdate {
        /*for (player_id, command) in commands {
            match CommandType::from_i32(command.command_type) {
                Some(CommandType::PlayerActionCommand) => {
                    match PlayerAction::decode(&*command.command_payload) {
                        Ok(player_action) => {
                            let player_action_type = PlayerActionType::from_i32(player_action.player_action_type).unwrap();
                            self.systems_map[&player_action_type]
                                .apply_player_action(player_id, &player_action.player_action_payload);
                        }
                        Err(_) => log::error!("failed to decode action tick")
                    }
                }
                None => {}
            }
        }*/

        WorldStateUpdate {
            entities_updates: Vec::new()
        }
    }
}
