use dotenvy_macro::dotenv;

pub fn api_key() -> String {
    dotenv!("ONEBUSAWAY_API_KEY").to_string()
}

pub fn stations_only() -> bool {
    dotenv!("STATIONS_ONLY").parse().unwrap_or(false)
}

pub fn display_type_int() -> u8 {
    dotenv!("LINK_BOARD_DISPLAY_TYPE").parse::<u8>().unwrap_or(0)
}