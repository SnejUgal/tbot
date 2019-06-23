use std::time::{Duration, Instant};
use tbot::{prelude::*, types::chat};
use tokio::{prelude::*, timer::Interval};

const CHAT: chat::Id = chat::Id(0);
const INTERVAL: u64 = 15;
const PLACES: [(f64, f64); 6] = [
    (38.904722, -77.016389), // Washington
    (51.507222, -0.1275),    // London
    (41.9, 12.5),            // Rome
    (59.329444, 18.068611),  // Stockholm
    (55.796389, 49.108889),  // Kazan
    (56.5, 84.966667),       // Tomsk
];
const UPDATE_PERIOD: u32 = 3600 * 24;

fn main() {
    let bot = tbot::bot!("BOT_TOKEN");
    let mut places = PLACES.iter().cycle();

    let live_location = bot
        .send_location(CHAT, *places.next().unwrap())
        .live_period(UPDATE_PERIOD)
        .into_future()
        .map_err(|err| {
            dbg!(err);
        })
        .and_then(|message| {
            Interval::new(Instant::now(), Duration::from_secs(INTERVAL))
                .skip(1)
                .zip(futures::stream::iter_ok(places))
                .map_err(|err| {
                    dbg!(err);
                })
                .for_each(move |(_, place)| {
                    bot.edit_message_location(
                        message.chat.id,
                        message.id,
                        *place,
                    )
                    .into_future()
                    .map_err(|err| {
                        dbg!(err);
                    })
                    .map(|_| ())
                })
        })
        .timeout(Duration::from_secs(UPDATE_PERIOD as u64))
        .map_err(|_| ()); // expected to timeout

    tbot::run(live_location);
}
