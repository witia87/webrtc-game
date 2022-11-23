use clap::{App, Arg};
use tokio::sync::mpsc;

use webrtc_unreliable::Server as RtcServer;
use webrtc_server::comms::messenger;
use webrtc_server::comms::messenger::DEFAULT_CHANNEL_BUFFER_SIZE;
use webrtc_server::comms::players_store::PlayersStore;
use webrtc_server::html::html_server;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

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

    loop {
        let messenger_tick = messenger_ticks_receiver.recv().await.unwrap();

        let players_data = players_store.update(&messenger_tick.clients_data);

        println!("Tick {:?} {:?}", messenger_tick.round_index, messenger_tick.total_game_time_elapsed);
    }
}
