use std::net::SocketAddr;
use linked_hash_map::LinkedHashMap;
use prost::Message;
use crate::messages::commands::{Command, CommandType};

pub fn parse_commands(collected_incoming_messages: LinkedHashMap<SocketAddr, Vec<u8>>,
                      players_mapper: fn(&SocketAddr) -> Option<u32>)
                      -> LinkedHashMap<CommandType, LinkedHashMap<u32, Vec<u8>>> {
    let mut parsed_commands = LinkedHashMap::new();

    for (socket_addr, data) in collected_incoming_messages {
        match (players_mapper(&socket_addr), Command::decode(&*data)) {
            (Some(player_id), Ok(command)) => {
                match CommandType::from_i32(command.command_type) {
                    Some(command_type) => {
                        if !parsed_commands.contains_key(&command_type) {
                            parsed_commands.insert(command_type, LinkedHashMap::new());
                        }
                        let commands = parsed_commands.get_mut(&command_type).unwrap();
                        commands.insert(player_id, command.command_payload);
                    }
                    None => log::error!("failed to parse a command - ignoring")
                }
            }
            (_, _) => log::error!("failed to parse a command - ignoring")
        }
    }

    parsed_commands
}