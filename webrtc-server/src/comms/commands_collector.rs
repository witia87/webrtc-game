use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};
use crate::comms::messenger::Message;
use crate::comms::ticker::Tick;

pub struct CommandsCollector {
    commands_map: Arc<Mutex<HashMap<SocketAddr, Message>>>,
}

impl CommandsCollector {
    pub fn create(messages_receiver: Receiver<Message>,
                  ticks_receiver: Receiver<Tick>,
                  collected_commands_sender: Sender<HashMap<SocketAddr, Message>>)
                  -> CommandsCollector {
        let commands_collector = CommandsCollector {
            commands_map: Arc::new(Mutex::new(HashMap::new()))
        };

        commands_collector.start_listening_to_messages(messages_receiver);
        commands_collector.start_listening_to_ticks(ticks_receiver, collected_commands_sender);

        commands_collector
    }

    pub fn start_listening_to_messages(&self,
                                       messages_receiver: Receiver<Message>) {
        let shared_commands_map = self.commands_map.clone();
        tokio::spawn(
            Self::listen_to_messages(messages_receiver, shared_commands_map));
    }

    pub fn start_listening_to_ticks(&self,
                                    ticks_receiver: Receiver<Tick>,
                                    collected_commands_sender: Sender<HashMap<SocketAddr, Message>>) {
        let shared_commands_map = self.commands_map.clone();
        tokio::spawn(
            Self::listen_to_ticks(
                ticks_receiver,
                collected_commands_sender,
                shared_commands_map));
    }

    async fn listen_to_ticks(mut ticks_receiver: Receiver<Tick>,
                             collected_commands_sender: Sender<HashMap<SocketAddr, Message>>,
                             shared_commands_map: Arc<Mutex<HashMap<SocketAddr, Message>>>) {
        loop {
            match ticks_receiver.recv().await {
                Some(tick) => {
                    dbg!(tick.index, tick.time_elapsed);
                    let cloned_commands = shared_commands_map.lock().unwrap().clone();
                    shared_commands_map.lock().unwrap().clear();
                    collected_commands_sender.send(cloned_commands).await
                        .map_err(|err| log::warn!("failed to publish collected commands {}", err))
                        .ok();
                }
                None => log::error!("failed to receive tick")
            };
        }
    }

    async fn listen_to_messages(mut messages_receiver: Receiver<Message>,
                                shared_commands_map: Arc<Mutex<HashMap<SocketAddr, Message>>>) {
        loop {
            match messages_receiver.recv().await {
                Some(message) => {
                    shared_commands_map.lock().unwrap().insert(message.socket_addr, message);
                }
                None => {
                    log::error!("failed to receive message from CommandsCollector receiver");
                }
            };
        }
    }
}
