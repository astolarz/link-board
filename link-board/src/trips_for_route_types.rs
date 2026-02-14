use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct TripsForRoute {
    pub data: Data,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Data {
    pub list: Vec<TripDetails>,
    pub references: References,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct TripDetails {
    pub trip_id: String,
    pub status: Option<TripStatus>,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct TripStatus {
    pub scheduled_distance_along_trip: Option<f64>,
    pub next_stop: Option<String>,
    pub next_stop_time_offset: Option<i64>,
    pub closest_stop_time_offset: i64,
}

#[derive(Deserialize)]
#[serde(rename_all="snake_case")]
pub struct References {
    pub stops: Vec<Stop>,
    pub trips: Vec<Trip>
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Stop {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Trip {
    pub id: String,
    pub direction_id: Option<String>, 
}