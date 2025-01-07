use crate::{constants::Direction, env, error::Error, train};
use std::{collections::HashMap, time::Instant};
use log::{debug, info};
use serde_json::Value;

const GET_1_LINE_URL: &str = "https://api.pugetsound.onebusaway.org/api/where/trips-for-route/40_100479.json?key=";

pub async fn get_one_line_trains(client: &reqwest::Client) -> Result<Vec<train::Train>, Error> {
    match get_one_line(&client).await {
        Ok(json_string) => {
            match parse_1_line_json(&json_string) {
                Ok(trains) => Ok(trains),
                Err(e) => Err(Error::json_error(e)),
            }
        },
        Err(e) => {
            Err(Error::client_error(e))
        },
    }
}

async fn get_one_line(client: &reqwest::Client) -> Result<String, reqwest::Error> {
    let url_with_key = format!("{}{}", GET_1_LINE_URL, env::api_key());
    debug!("{}", url_with_key);
    let get_time = Instant::now();
    let result = client.get(url_with_key)
        .send()
        .await?
        .text()
        .await?;
    
    info!("get_one_line took {} seconds", get_time.elapsed().as_secs());
    Ok(result)
}

fn parse_1_line_json(json_string: &String) -> Result<Vec<train::Train>, serde_json::Error> {
    let json = serde_json::from_str::<Value>(json_string)?;
    let references = &json["data"]["references"];
    let stops_to_names = parse_stop_names(&references["stops"]);

    let trip_values = json["data"]["list"].as_array();
    let mut trains = vec![];
    if let Some(trips) = trip_values {
        for trip in trips {
            let id = trip["tripId"].as_str().unwrap();
            let name = stops_to_names.get(&trip["status"]["nextStop"].as_str().unwrap()).unwrap().to_string();
            let closest_stop_time_offset = trip["status"]["closestStopTimeOffset"].as_i64().unwrap();
            let at_station = closest_stop_time_offset == 0;
            if let Some(direction) = parse_trip_direction(&id, &json["data"]["references"]["trips"]) {
                trains.push(train::Train::new(name, direction, at_station));
            } else {
                debug!("id: {}, json: {}", id, json["data"]["references"]["trips"]);
                panic!("Couldn't parse direction");
            }
        }
    }

    Ok(trains)
}

// stop ID to stop name
fn parse_stop_names(json: &Value) -> HashMap<&str, &str> {
    let mut stop_map = HashMap::new();

    if let Some(stops) = json.as_array() {
        for stop in stops {
            stop_map.insert(stop["id"].as_str().unwrap(), stop["name"].as_str().unwrap());
        }
    }

    stop_map
}

fn parse_trip_direction(trip_id: &str, trips_json: &Value) -> Option<Direction> {
    if let Some(trips) = trips_json.as_array() {
        for trip in trips {
            let tmp_trip_id = trip["id"].as_str().unwrap();

            if trip_id == tmp_trip_id {
                debug!("trip: {:?}", trip);
                if trip["directionId"].as_str().unwrap() == "0" {
                    return Some(Direction::S);
                } else if trip["directionId"].as_str().unwrap() == "1" {
                    return Some(Direction::N);
                } else {
                    debug!("trip matched, direction failed.");
                }
            }
        }
    }
    debug!("id: {trip_id}");
    None
}