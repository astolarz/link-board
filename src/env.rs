use crate::constants;

const OBA_ENV_VAR: &str = "ONEBUSAWAY_API_KEY";
const STATIONS_ONLY_VAR: &str = "STATIONS_ONLY";

pub fn api_key() -> String {
    let key = dotenvy::var(OBA_ENV_VAR);
    if key.is_err() {
        panic!("Failed to get API key!");
    }
    key.unwrap()
}

pub fn stations_only() -> bool {
    if let Ok(stations_only) = dotenvy::var(STATIONS_ONLY_VAR) {
        if stations_only == "true" {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn px_for_stns() -> usize {
    constants::PIXELS_FOR_STATIONS
    // TODO: figure out how to make these const/static/or whatever the rustic way is
    // if stations_only() {
    //     constants::PIXELS_FOR_STATIONS_ONLY
    // } else {
    //     constants::PIXELS_FOR_STATIONS_EXPANDED
    // }
}