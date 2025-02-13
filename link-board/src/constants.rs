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
    // unused below here until May 10, 2025
    "Marymoor Village" => 23,
    "Downtown Redmond" => 24,
};

/// Key: Station Name, Value: (south index, north index)
pub const LN_1_STN_NAME_TO_LED_MAP_IDX:  phf::Map<&'static str, (usize, usize)> = phf_map! {
    "Federal Way Downtown" =>   (1, 209),
    "Star Lake" =>              (6, 204),
    "Kent Des Moines" =>        (10, 200),
    // unused above here until 2026
    "Angle Lake" =>             (17, 193),
    "SeaTac/Airport"=>          (23, 187),
    "Tukwila Int'l Blvd"=>      (31, 182),
    "Rainier Beach"=>           (36, 173),
    "Othello"=>                 (39, 170),
    "Columbia City"=>           (42, 167),
    "Mount Baker"=>             (45, 164),
    "Beacon Hill"=>             (47, 159),
    "SODO"=>                    (51, 157),
    "Stadium"=>                 (53, 155),
    "Int'l Dist/Chinatown"=>    (56, 152),
    "Pioneer Square"=>          (58, 150),
    "Symphony"=>                (60, 148),
    "Westlake"=>                (62, 146),
    "Capitol Hill"=>            (66, 143),
    "Univ of Washington"=>      (70, 137),
    "U District"=>              (73, 133),
    "Roosevelt"=>               (78, 129),
    "Northgate"=>               (84, 123),
    "NE 130th" =>               (89, 118),
    "Shoreline South/148th"=>   (91, 116),
    "Shoreline North/185th"=>   (94, 113),
    "Mountlake Terrace"=>       (99, 110),
    "Lynnwood City Center"=>    (103, 106),
};

/// Key: Station Name, Value: (south index, north index)
pub const LN_2_STN_NAME_TO_LED_MAP_IDX: phf::Map<&'static str, (usize, usize)> = phf_map! {
    "Lynnwood City Center"=>    (103, 106),
    "Mountlake Terrace"=>       (99, 110),
    "Shoreline North/185th"=>   (94, 113),
    "Shoreline South/148th"=>   (91, 116),
    "NE 130th" =>               (89, 118),
    "Northgate"=>               (84, 123),
    "Roosevelt"=>               (78, 129),
    "U District"=>              (73, 133),
    "Univ of Washington"=>      (70, 137),
    "Capitol Hill"=>            (66, 143),
    "Westlake"=>                (62, 146),
    "Symphony"=>                (60, 148),
    "Pioneer Square"=>          (58, 150),
    "Int'l Dist/Chinatown"=>    (56, 152),
    "Judkins Park" =>           (248, 261),
    "Mercer Island" =>          (240, 269),
    // unused above here until late 2025
    "South Bellevue" =>         (236, 276),
    "East Main" =>              (234, 278),
    "Bellevue Downtown" =>      (231, 280),
    "Wilburton" =>              (229, 282),
    "Spring District" =>        (226, 285),
    "BelRed" =>                 (223, 288),
    "Overlake Village" =>       (220, 291),
    "Redmond Technology" =>     (217, 294),
    // unused below here until May 10, 2025
    "Marymoor Village" =>       (214, 298),
    "Downtown Redmond" =>       (212, 300),
};

// size of station map * 2 for one LED in between, plus one more for beginning buffer.
pub const PIXELS_FOR_STATIONS: usize = (LN_1_STN_NAME_TO_LED_IDX.len() * 2) - 1;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Destination {
    #[default]
    LynnwoodCC,
    AngleLake,
    SouthBellevue,
    RedmondTech,
}

pub const LED_OFF: Led = Led::off();
pub const LED_RED: Led = Led::red();
pub const STAGING_LED: Led = Led::purple();