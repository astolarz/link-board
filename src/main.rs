use core::time;
use std::{thread::sleep, time::Instant};
use led_adapter::{get_adapter, LedAdapter};
use phf::phf_map;
use log::{debug, error, info, warn};
use dotenvy::{self, dotenv};
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

mod train;
mod data_parser;
mod led_adapter;

const OBA_ENV_VAR: &str = "ONEBUSAWAY_API_KEY";
pub const GET_1_LINE_URL: &str = "https://api.pugetsound.onebusaway.org/api/where/trips-for-route/40_100479.json?key=";
const MAX_LEDS: usize = 144;
pub const LED_BUFFER: usize = 3;
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

fn api_key() -> String {
    let key = dotenvy::var(OBA_ENV_VAR);
    if key.is_err() {
        panic!("Failed to get API key!");
    }
    key.unwrap()
}

async fn get_one_line(client: &reqwest::Client) -> Result<String, reqwest::Error> {
    let url_with_key = format!("{}{}", GET_1_LINE_URL, api_key());
    debug!("{}", url_with_key);
    let result = client.get(url_with_key)
        .send()
        .await?
        .text()
        .await?;
    
    Ok(result)
}

fn start_buf_idx(idx: usize) -> usize {
    idx
}

fn mid_buf_idx(idx: usize) -> usize {
    LED_BUFFER + PIXELS_FOR_STATIONS + idx
}

fn end_buf_idx(idx: usize) -> usize {
    (LED_BUFFER * 3) + (PIXELS_FOR_STATIONS * 2) + idx
}

fn write_buffer_leds(led_strip: &mut Vec<(u8, u8, u8)>, buf_size: usize, idx_fn: fn(usize) -> usize, led_val: (u8, u8, u8)) -> usize {
    let mut count_written = 0;
    for i in 0..buf_size {
        led_strip[idx_fn(i)] = led_val;
        count_written += 1;
    }
    count_written
}

#[tokio::main]
async fn main() -> Result<(), tokio::time::error::Error> {
    dotenv().ok();
    simple_logger::init_with_env().unwrap();
    let now = Instant::now();
    info!("!!!starting!!!");

    let client = reqwest::Client::new();
    let mut adapter = get_adapter();
    let mut spi_encoded_rgb_bits = vec![];
    info!("!!!adapter running!!!");

    let mut i = 0;
    loop {
        info!("!!!loop starting!!!");
        info!("{:?} secs since main loop started.", now.elapsed().as_secs());
        info!("!!!main loop!!!");
        let res = get_one_line(&client).await;
        if res.is_err() {
            error!("Failed to get 1 Line data: {:?}", res);
        }
        if let Ok(json) = res {
            info!("it_{}: get_one_line took {} seconds", i,  now.elapsed().as_secs());
            i += 1;
            let trains_result = data_parser::parse_from_string(&json);
            if let Ok(trains) = trains_result {
                let mut led_strip: Vec<(u8, u8, u8)> = vec![(0, 0, 0); MAX_LEDS];
                let mut count = 0;

                // write initial leds
                info!("START BUFFER");
                count += write_buffer_leds(&mut led_strip, LED_BUFFER, start_buf_idx, START_BUF_LED);

                let mut total = 0;

                for train in trains {
                    total += 1;

                    let idx = if train.direction() == Direction::N {
                        train.get_idx()
                    } else {
                        LED_BUFFER + PIXELS_FOR_STATIONS + train.get_idx()
                    };

                    led_strip[idx] = train.get_led_rgb();
                }

                info!("{} total trains", total);

                // write mid buffer LEDs
                info!("MID BUFFER");
                count += write_buffer_leds(&mut led_strip, LED_BUFFER, mid_buf_idx, MID_BUF_LED);

                // write end buffer LEDs
                info!("END BUFFER");
                count += write_buffer_leds(&mut led_strip, LED_BUFFER, end_buf_idx, END_BUF_LED);
                info!("expecting {} leds", count);
                
                for led in led_strip {
                    spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(led.0, led.1, led.2));
                }
                adapter.write_encoded_rgb(&spi_encoded_rgb_bits).unwrap();
            } else {
                warn!("json parse error 2")
            }
        } else {
            warn!("json parse error 1");
        }
        info!("i_{} going to sleep after {} seconds", i, now.elapsed().as_secs());
        info!("!!!sleeping!!!");
        sleep(time::Duration::from_secs(15));
        spi_encoded_rgb_bits.clear();
        info!("!!!end main loop!!!");
    }
}