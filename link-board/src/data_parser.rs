use crate::{constants::Destination, data_retriever::DataRetriever, display::Route, error::{Error, TripParseErr}, train::Train};
use std::{collections::HashMap, mem};
use log::{info, warn};
use serde_json::Value;

pub async fn get_all_trains(data_retriever: &impl DataRetriever) -> Result<Vec<Train>, Error> {
    let mut all_trains = vec![];
    let trains_json = data_retriever.get_json_for_all_trains().await?;

    for (route, json) in trains_json {
        let mut trains = parse_route(&json, route)?;
        all_trains.append(&mut trains);
    }

    Ok(all_trains)
}

fn parse_route(json_string: &String, route: Route) -> Result<Vec<Train>, Error> {
    info!("json_string.len(): {}", json_string.len());
    let json = serde_json::from_str::<Value>(json_string)?;
    info!("json: {}", mem::size_of_val(&json));
    let json_data = &json["data"];
    info!("json_data: {}", mem::size_of_val(&json_data));
    let references = &json_data["references"];
    info!("references: {}", mem::size_of_val(&references));
    info!("parsing stop names");
    let stops_to_names = parse_stop_names(&references["stops"]);
    info!("stops_to_len: {}", stops_to_names.len());
    let trip_ref_values = references["trips"].as_array();
    info!("trip_refs: {}", mem::size_of_val(&trip_ref_values));

    let mut trains = vec![];

    if let Some(trip_ref_values) = trip_ref_values {
        info!("found {} trip references for route {:?}", trip_ref_values.len(), route);
        
        if let Ok(trip_ids_to_dests) = parse_trips_from_refs(trip_ref_values, route) {
            if let Some(trip_values) = json_data["list"].as_array() {
                info!("found {} tripDetails for route {:?}", trip_values.len(), route);

                for trip_value in trip_values {
                    match parse_train_from_trip_details(trip_value, &trip_ids_to_dests, &stops_to_names, route) {
                        Ok(train) => trains.push(train),
                        Err(e) => {
                            if !e.is_not_in_progress_err() {
                                return Err(e);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(trains)
}

fn parse_trips_from_refs(trip_values: &Vec<Value>, route: Route) -> Result<HashMap<&str, Destination>, Error> {
    let mut trips_map = HashMap::new();

    for trip in trip_values {
        let id = trip["id"].as_str()
            .ok_or_else(|| Error::trip_parse_error(TripParseErr::Id))?;
        if let Some(destination) = dir_id_to_destination(trip["directionId"].as_str(), route) {
            trips_map.insert(id, destination);
        } else {
            warn!("unable to parse directionId");
        }
    }

    Ok(trips_map)
}

fn parse_train_from_trip_details(
        trip_detail_value: &Value,
        trip_ids_to_dests: &HashMap<&str, Destination>,
        stops_to_names: &HashMap<&str, &str>,
        route: Route
    ) -> Result<Train, Error> {

    let id = trip_detail_value["tripId"].as_str()
        .ok_or_else(|| Error::trip_parse_error(TripParseErr::Id))?;

    let trip_status = &trip_detail_value["status"];

    // If scheduledDistanceAlongTrip is 0 or not present, then the trip is not currently in progress.
    if let Some(sched_dist) = trip_status["scheduledDistanceAlongTrip"].as_f64() {
        if sched_dist == 0.0 {
            warn!("trip {} not in progress yet on route {:?}, scheduledDistanceAlongTrip: {}", id, route, sched_dist);
            return Err(Error::trip_parse_error(TripParseErr::NotInProgress));
        }
    } else {
        warn!("trip {} not in progress yet on route {:?}, no scheduledDistanceAlongTrip", id, route);
        return Err(Error::trip_parse_error(TripParseErr::NotInProgress));
    }

    let destination = trip_ids_to_dests[id];

    let next_stop_id = trip_status["nextStop"].as_str()
        .ok_or_else(|| Error::trip_parse_error(TripParseErr::BeyondLastStop))?;

    let next_stop_name = stops_to_names.get(&next_stop_id)
        .ok_or_else(|| Error::trip_parse_error(TripParseErr::NextStop))?.to_string();

    let next_stop_time_offset = trip_status["nextStopTimeOffset"].as_i64()
        .ok_or_else(|| Error::trip_parse_error(TripParseErr::BeyondLastStop))?;

    let closest_stop_time_offset = trip_status["closestStopTimeOffset"].as_i64()
        .ok_or_else(|| Error::trip_parse_error(TripParseErr::ClosestStopTimeOffset))?;

    Ok(Train::new(
        next_stop_name,
        route,
        destination,
        next_stop_time_offset,
        closest_stop_time_offset
    ))
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

fn dir_id_to_destination(dir_id: Option<&str>, route: Route) -> Option<Destination> {
    // directionId can only be 0 or 1 per GTFS docs
    match dir_id {
        Some("0") => match route {
            Route::Line1 => Some(Destination::AngleLake),
            Route::Line2 => Some(Destination::RedmondTech),
        },
        Some("1") => match route {
            Route::Line1 => Some(Destination::LynnwoodCC),
            Route::Line2 => Some(Destination::SouthBellevue),
        },
        _ => {
            warn!("invalid directionId");
            None
        },
    }
}