use core::time;
use std::thread::sleep;
use phf::phf_map;

use dotenvy::{self};

use train::Train;
use ws2818_rgb_led_spi_driver::{adapter_gen::WS28xxAdapter, adapter_spi::WS28xxSpiAdapter, encoding::encode_rgb};

mod train;
mod data_parser;

const OBA_ENV_VAR: &str = "ONEBUSAWAY_API_KEY";
pub const GET_1_LINE_URL: &str = "https://api.pugetsound.onebusaway.org/api/where/trips-for-route/40_100479.json?key=";
const MAX_LEDS: usize = 144;
const FRONT_BUF_OFFSET: usize = 4;

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
    // println!("{url_with_key}");
    let result = client.get(url_with_key)
        .send()
        .await?
        .text()
        .await?;
    
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), tokio::time::error::Error> {

    let client = reqwest::Client::new();
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
    let mut spi_encoded_rgb_bits = vec![];

    ctrlc::set_handler(move || {
        let mut close_adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
        let mut close_spi_encoded_rgb_bits = vec![];
        // turn off all LEDs
        close_spi_encoded_rgb_bits.clear();
        for _ in 0..MAX_LEDS {
            close_spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 0, 0));
        }
        close_adapter.write_encoded_rgb(&close_spi_encoded_rgb_bits).unwrap();
    }).expect("Failed to set ctrlc handler");

    loop {
    // for _ in 0..2 {
        if let Ok(json) = get_one_line(&client).await {
            // println!("{json}");
            let trains_result = data_parser::parse_from_string(&json);
            if let Ok(trains) = trains_result {
                let mut n_trains: Vec<Option<Train>> = vec![None; MAX_LEDS];
                let mut s_trains: Vec<Option<Train>> = vec![None; MAX_LEDS];

                // write initial leds
                println!("START BUFFER");
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 0, 25));
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 0, 25));
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 0, 25));

                for train in trains {
                    // println!("{:?}", train);
                    println!("trying to get idx for {:?}", train.next_stop_name.as_str());
                    let raw_idx = STN_NAME_TO_LED_IDX[train.next_stop_name.as_str()];
                    println!("raw_idx {:?}", raw_idx);
                    let idx = if train.at_station {
                        (raw_idx+FRONT_BUF_OFFSET)*2
                    } else {
                        (raw_idx+FRONT_BUF_OFFSET)*2 - 1
                    };
                    print!("idx is {:?} because train.at_station is {}, heading ", idx, train.at_station);

                    if train.direction() == Direction::N {
                        // n_trains.push(train);
                        n_trains[idx] = Some(train);
                        println!("North");
                    } else {
                        // s_trains.push(train);
                        s_trains[idx] = Some(train);
                        println!("South");
                    }
                }
                // println!("{:?}", n_trains);
                // println!("{:?}", s_trains);

                println!("NORTH TRAINS");
                for train_opt in n_trains {
                    if train_opt.is_some() {
                        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 25, 0));
                    } else {
                        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 0, 0));
                    }
                }

                // write mid buffer LEDs
                println!("MID BUFFER");
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(25, 0, 25));
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(25, 0, 25));
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(25, 0, 25));

                println!("SOUTH TRAINS");
                for train_opt in s_trains {
                    if train_opt.is_some() {
                        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(25, 0, 0));
                    } else {
                        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 0, 0));
                    }
                }

                // write end buffer LEDs
                println!("END BUFFER");
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(25, 25, 0));
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(25, 25, 0));
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(25, 25, 0));
                
                println!("trying to write {} bytes to SPI", spi_encoded_rgb_bits.len());
                // for chunk in spi_encoded_rgb_bits.chunks(4096) {
                //     adapter.write_encoded_rgb(&chunk).unwrap();
                // }
                adapter.write_encoded_rgb(&spi_encoded_rgb_bits).unwrap();
            } else {
                println!("json parse error 2")
            }
        } else {
            println!("json parse error 1");
        }
        sleep(time::Duration::from_secs(5));
        spi_encoded_rgb_bits.clear();
    }

    // Ok(())
}