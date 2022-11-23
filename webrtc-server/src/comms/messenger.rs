use std::collections::HashSet;
use std::net::SocketAddr;
use std::time::Duration;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use tokio::{select, time};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use webrtc_unreliable::MessageType::Binary;
use webrtc_unreliable::Server;

pub const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 100;
pub const ROUND_INTERVAL_DURATION: Duration = Duration::from_millis(100);

pub struct ClientsData {
    pub newly_connected_clients: Vec<SocketAddr>,
    pub newly_disconnected_clients: Vec<SocketAddr>,
    pub active_clients: Vec<SocketAddr>,
}

pub struct MessengerTick {
    pub round_index: u32,
    pub total_game_time_elapsed: Duration,
    pub clients_data: ClientsData,
    pub collected_incoming_messages: LinkedHashMap<SocketAddr, Vec<u8>>,
}

pub enum MessengerEventType {
    IncomingMessageReceived(SocketAddr, Vec<u8>),
    MessagesToSendReceived(LinkedHashMap<SocketAddr, Vec<u8>>),
    Tick,
}

pub fn start(rtc_server: Server,
             messages_to_send_receiver: Receiver<LinkedHashMap<SocketAddr, Vec<u8>>>)
             -> Receiver<MessengerTick> {
    let (messenger_tick_sender, messenger_tick_receiver)
        = mpsc::channel(DEFAULT_CHANNEL_BUFFER_SIZE);

    tokio::spawn(listen(rtc_server, messages_to_send_receiver, messenger_tick_sender));

    messenger_tick_receiver
}

async fn listen(mut rtc_server: Server,
                mut messages_to_send_receiver: Receiver<LinkedHashMap<SocketAddr, Vec<u8>>>,
                messenger_tick_sender: Sender<MessengerTick>)
{
    let mut interval = time::interval(ROUND_INTERVAL_DURATION);
    let mut round_index: u32 = 0;
    let now = time::Instant::now();

    let mut collected_incoming_messages: LinkedHashMap<SocketAddr, Vec<u8>> = LinkedHashMap::new();
    let previous_active_connections: HashSet<SocketAddr> = HashSet::new();

    loop {
        let event_type: MessengerEventType = select! {
                _ = interval.tick() => {
                    MessengerEventType::Tick
                },
                Ok(received) = rtc_server.recv() => {
                    MessengerEventType::IncomingMessageReceived(received.remote_addr, received.message.as_ref().to_vec())
                },
                Some(message) = messages_to_send_receiver.recv() => {
                    MessengerEventType::MessagesToSendReceived(message)
                },
            };

        match event_type {
            MessengerEventType::Tick => {
                round_index = round_index + 1;
                let total_game_time_elapsed = now.elapsed();
                let clients_data = create_clients_data(
                    &previous_active_connections, &mut rtc_server);

                messenger_tick_sender.send(MessengerTick {
                    round_index,
                    total_game_time_elapsed,
                    clients_data,
                    collected_incoming_messages: collected_incoming_messages.clone(),
                }).await
                    .map_err(|err| log::warn!("failed to publish incoming message {}", err))
                    .ok();
                collected_incoming_messages.clear();
            }
            MessengerEventType::IncomingMessageReceived(socket_addr, message) => {
                collected_incoming_messages.insert(socket_addr, message);
            }
            MessengerEventType::MessagesToSendReceived(messages) => {
                for (socket_addr, message) in messages {
                    rtc_server.send(&message, Binary, &socket_addr).await
                        .map_err(|err| log::warn!("failed to send out message {}", err))
                        .ok();
                }
            }
        }
    };
}

fn create_clients_data(previous_active_connections: &HashSet<SocketAddr>,
                       server: &mut Server)
                       -> ClientsData {
    let mut newly_connected_clients = Vec::new();

    let active_clients
        = Vec::from_iter(server.connected_clients().cloned().map(|v| v.clone()));

    let mut newly_disconnected_clients_set: LinkedHashSet<SocketAddr>
        = LinkedHashSet::from_iter(active_clients.iter().cloned());

    for socket_addr in &active_clients {
        if !previous_active_connections.contains(&socket_addr) {
            newly_connected_clients.push(socket_addr.clone());
        } else {
            newly_disconnected_clients_set.remove(&socket_addr);
        }
    }

    ClientsData {
        newly_connected_clients,
        newly_disconnected_clients: Vec::from_iter(newly_disconnected_clients_set.iter().cloned()),
        active_clients,
    }
}
