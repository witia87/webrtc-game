use std::net::SocketAddr;
use clap::{App, Arg};
use linked_hash_map::LinkedHashMap;
use tokio::sync::mpsc;

use webrtc_unreliable::Server as RtcServer;
use webrtc_server::comms::commands_decoder::decode_commands;
use webrtc_server::comms::entities_updates_encoder::encode_entities_updates_notification;
use webrtc_server::comms::messenger;
use webrtc_server::comms::messenger::DEFAULT_CHANNEL_BUFFER_SIZE;
use webrtc_server::comms::player_actions_decoder::decode_player_actions;
use webrtc_server::comms::player_setups_encoder::encode_player_setup_notification;
use webrtc_server::comms::players_store::PlayersStore;
use webrtc_server::game::RoundInput;
use webrtc_server::game::world::World;
use webrtc_server::html::html_server;
use webrtc_server::messages::commands::CommandType;

#[tokio::main]
async fn main() {
    log4rs::init_file("logger_config.yaml", Default::default()).unwrap();

    let matches = App::new("echo_server")
        .arg(
            Arg::with_name("data")
                .short("d")
                .long("data")
                .takes_value(true)
                .required(true)
                .help("listen on the specified address/port for UDP WebRTC data channels"),
        )
        .arg(
            Arg::with_name("public")
                .short("p")
                .long("public")
                .takes_value(true)
                .required(true)
                .help("advertise the given address/port as the public WebRTC address/port"),
        )
        .arg(
            Arg::with_name("http")
                .short("h")
                .long("http")
                .takes_value(true)
                .required(true)
                .help("listen on the specified address/port for incoming HTTP (session reqeusts and test page"),
        )
        .get_matches();

    let webrtc_listen_addr = matches
        .value_of("data")
        .unwrap()
        .parse()
        .expect("could not parse WebRTC data address/port");

    let public_webrtc_addr = matches
        .value_of("public")
        .unwrap()
        .parse()
        .expect("could not parse advertised public WebRTC data address/port");

    let session_listen_addr = matches
        .value_of("http")
        .unwrap()
        .parse()
        .expect("could not parse HTTP address/port");

    let rtc_server = RtcServer::new(webrtc_listen_addr, public_webrtc_addr)
        .await
        .expect("could not start RTC server");

    let session_endpoint = rtc_server.session_endpoint();

    html_server::start_hosting(session_endpoint, session_listen_addr);

    let (messages_to_send_sender, messages_to_send_receiver)
        = mpsc::channel(DEFAULT_CHANNEL_BUFFER_SIZE);
    let mut messenger_ticks_receiver = messenger::start(rtc_server, messages_to_send_receiver);

    let mut players_store = PlayersStore::new();

    let mut world = World::new();

    loop {
        let messenger_tick = messenger_ticks_receiver.recv().await.unwrap();

        let players_data = players_store.update(&messenger_tick.clients_data);

        let player_setup_notifications: Vec<(SocketAddr, Vec<u8>)> = messenger_tick.clients_data.newly_connected_clients.iter()
            .map(|socket_addr|
                (socket_addr.clone(), encode_player_setup_notification(
                    players_store.get_player_id(&socket_addr).unwrap())))
            .collect();
        if !player_setup_notifications.is_empty() {
            messages_to_send_sender.send(player_setup_notifications).await
                .expect("failed to send player setup notifications");
        }

        let commands_map
            = decode_commands(messenger_tick.collected_incoming_messages, &players_store);

        let players_actions = decode_player_actions(
            commands_map.get(&CommandType::PlayerActionCommand).unwrap_or(&LinkedHashMap::new()));

        let round_output =
            world.execute_next_round(&RoundInput { players_data, players_actions_for_each_type: players_actions });

        let entities_updates_notifications = messenger_tick.clients_data.active_clients.iter()
            .map(|socket_addr|
                (socket_addr.clone(), encode_entities_updates_notification(round_output.entities_updates.clone())))
            .collect();
        messages_to_send_sender.send(entities_updates_notifications).await
            .expect("failed to send entities updates notifications");
    }
}
