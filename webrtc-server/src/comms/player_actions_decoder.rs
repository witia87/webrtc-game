use linked_hash_map::LinkedHashMap;
use prost::{Message};
use crate::messages::commands::{PlayerAction};
use crate::messages::player_actions::PlayerActionType;

pub fn decode_player_actions(collected_player_actions: &LinkedHashMap<u32, Vec<u8>>)
                             -> LinkedHashMap<PlayerActionType, LinkedHashMap<u32, Vec<u8>>> {
    let mut parsed_player_actions = LinkedHashMap::new();

    for (player_id, player_action_bytes) in collected_player_actions {
        match PlayerAction::decode(&**player_action_bytes) {
            Ok(player_action) => {
                match PlayerActionType::from_i32(player_action.player_action_type) {
                    Some(player_action_type) => {
                        if !parsed_player_actions.contains_key(&player_action_type) {
                            parsed_player_actions.insert(player_action_type, LinkedHashMap::new());
                        }
                        let player_actions = parsed_player_actions.get_mut(&player_action_type).unwrap();
                        player_actions.insert(player_id.clone(), player_action.player_action_payload);
                    }
                    None => log::error!("failed to parse player action - ignoring")
                }
            }
            _ => log::error!("failed to parse player action - ignoring")
        }
    }

    parsed_player_actions
}
