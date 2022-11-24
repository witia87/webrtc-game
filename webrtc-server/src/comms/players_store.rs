use std::net::SocketAddr;
use linked_hash_map::LinkedHashMap;
use crate::comms::messenger::ClientsData;

pub struct PlayersData {
    pub newly_joined_players: Vec<u32>,
    pub newly_quit_players: Vec<u32>,
    pub active_players: Vec<u32>,
}

pub struct PlayersStore {
    next_player_id_to_assign: u32,
    player_ids: LinkedHashMap<SocketAddr, u32>,
}

impl PlayersStore {
    pub fn new() -> PlayersStore {
        let players_store = PlayersStore {
            next_player_id_to_assign: 0,
            player_ids: LinkedHashMap::new(),
        };
        players_store
    }

    pub fn update(&mut self,
                  clients_data: &ClientsData) -> PlayersData {
        let mut newly_joined_players = Vec::new();
        for socket_addr in &clients_data.newly_connected_clients {
            if !self.player_ids.contains_key(&socket_addr) {
                self.player_ids.insert(socket_addr.clone(), self.next_player_id_to_assign);
                newly_joined_players.push(self.next_player_id_to_assign);
                self.next_player_id_to_assign = self.next_player_id_to_assign + 1;
            } else {
                log::error!("player already joined");
            }
        }

        let mut newly_quit_players = Vec::new();
        for socket_addr in &clients_data.newly_disconnected_clients {
            if self.player_ids.contains_key(&socket_addr) {
                let disconnected_player_id = self.player_ids.remove(&socket_addr).unwrap();
                newly_quit_players.push(disconnected_player_id);
            } else {
                log::error!("player already quit");
            }
        }

        let mut active_players = Vec::new();
        for socket_addr in &clients_data.active_clients {
            if self.player_ids.contains_key(&socket_addr) {
                let active_player_id = self.player_ids.get(&socket_addr).unwrap();
                active_players.push(active_player_id.clone());
            } else {
                log::error!("player is not registered as active");
            }
        }
        if active_players.len() != clients_data.active_clients.len() {
            log::error!("mismatching amounts of active players and clients");
        }

        PlayersData {
            newly_joined_players,
            newly_quit_players,
            active_players,
        }
    }

    pub fn get_player_id(&self,
                         socket_addr: &SocketAddr)
                         -> Option<&u32> {
        self.player_ids.get(socket_addr)
    }
}
