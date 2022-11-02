use tokio::sync::mpsc::Sender;
use tokio::time;
use std::time::Duration;

const ROUND_INTERVAL: Duration = Duration::from_millis(100);

pub struct Tick {
    pub index: u32,
    pub time_elapsed: Duration,
}

pub struct Ticker {}

impl Ticker {
    pub fn create(ticks_sender: Sender<Tick>) -> Ticker {
        let ticker = Ticker {};

        tokio::spawn(Ticker::listen(ticks_sender));

        ticker
    }

    async fn listen(ticks_sender: Sender<Tick>)
    {
        let mut index = 0;
        let now = time::Instant::now();
        loop {
            time::sleep(ROUND_INTERVAL).await;
            index = index + 1;
            let time_elapsed = now.elapsed();
            ticks_sender.send(Tick { index, time_elapsed }).await
                .map_err(|err| log::warn!("failed to publish tick {}", err)).ok();
        }
    }
}
