use crate::{constants::Destination, data_retriever::DataRetriever, display::Route, error::Error, train::Train, trips_for_route_types::TripsForRoute};
use std::collections::HashMap;
use log::{info, warn};

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
    let mut trains = vec![];
    let trips_for_route: TripsForRoute = serde_json::from_str(json_string)?;
    info!("successfully parsed trips for route");

    let mut trip_ids_to_dests = HashMap::new();
    for trip in trips_for_route.data.references.trips {
        if let Some(direction_id) = dir_id_to_destination(trip.direction_id.as_deref(), route) {
            trip_ids_to_dests.insert(trip.id,direction_id);
        } else {
            warn!("no directionId for trip {}", trip.id);
        }
    }

    let mut stops_to_names = HashMap::new();
    for stop in trips_for_route.data.references.stops {
        stops_to_names.insert(stop.id, stop.name);
    }
    
    for trip in trips_for_route.data.list {
        if trip.status.next_stop.is_none() || trip.status.next_stop_time_offset.is_none() {
            continue;
        }
        
        if let Some(sched_dist) = trip.status.scheduled_distance_along_trip {
            if sched_dist == 0.0 {
                warn!("trip {} not in progress yet on route {:?}, scheduledDistanceAlongTrip: {}", trip.trip_id, route, sched_dist);
            }
        } else {
            warn!("trip {} not in progress yet on route {:?}, no scheduledDistanceAlongTrip", trip.trip_id, route);
        }

        trains.push(Train::new(
            stops_to_names[&trip.status.next_stop.unwrap()].clone(),
            route,
            trip_ids_to_dests[&trip.trip_id],
            trip.status.next_stop_time_offset.unwrap(),
            trip.status.closest_stop_time_offset
        ));
    }

    Ok(trains)
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