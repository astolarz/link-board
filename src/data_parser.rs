use crate::{constants::Destination, data_retriever::DataRetriever, display::Route, error::Error, train::Train, trips_for_route_types::TripsForRoute};
use std::collections::HashMap;
use log::{info, warn};

pub async fn get_all_trains(data_retriever: &mut impl DataRetriever) -> Result<Vec<Train>, Error> {
    let routes = vec![Route::Line1, Route::Line2];
    let mut trains = vec![];

    for route in routes {
        parse_route(route, data_retriever, &mut trains).await?;
    }

    Ok(trains)
}

async fn parse_route(route: Route, data_retriever: &mut impl DataRetriever, trains: &mut Vec<Train>) -> Result<(), Error> {
    let json_string = data_retriever.get_json_for_route(route).await?;

    let trips_for_route: TripsForRoute = serde_json::from_str(&json_string)?;
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
        let Some(status) = trip.status else {
            warn!("status missing for {}", trip.trip_id);
            continue;
        };
        let Some(next_stop) = status.next_stop else {
            warn!("no next stop for {}", trip.trip_id);
            continue;
        };
        let Some(next_stop_time_offset) = status.next_stop_time_offset else {
            warn!("no next stop time offset for {}", trip.trip_id);
            continue;
        };
        
        if let Some(sched_dist) = status.scheduled_distance_along_trip {
            if sched_dist == 0.0 {
                warn!("trip {} not in progress yet on route {:?}, scheduledDistanceAlongTrip: {}", trip.trip_id, route, sched_dist);
            }
        } else {
            warn!("trip {} not in progress yet on route {:?}, no scheduledDistanceAlongTrip", trip.trip_id, route);
        }

        trains.push(Train::new(
            stops_to_names[&next_stop].clone(),
            route,
            trip_ids_to_dests[&trip.trip_id],
            next_stop_time_offset,
            status.closest_stop_time_offset
        ));
    }

    Ok(())
}

fn dir_id_to_destination(dir_id: Option<&str>, route: Route) -> Option<Destination> {
    // directionId can only be 0 or 1 per GTFS docs
    match dir_id {
        Some("0") => match route {
            Route::Line1 => Some(Destination::FederalWayDT),
            Route::Line2 => Some(Destination::RedmondDT),
        },
        Some("1") => Some(Destination::LynnwoodCC),
        _ => {
            warn!("invalid directionId");
            None
        },
    }
}