use tokio::sync::mpsc;
use clap::{App, Arg};

use webrtc_unreliable::Server as RtcServer;
use webrtc_server::comms::commands_collector::CommandsCollector;
use webrtc_server::comms::messenger::Messenger;
use webrtc_server::comms::ticker::Ticker;

use webrtc_server::html::html_server;

const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 100;

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

    let (received_messages_sender, messages_receiver)
        = mpsc::channel(DEFAULT_CHANNEL_BUFFER_SIZE);
    let (messages_to_send_sender, messages_to_send_receiver)
        = mpsc::channel(DEFAULT_CHANNEL_BUFFER_SIZE);
    Messenger::create(rtc_server, received_messages_sender, messages_to_send_receiver);


    let (ticks_sender, ticks_receiver)
        = mpsc::channel(DEFAULT_CHANNEL_BUFFER_SIZE);
    let (collected_commands_sender, mut collected_commands_receiver)
        = mpsc::channel(DEFAULT_CHANNEL_BUFFER_SIZE);
    CommandsCollector::create(
        messages_receiver, ticks_receiver, collected_commands_sender);

    Ticker::create(ticks_sender);

    loop {
        let commands = collected_commands_receiver.recv().await.unwrap();
        dbg!(commands.len());
        for (_, message) in commands {
            messages_to_send_sender.send(message).await
                .map_err(|err| log::warn!("failed to publish tick {}", err))
                .ok();
        }
    }
}
