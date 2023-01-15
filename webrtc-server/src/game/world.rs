use linked_hash_map::LinkedHashMap;
use crate::game::{RoundInput, RoundOutput};
use crate::game::systems::movement_system::MovementSystem;
use crate::game::systems::System;
use crate::messages::entities_updates::EntitiesUpdate;
use crate::messages::player_actions::{PlayerActionType};

pub struct World {
    systems_map: LinkedHashMap<PlayerActionType, Box<dyn System>>,
}

impl World {
    pub fn new() -> World {
        let mut systems_map: LinkedHashMap<_, Box<dyn System>> = LinkedHashMap::new();
        systems_map.insert(PlayerActionType::Move,
                           Box::new(MovementSystem::create()));
        let world = World { systems_map };
        world
    }

    pub fn execute_next_round(&mut self,
                              round_input: &RoundInput)
                              -> RoundOutput {
        for (_, system) in &mut self.systems_map {
            for player_id in &round_input.players_data.newly_quit_players {
                system.remove_player(&player_id);
            }
        }

        for (_, system) in &mut self.systems_map {
            for player_id in &round_input.players_data.newly_joined_players {
                system.join_player(&player_id);
            }
        }

        for (actions_type, players_actions)
        in &round_input.players_actions_for_each_type {
            log::debug!("applying command with type {:?}, {:?}", actions_type, players_actions);
            match self.systems_map.get_mut(&actions_type) {
                Some(system) => {
                    for (player_id, action_payload) in players_actions {
                        match system.apply_player_action(&player_id, &action_payload) {
                            Ok(()) => (),
                            Err(_) => log::error!("action payload failed to parse")
                        }
                    }
                }
                None => log::error!("system not recognised for action type")
            }
        }

        let mut entities_updates: Vec<EntitiesUpdate> = Vec::new();
        for (_, system) in &self.systems_map {
            entities_updates.push(system.collect_entities_updates());
        }

        RoundOutput {
            players_to_disconnect: Vec::new(),
            entities_updates
        }
    }
}
