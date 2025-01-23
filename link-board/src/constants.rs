use crate::led::Led;
use phf::phf_map;

pub const LN_1_STN_NAME_TO_LED_IDX:  phf::Map<&'static str, usize> = phf_map! {
    "Angle Lake" => 0,
    "SeaTac/Airport"=> 1,
    "Tukwila Int'l Blvd"=> 2,
    "Rainier Beach"=> 3,
    "Othello"=> 4,
    "Columbia City"=> 5,
    "Mount Baker"=> 6,
    "Beacon Hill"=> 7,
    "SODO"=> 8,
    "Stadium"=> 9,
    "Int'l Dist/Chinatown"=> 10,
    "Pioneer Square"=> 11,
    "Symphony"=> 12,
    "Westlake"=> 13,
    "Capitol Hill"=> 14,
    "Univ of Washington"=> 15,
    "U District"=> 16,
    "Roosevelt"=> 17,
    "Northgate"=> 18,
    "Shoreline South/148th"=> 19,
    "Shoreline North/185th"=> 20,
    "Mountlake Terrace"=> 21,
    "Lynnwood City Center"=> 22
};

#[allow(dead_code)]
pub const LN_2_STN_NAME_TO_LED_IDX: phf::Map<&'static str, usize> = phf_map! {
    "Redmond Technology" => 0,
    "Overlake Village" => 1,
    "BelRed" => 2,
    "Spring District" => 3,
    "Wilburton" => 4,
    "Bellevue Downtown" => 5,
    "East Main" => 6,
    "South Bellevue" => 7,
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