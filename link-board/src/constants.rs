use crate::led::Led;
use phf::phf_map;

pub const LN_1_STN_NAME_TO_LED_IDX:  phf::Map<&'static str, usize> = phf_map! {
    "Federal Way Downtown" => 0,
    "Star Lake" => 1,
    "Kent Des Moines" => 2,
    // unused above here until 2026
    "Angle Lake" => 3,
    "SeaTac/Airport"=> 4,
    "Tukwila Int'l Blvd"=> 5,
    "Rainier Beach"=> 6,
    "Othello"=> 7,
    "Columbia City"=> 8,
    "Mount Baker"=> 9,
    "Beacon Hill"=> 10,
    "SODO"=> 11,
    "Stadium"=> 12,
    "Int'l Dist/Chinatown"=> 13,
    "Pioneer Square"=> 14,
    "Symphony"=> 15,
    "Westlake"=> 16,
    "Capitol Hill"=> 17,
    "Univ of Washington"=> 18,
    "U District"=> 19,
    "Roosevelt"=> 20,
    "Northgate"=> 21,
    "Shoreline South/148th"=> 22,
    "Shoreline North/185th"=> 23,
    "Mountlake Terrace"=> 24,
    "Lynnwood City Center"=> 25,
};

#[allow(dead_code)]
pub const LN_2_STN_NAME_TO_LED_IDX: phf::Map<&'static str, usize> = phf_map! {
    "Lynnwood City Center"=> 0,
    "Mountlake Terrace"=> 1,
    "Shoreline North/185th"=> 2,
    "Shoreline South/148th"=> 3,
    "Northgate"=> 4,
    "Roosevelt"=> 5,
    "U District"=> 6,
    "Univ of Washington"=> 7,
    "Capitol Hill"=> 8,
    "Westlake"=> 9,
    "Symphony"=> 10,
    "Pioneer Square"=> 11,
    "Int'l Dist/Chinatown"=> 12,
    "Judkins Park" => 13,
    "Mercer Island" => 14,
    // unused above here until late 2025
    "South Bellevue" => 15,
    "East Main" => 16,
    "Bellevue Downtown" => 17,
    "Wilburton" => 18,
    "Spring District" => 19,
    "BelRed" => 20,
    "Overlake Village" => 21,
    "Redmond Technology" => 22,
    // unused below here until spring 2025
    "Marymoor Village" => 23,
    "Downtown Redmond" => 24,
};

// size of station map * 2 for one LED in between, plus one more for beginning buffer.
pub const PIXELS_FOR_STATIONS: usize = (LN_1_STN_NAME_TO_LED_IDX.len() * 2) - 1;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(dead_code)]
pub enum Destination {
    #[default]
    LynnwoodCC,
    AngleLake,
    SouthBellevue,
    RedmondTech,
}

pub const LED_OFF: Led = Led::off();
pub const STAGING_LED: Led = Led::purple();