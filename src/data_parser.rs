use crate::{constants::{Route, Destination}, env, error::{Error, TripParseErr}, train};
use std::{collections::HashMap, time::Instant};
use futures::{stream, StreamExt};
use log::{debug, info, warn};
use serde_json::Value;

const LINE_1_ROUTE_ID: &str = "40_100479";
const LINE_2_ROUTE_ID: &str = "40_2LINE";
const CONCURRENT_REQUESTS: usize = 2;

pub async fn get_all_trains() -> Result<Vec<train::Train>, Error> {
    let mut all_trains = vec![];
    let trains_json = get_json_for_all_trains().await?;

    for (route, json) in trains_json {
        let mut trains = parse_route_json(&json, route)?;
        all_trains.append(&mut trains);
    }

    Ok(all_trains)
}

async fn get_json_for_all_trains() -> Result<Vec<(Route, String)>, Error> {
    let routes = vec![Route::Line1, Route::Line2];
    let urls = routes.into_iter().map(|route| (route, url_for_route(route)));
    let mut results = Vec::with_capacity(urls.len());
    let client =  reqwest::Client::new();

    let fetches = stream::iter(
        urls.map(|(route, url)| {
            let mut results = vec![];
            let client = client.clone();
            async move {
                match client.get(&url).send().await {
                    Ok(response) => {
                        match response.text().await {
                            Ok(text) => {
                                debug!("retrieved text of len {} for route {:?}", text.len(), route);
                                results.push((route, text.to_owned()))
                            },
                            Err(e) => return Err(Error::client_error(e))
                        }
                    },
                    Err(e) => return Err(Error::client_error(e))
                }
                Ok(results)
            }
        })
    ).buffer_unordered(CONCURRENT_REQUESTS).collect::<Vec<Result<Vec<(Route, String)>, Error>>>();

    for result in fetches.await {
        match result {
            Ok(mut route_and_json) => results.append(&mut route_and_json),
            Err(e) => return Err(e),
        }
    }

    debug!("retrieved {} results", results.len());
    Ok(results)
}

#[allow(dead_code)]
pub async fn get_trains_for_route(client: &reqwest::Client, route: Route) -> Result<Vec<train::Train>, Error> {
    match get_route_json_string(&client, route).await {
        Ok(json_string) => {
            match parse_route_json(&json_string, route) {
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
    let url_with_key = url_for_route(route);
    let get_time = Instant::now();
    let result = client.get(url_with_key)
        .send()
        .await?
        .text()
        .await?;
    
    info!("get_one_line took {} seconds", get_time.elapsed().as_secs());
    Ok(result)
}

fn url_for_route(route: Route) -> String {
    let route_id = match route {
        Route::Line1 => LINE_1_ROUTE_ID,
        Route::Line2 => LINE_2_ROUTE_ID,
    };
    format!(
        "https://api.pugetsound.onebusaway.org/api/where/trips-for-route/{}.json?key={}",
        route_id,
        env::api_key()
    )
}

fn parse_route_json(json_string: &String, route: Route) -> Result<Vec<train::Train>, Error> {
    let json = serde_json::from_str::<Value>(json_string)?;
    let json_data = &json["data"];
    let references = &json_data["references"];
    let stops_to_names = parse_stop_names(&references["stops"]);

    let trip_values = json_data["list"].as_array();
    let mut trains = vec![];

    if let Some(trips) = trip_values {
        for trip in trips {
            match parse_trip(trip, &stops_to_names, references, route) {
                Ok(train) => trains.push(train),
                Err(_e) => {}
            };
        }
    }

    Ok(trains)
}

fn parse_trip(trip: &Value, stops_to_names: &HashMap<&str, &str>, references: &Value, route: Route) -> Result<train::Train, Error> {
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

    match parse_trip_destination(&id, &references["trips"]) {
        Some(destination) => Ok(train::Train::new(name, route, destination, at_station)),
        None => Err(Error::trip_parse_error(TripParseErr::Destination)),
    }
}

// stop ID to stop name
fn parse_stop_names(json: &Value) -> HashMap<&str, &str> {
    let mut stop_map = HashMap::new();

    if let Some(stops) = json.as_array() {
        for stop in stops {
            let stop_id = stop["id"].as_str().unwrap_or("");
            let stop_name = stop["name"].as_str().unwrap_or("");

            if !stop_id.is_empty() && !stop_name.is_empty() {
                stop_map.insert(stop_id, stop_name);
            } else {
                warn!("invalid stop id ({stop_id:?}) or name ({stop_name:?})");
            }
        }
    }

    stop_map
}

fn parse_trip_destination(trip_id: &str, trips_json: &Value) -> Option<Destination> {
    if let Some(trips) = trips_json.as_array() {
        for trip in trips {
            if let Some(tmp_trip_id) = trip["id"].as_str() {
                if trip_id == tmp_trip_id {
                    return dir_id_to_destination(trip["directionId"].as_str());
                }
            } else {
                warn!("invalid trip id");
            }
        }
    }
    warn!("no trips in trips_json");
    None
}

fn dir_id_to_destination(dir_id: Option<&str>) -> Option<Destination> {
    // directionId can only be 0 or 1 per GTFS docs
    match dir_id {
        Some("0") => Some(Destination::AngleLake),
        Some("1") => Some(Destination::LynnwoodCC),
        _ => {
            warn!("invalid directionId");
            None
        },
    }
}