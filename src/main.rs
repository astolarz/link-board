use core::time;
use std::thread::sleep;

use dotenvy::{self};
use serde_json::Value;

mod train;
mod data_parser;

const OBA_ENV_VAR: &str = "ONEBUSAWAY_API_KEY";
pub const GET_1_LINE_URL: &str = "https://api.pugetsound.onebusaway.org/api/wheretrips-for-route/40_100479.json?key=";

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

    loop {
        if let Ok(json) = get_one_line(&client).await {
            if let Ok(val ) = serde_json::from_str::<Value>(&json) {
                let trains_result = data_parser::parse(&val);
                if let Ok(trains) = trains_result {
                    let mut n_trains = vec![];
                    let mut s_trains = vec![];
                    for train in trains {
                        if train.direction() == Direction::N {
                            n_trains.push(train);
                        } else {
                            s_trains.push(train);
                        }
                    }
                }
            }
        }
        sleep(time::Duration::from_secs(5));
    }
}