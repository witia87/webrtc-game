use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use webrtc_unreliable::MessageType::Binary;
use webrtc_unreliable::Server;

#[derive(Clone)]
pub struct Message {
    pub socket_addr: SocketAddr,
    pub data: Vec<u8>,
}

enum MessagingDirection {
    IN,
    OUT,
}

pub struct Messenger {}

impl Messenger {
    pub fn create(rtc_server: Server,
                  received_messages_sender: Sender<Message>,
                  messages_to_send_receiver: Receiver<Message>)
                  -> Messenger {
        let messenger = Messenger {};

        tokio::spawn(Messenger::listen(
            rtc_server,
            received_messages_sender,
            messages_to_send_receiver));

        messenger
    }

    async fn listen(mut rtc_server: Server,
                    received_messages_sender: Sender<Message>,
                    mut messages_to_send_receiver: Receiver<Message>)
    {
        loop {
            let (direction, message) = select! {
                Some(message) = messages_to_send_receiver.recv() => {
                    println!("message for sending received");
                    (MessagingDirection::OUT, message)
                },
                Ok(received) = rtc_server.recv() => {
                    println!("incoming message received");
                    (MessagingDirection::IN, Message{
                        socket_addr: received.remote_addr,
                        data: received.message.as_ref().to_vec()})
                }
            };

            match direction {
                MessagingDirection::OUT => {
                    rtc_server.send(&message.data, Binary, &message.socket_addr).await
                        .map_err(|err| log::warn!("failed to send out message {}", err))
                        .ok();
                }
                MessagingDirection::IN => {
                    received_messages_sender.send(message).await
                        .map_err(|err| log::warn!("failed to publish incoming message {}", err))
                        .ok();
                }
            }
        };
    }
}