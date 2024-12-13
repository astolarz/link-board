use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Duration, Instant}};
use led_adapter::{get_adapter, LedAdapter};
use phf::phf_map;
use log::{error, info, warn};
use dotenvy::{self, dotenv};
use colored::Colorize;

mod train;
mod data_parser;
mod led_adapter;

const MAX_LEDS_FOR_STRIP: usize = 144;
const LED_BUFFER_COUNT: usize = 3;
const LED_OFF: (u8, u8, u8) = (0, 0, 0,);
const START_BUF_LED: (u8, u8, u8) = (255, 0, 0);
const MID_BUF_LED: (u8, u8, u8) = (255, 165, 0);
const END_BUF_LED: (u8, u8, u8) = (0, 0, 255);
const STAGING_LED: (u8, u8, u8) = (255, 0, 255);

const STN_NAME_TO_LED_IDX:  phf::Map<&'static str, usize> = phf_map! {
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
const PIXELS_FOR_STATIONS: usize = (STN_NAME_TO_LED_IDX.len() * 2) - 1;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
enum Direction {
    N,
    S,
    E, // for 2 Line
    W, // for 2 Line
}


// First three LEDs are start buffer (red).
//
// Next 45 LEDs are for northbound trains, starting with Angle Lake station,
// second to last is Lynnwood City Center, last is staging (purple) for train
// about to return south.
//
// Three LEDs for mid buffer (orange).
//
// Next 45 LEDs are for southbound trains, starting with staging (purple) for
// train about to return north, then Angle Lake, ending with Lynnwood City Center.
//
// Three LEDs for end buffer (blue).
//
// For north and southbound train sections, LEDs alternate between at station
// (green), and in between stations (yellow). If more than one train is in one
// of those sections, 10 blue is added, making at-station LEDs more teal, and in
// between stations more purple.
//
// Both north and southbound sections have Angle Lake (southernmost station) on
// the left, and Lynwood City Center (northernmost station) on the right.
const START_BUF_INIT_IDX: usize = 0;

const NORTH_TRAIN_INIT_IDX: usize = START_BUF_INIT_IDX + LED_BUFFER_COUNT;
// Lynnwood City Center + 1
const NORTH_TRAIN_STAGING_IDX: usize = NORTH_TRAIN_INIT_IDX + PIXELS_FOR_STATIONS;

const MID_BUF_INIT_IDX: usize = NORTH_TRAIN_STAGING_IDX + 1;

// Angle Lake - 1
const SOUTH_TRAIN_STAGING_IDX: usize = MID_BUF_INIT_IDX + LED_BUFFER_COUNT;
const SOUTH_TRAIN_INIT_IDX: usize = SOUTH_TRAIN_STAGING_IDX + 1;

const END_BUF_INIT_IDX: usize = SOUTH_TRAIN_INIT_IDX + PIXELS_FOR_STATIONS;

const MAX_LEDS_NEEDED: usize = END_BUF_INIT_IDX + LED_BUFFER_COUNT;

fn prepare_buffer_leds(led_strip: &mut Vec<(u8, u8, u8)>, init_idx: usize, led_val: (u8, u8, u8)) -> usize {
    let mut count_written = 0;
    for i in 0..LED_BUFFER_COUNT {
        let idx = init_idx + i;
        if led_strip[idx] != LED_OFF {
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
            NORTH_TRAIN_INIT_IDX + train.get_relative_idx()
        } else {
            SOUTH_TRAIN_INIT_IDX + train.get_relative_idx()
        };

        let current_color = led_strip[idx];
        let final_color = if idx == NORTH_TRAIN_STAGING_IDX || idx == SOUTH_TRAIN_STAGING_IDX {
            STAGING_LED
        } else {
            let mut new_color = train.get_led_rgb();
            if current_color == LED_OFF {
                new_color
            } else {
                new_color.2 += 10;
                new_color
            }
        };
        led_strip[idx] = final_color;

        let colorized_dir = if train.direction() == Direction::N {
            "(N)".red()
        } else {
            "(S)".blue()
        };
        info!("placing {} {} at index [{:3}]; next stop: {}", 
            colorized_dir,
            "train".truecolor(final_color.0, final_color.1, final_color.2),
            idx,
            train.next_stop_name);
    }

    info!("{} total trains", total);
    total
}

#[tokio::main]
async fn main() -> Result<(), tokio::time::error::Error> {
    assert!(MAX_LEDS_NEEDED <= MAX_LEDS_FOR_STRIP);
    dotenv().ok();
    simple_logger::init_with_env().unwrap();

    let prog_start = Instant::now();

    let client = reqwest::Client::new();
    let mut adapter = get_adapter();

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
                let mut led_strip: Vec<(u8, u8, u8)> = vec![LED_OFF; MAX_LEDS_NEEDED];
                let mut count = 0;

                // write initial leds
                info!("START BUFFER");
                count += prepare_buffer_leds(&mut led_strip, START_BUF_INIT_IDX, START_BUF_LED);

                count += index_trains(&mut led_strip, trains);

                // write mid buffer LEDs
                info!("MID BUFFER");
                count += prepare_buffer_leds(&mut led_strip, MID_BUF_INIT_IDX, MID_BUF_LED);

                // write end buffer LEDs
                info!("END BUFFER");
                count += prepare_buffer_leds(&mut led_strip, END_BUF_INIT_IDX, END_BUF_LED);
                info!("expecting {} leds", count);
                
                adapter.write_rgb(led_strip).unwrap();
            } else {
                warn!("json parse error 2")
            }
        } else {
            warn!("json parse error 1");
        }
        info!("i_{} going to sleep after {} seconds", i, loop_time.elapsed().as_secs());
    }

    info!("clearing LED strip");
    adapter.clear().unwrap();

    info!("exiting");
    Ok(())
}