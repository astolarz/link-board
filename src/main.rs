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
const LED_BUFFER: usize = 3;

const AT_STATION: (u8, u8, u8) = (0, 25, 0);
const BTW_STATION: (u8, u8, u8) = (5, 5, 0);

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
            warn!("!!!!!!JSON ERROR!!!!!!");
            error!("{:?}", res);
            warn!("!!!!!!JSON ERROR!!!!!!");
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
                for i in 0..LED_BUFFER {
                    led_strip[i] = (0, 0, 25);
                }
                count += LED_BUFFER;

                let mut total = 0;

                for train in trains {
                    total += 1;
                    debug!("trying to get idx for {:?}", train.next_stop_name.as_str());
                    let raw_idx = STN_NAME_TO_LED_IDX[train.next_stop_name.as_str()];
                    debug!("raw_idx {:?}", raw_idx);
                    let idx = if train.at_station {
                        (raw_idx+LED_BUFFER)*2
                    } else {
                        (raw_idx+LED_BUFFER)*2 - 1
                    };
                    debug!("idx is {:?} because train.at_station is {}, heading ", idx, train.at_station);

                    if train.direction() == Direction::N {
                        led_strip[idx] = if train.at_station {
                            AT_STATION
                        } else {
                            BTW_STATION
                        };
                        debug!("North");
                    } else {
                        led_strip[LED_BUFFER + PIXELS_FOR_STATIONS + idx] = if train.at_station {
                            AT_STATION
                        } else {
                            BTW_STATION
                        };
                        debug!("South");
                    }
                }

                info!("{} total trains", total);

                // write mid buffer LEDs
                info!("MID BUFFER");
                for i in 0..LED_BUFFER {
                    led_strip[LED_BUFFER + PIXELS_FOR_STATIONS + i] = (25, 0, 0);
                }
                count += LED_BUFFER;

                // write end buffer LEDs
                info!("END BUFFER");
                for i in 0..LED_BUFFER {
                    let idx = (LED_BUFFER * 3) + (PIXELS_FOR_STATIONS * 2) + i;
                    led_strip[idx] = (25, 25, 0);
                }
                count += LED_BUFFER;
                info!("expecting {} leds", count);
                
                for led in led_strip {
                    spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(led.0, led.1, led.2));
                }
                adapter.write_encoded_rgb(&spi_encoded_rgb_bits).unwrap();
            } else {
                info!("json parse error 2")
            }
        } else {
            info!("json parse error 1");
        }
        info!("i_{} going to sleep after {} seconds", i, now.elapsed().as_secs());
        info!("!!!sleeping!!!");
        sleep(time::Duration::from_secs(15));
        spi_encoded_rgb_bits.clear();
        info!("!!!end main loop!!!");
    }
}