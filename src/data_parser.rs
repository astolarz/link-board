use crate::{constants::{Route, Terminus}, env, error::{Error, TripParseErr}, train};
use std::{collections::HashMap, time::Instant};
use log::{debug, info, warn};
use serde_json::Value;

const LINE_1_ROUTE_ID: &str = "40_100479";
const LINE_2_ROUTE_ID: &str = "40_2LINE";

pub async fn get_trains_for_route(client: &reqwest::Client, route: Route) -> Result<Vec<train::Train>, Error> {
    match get_route_json_string(&client, route).await {
        Ok(json_string) => {
            match parse_1_line_json(&json_string) {
                Ok(trains) => Ok(trains),
                Err(e) => Err(e),
            }
        },
        Err(e) => {
            Err(Error::client_error(e))
        },
    }
}

async fn get_route_json_string(client: &reqwest::Client, route: Route) -> Result<String, reqwest::Error> {
    let route_id = match route {
        Route::Line1 => LINE_1_ROUTE_ID,
        Route::Line2 => LINE_2_ROUTE_ID,
    };
    let url_with_key = format!(
        "https://api.pugetsound.onebusaway.org/api/where/trips-for-route/{}.json?key={}",
        route_id,
        env::api_key()
    );
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

fn parse_1_line_json(json_string: &String) -> Result<Vec<train::Train>, Error> {
    let json = serde_json::from_str::<Value>(json_string).map_err(|e| Error::json_error(e))?;
    let json_data = &json["data"];
    let references = &json_data["references"];
    let stops_to_names = parse_stop_names(&references["stops"]);

    let trip_values = json_data["list"].as_array();
    let mut trains = vec![];

    if let Some(trips) = trip_values {
        for trip in trips {
            match parse_trip(trip, &stops_to_names, references) {
                Ok(train) => trains.push(train),
                Err(_e) => {}
            };
        }
    }

    Ok(trains)
}

fn parse_trip(trip: &Value, stops_to_names: &HashMap<&str, &str>, references: &Value) -> Result<train::Train, Error> {
    let id = trip["tripId"].as_str()
        .ok_or_else(|| Error::trip_parse_error(TripParseErr::Id))?;

    let trip_status = &trip["status"];

    let next_stop = trip_status["nextStop"].as_str()
        .ok_or_else(|| Error::trip_parse_error(TripParseErr::NextStop))?;

    let name = stops_to_names.get(&next_stop)
        .ok_or_else(|| Error::trip_parse_error(TripParseErr::NextStop))?.to_string();

    let closest_stop_time_offset = trip_status["closestStopTimeOffset"].as_i64()
        .ok_or_else(|| Error::trip_parse_error(TripParseErr::ClosestStopTimeOffset))?;

    let at_station = closest_stop_time_offset == 0;

    match parse_trip_direction(&id, &references["trips"]) {
        Some(direction) => Ok(train::Train::new(name, direction, at_station)),
        None => Err(Error::trip_parse_error(TripParseErr::Direction)),
    }
}

// stop ID to stop name
fn parse_stop_names(json: &Value) -> HashMap<&str, &str> {
    let mut stop_map = HashMap::new();

    if let Some(stops) = json.as_array() {
        for stop in stops {
            let stop_id = stop["id"].as_str();
            let stop_name = stop["name"].as_str();

            if stop_id.is_some() && stop_name.is_some() {
                stop_map.insert(stop_id.unwrap(), stop_name.unwrap());
            } else {
                warn!("invalid stop id ({stop_id:?}) or name ({stop_name:?})");
            }
        }
    }

    stop_map
}

fn parse_trip_direction(trip_id: &str, trips_json: &Value) -> Option<Terminus> {
    if let Some(trips) = trips_json.as_array() {
        for trip in trips {
            if let Some(tmp_trip_id) = trip["id"].as_str() {
                if trip_id == tmp_trip_id {
                    debug!("trip: {:?}", trip);
                    
                    return dir_id_to_direction(trip["directionId"].as_str());
                }
            } else {
                warn!("invalid trip id");
            }
        }
    }
    warn!("no trips in trips_json");
    None
}

fn dir_id_to_direction(dir_id: Option<&str>) -> Option<Terminus> {
    // directionId can only be 0 or 1 per GTFS docs
    match dir_id {
        Some("0") => Some(Terminus::AngleLake),
        Some("1") => Some(Terminus::LynnwoodCC),
        _ => {
            warn!("invalid directionId");
            None
        },
    }
}