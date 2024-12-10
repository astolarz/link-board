use std::collections::HashMap;
use log::debug;

use serde_json::Value;

use crate::{train, Direction};

fn parse_1_line_json(json: &Value) -> Result<Vec<train::Train>, serde_json::Error> {
    let references = &json["data"]["references"];
    let stops_to_names = parse_stop_names(&references["stops"]);

    let trip_values = json["data"]["list"].as_array();
    let mut trains = vec![];
    if let Some(trips) = trip_values {
        for trip in trips {
            debug!("{}", trip["tripId"]);
            let id = trip["tripId"].as_str().unwrap();
            // let next_stop = trip["status"]["nextStop"].as_str().unwrap_or("").to_string();
            let name = stops_to_names.get(&trip["status"]["nextStop"].as_str().unwrap()).unwrap().to_string();
            let closest_stop_time_offset = trip["status"]["closestStopTimeOffset"].as_i64().unwrap();
            let at_station = closest_stop_time_offset == 0;
            if let Some(direction) = parse_trip_direction(&id, &json["data"]["references"]["trips"]) {
                trains.push(train::Train::new(/*id.to_string(), next_stop,*/ name, direction, at_station));
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
            debug!("{:?}\n{:?}\n", trip_id, tmp_trip_id);
            if trip_id == tmp_trip_id {
                debug!("{:?}", trip);
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

pub fn parse(json: &Value) -> Result<Vec<train::Train>, serde_json::Error> {
    Ok(parse_1_line_json(&json)?)
}

pub fn parse_from_string(json: &String) -> Result<Vec<train::Train>, serde_json::Error> {
    let json_val = serde_json::from_str::<Value>(json)?;
    parse(&json_val)
}