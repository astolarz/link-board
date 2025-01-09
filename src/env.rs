const OBA_ENV_VAR: &str = "ONEBUSAWAY_API_KEY";
const STATIONS_ONLY_VAR: &str = "STATIONS_ONLY";
const DISPLAY_TYPE_VAR: &str = "LINK_BOARD_DISPLAY_TYPE";

pub fn api_key() -> String {
    let key = dotenvy::var(OBA_ENV_VAR);
    if key.is_err() {
        panic!("Failed to get API key!");
    }
    key.unwrap()
}

pub fn stations_only() -> bool {
    match dotenvy::var(STATIONS_ONLY_VAR) {
        Ok(stations_only) => stations_only.parse().unwrap_or(false),
        Err(_) => false,
    }
}

pub fn display_type_string() -> String {
    match dotenvy::var(DISPLAY_TYPE_VAR) {
        Ok(disp_type_string) => disp_type_string,
        Err(_) => String::from("0"),
    }
}