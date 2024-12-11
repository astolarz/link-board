use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Duration, Instant}};
use led_adapter::{get_adapter, LedAdapter};
use phf::phf_map;
use log::{error, info, warn};
use dotenvy::{self, dotenv};

mod train;
mod data_parser;
mod led_adapter;

const MAX_LEDS: usize = 144;
pub const LED_BUFFER_COUNT: usize = 3;
const START_BUF_LED: (u8, u8, u8) = (0, 0, 25);
const MID_BUF_LED: (u8, u8, u8) = (25, 0, 0);
const END_BUF_LED: (u8, u8, u8) = (25, 25, 0);

pub static STN_NAME_TO_LED_IDX:  phf::Map<&'static str, usize> = phf_map! {
    "Angle Lake" => 0,
    "SeaTac/Airport"=> 1,
    "Tukwila Int'l Blvd"=> 2,
    "Rainier Beach"=> 3,
    "Othello"=> 4,
    "Columbia City"=> 5,
    "Mount Baker"=> 6,
    "Beacon Hill"=> 7,
    "SODO"=> 8,
    "Stadium"=> 9,
    "Int'l Dist/Chinatown"=> 10,
    "Pioneer Square"=> 11,
    "Symphony"=> 12,
    "Westlake"=> 13,
    "Capitol Hill"=> 14,
    "Univ of Washington"=> 15,
    "U District"=> 16,
    "Roosevelt"=> 17,
    "Northgate"=> 18,
    "Shoreline South/148th"=> 19,
    "Shoreline North/185th"=> 20,
    "Mountlake Terrace"=> 21,
    "Lynnwood City Center"=> 22
};
// size of station map * 2 for one LED in between, plus one more for beginning buffer.
static PIXELS_FOR_STATIONS: usize = STN_NAME_TO_LED_IDX.len()*2 + 1;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    N,
    S,
    E, // for 2 Line
    W, // for 2 Line
}

fn start_buf_init_idx() -> usize {
    0
}

fn north_train_init_idx() -> usize {
    start_buf_init_idx() + LED_BUFFER_COUNT
}

fn mid_buf_init_idx() -> usize {
    north_train_init_idx() + PIXELS_FOR_STATIONS
}

fn south_train_init_idx() -> usize {
    mid_buf_init_idx() + LED_BUFFER_COUNT
}

fn end_buf_init_idx() -> usize {
    south_train_init_idx() + PIXELS_FOR_STATIONS
}

fn prepare_buffer_leds(led_strip: &mut Vec<(u8, u8, u8)>, init_idx_fn: fn() -> usize, led_val: (u8, u8, u8)) -> usize {
    let mut count_written = 0;
    for i in 0..LED_BUFFER_COUNT {
        let idx = init_idx_fn() + i;
        if led_strip[idx] != (0, 0, 0) {
            warn!("multiple trains at index [{}]", idx);
        }
        led_strip[idx] = led_val;
        info!("placing buffer at index [{}]", idx);
        count_written += 1;
    }
    count_written
}

fn index_trains(led_strip: &mut Vec<(u8, u8, u8)>, trains: Vec<train::Train>) -> usize {
    let mut total = 0;

    for train in trains {
        total += 1;

        let idx = if train.direction() == Direction::N {
            north_train_init_idx() + train.get_relative_idx()
        } else {
            south_train_init_idx() + train.get_relative_idx()
        };
        info!("placing train at index [{}]", idx);

        led_strip[idx] = train.get_led_rgb();
    }

    info!("{} total trains", total);
    total
}

#[tokio::main]
async fn main() -> Result<(), tokio::time::error::Error> {
    dotenv().ok();
    simple_logger::init_with_env().unwrap();

    let prog_start = Instant::now();
    info!("!!!starting!!!");

    let client = reqwest::Client::new();
    let mut adapter = get_adapter();
    info!("!!!adapter running!!!");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("ctrl-c interrupt");
        r.store(false, Ordering::SeqCst);
    });

    let mut i = 0;
    let mut wait_time = Instant::now() - Duration::new(15, 0);
    while running.load(Ordering::SeqCst) {
        if wait_time.elapsed().as_secs() < 15 {
            continue;
        } else {
            wait_time = Instant::now();
        }

        let loop_time = Instant::now();

        info!("!!!main loop starting!!!");
        info!("{:?} secs since main loop started.", prog_start.elapsed().as_secs());

        let res = data_parser::get_one_line(&client).await;
        if res.is_err() {
            error!("Failed to get 1 Line data: {:?}", res);
        }
        if let Ok(json) = res {
            info!("it_{}: get_one_line took {} seconds", i,  loop_time.elapsed().as_secs());
            i += 1;
            let trains_result = data_parser::parse_from_string(&json);
            if let Ok(trains) = trains_result {
                let mut led_strip: Vec<(u8, u8, u8)> = vec![(0, 0, 0); MAX_LEDS];
                let mut count = 0;

                // write initial leds
                info!("START BUFFER");
                count += prepare_buffer_leds(&mut led_strip, start_buf_init_idx, START_BUF_LED);

                count += index_trains(&mut led_strip, trains);

                // write mid buffer LEDs
                info!("MID BUFFER");
                count += prepare_buffer_leds(&mut led_strip, mid_buf_init_idx, MID_BUF_LED);

                // write end buffer LEDs
                info!("END BUFFER");
                count += prepare_buffer_leds(&mut led_strip, end_buf_init_idx, END_BUF_LED);
                info!("expecting {} leds", count);
                
                adapter.write_rgb(led_strip).unwrap();
            } else {
                warn!("json parse error 2")
            }
        } else {
            warn!("json parse error 1");
        }
        info!("i_{} going to sleep after {} seconds", i, loop_time.elapsed().as_secs());
        info!("!!!end main loop!!!");
    }

    info!("clearing LED strip");
    adapter.clear().unwrap();

    info!("exiting");
    Ok(())
}