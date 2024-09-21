use std::collections::HashMap;

use serde_json::Value;

use crate::{train, Direction};

fn parse_1_line_json(json: &Value) -> Result<Vec<train::Train>, serde_json::Error> {
    let references = &json["data"]["references"];
    let trips_to_dirs = parse_trip_directions(&references["trips"]);
    let stops_to_names = parse_stop_names(&references["stops"]);

    let trip_values = json["data"]["list"].as_array();
    let mut trains = vec![];
    if let Some(trips) = trip_values {
        for trip in trips {
            let id = json["tripId"].as_u64().unwrap_or(0) as u16;
            let next_stop = trip["status"]["nextStop"].to_string();
            let name = stops_to_names.get(&trip["status"]["nextStop"].to_string()).unwrap().to_string();
            let direction = *trips_to_dirs.get(&trip["tripId"].to_string()).unwrap();
            trains.push(train::Train::new(id, next_stop, name, direction));
        }
    }

    Ok(trains)
}

// stop ID to stop name
fn parse_stop_names(json: &Value) -> HashMap<String, String> {
    let mut stop_map = HashMap::new();

    if let Some(stops) = json.as_array() {
        for stop in stops {
            stop_map.insert(stop["id"].to_string(), stop["name"].to_string());
        }
    }

    stop_map
}

// trip ID to direction
fn parse_trip_directions(trips_json: &Value) -> HashMap<String, Direction> {
    let mut trip_dirs = HashMap::new();
    if let Some(trips) = trips_json.as_array() {
        for trip in trips {
            trip_dirs.insert(trip["id"].to_string(),
                if trip["directionId"].to_string() == "0" {
                    Direction::S
                } else {
                    Direction::N
                });
        }
    }
    trip_dirs
}

pub fn parse(json: &Value) -> Result<Vec<train::Train>, serde_json::Error> {
    Ok(parse_1_line_json(&json)?)
}