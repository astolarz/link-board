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

/// Key: Station Name,
/// Value: ((south index, LEDs since previous stop), (north index, LEDs since previous stop))
pub const LN_1_STN_NAME_TO_LED_MAP_IDX:  phf::Map<&'static str, ((usize, usize), (usize, usize))> = phf_map! {
    "Federal Way Downtown" =>   ((1, 4), (209, 1)),
    "Star Lake" =>              ((6, 3), (204, 4)),
    "Kent Des Moines" =>        ((10, 6), (200, 3)),
    // unused above here until 2026
    "Angle Lake" =>             ((17, 5), (193, 6)),
    "SeaTac/Airport"=>          ((23, 7), (187, 5)),
    "Tukwila Int'l Blvd"=>      ((31, 4), (182, 4)),
    "Rainier Beach"=>           ((36, 2), (173, 8)),
    "Othello"=>                 ((39, 2), (170, 2)),
    "Columbia City"=>           ((42, 2), (167, 2)),
    "Mount Baker"=>             ((45, 1), (164, 2)),
    "Beacon Hill"=>             ((47, 3), (159, 4)),
    "SODO"=>                    ((51, 1), (157, 1)),
    "Stadium"=>                 ((53, 2), (155, 1)),
    "Int'l Dist/Chinatown"=>    ((56, 1), (152, 2)),
    "Pioneer Square"=>          ((58, 1), (150, 1)),
    "Symphony"=>                ((60, 1), (148, 1)),
    "Westlake"=>                ((62, 3), (146, 1)),
    "Capitol Hill"=>            ((66, 3), (143, 2)),
    "Univ of Washington"=>      ((70, 2), (137, 5)),
    "U District"=>              ((73, 4), (133, 3)),
    "Roosevelt"=>               ((78, 5), (129, 3)),
    "Northgate"=>               ((84, 4), (123, 5)),
    "NE 130th" =>               ((89, 1), (118, 4)),
    "Shoreline South/148th"=>   ((91, 2), (116, 1)),
    "Shoreline North/185th"=>   ((94, 4), (113, 2)),
    "Mountlake Terrace"=>       ((99, 3), (110, 2)),
    "Lynnwood City Center"=>    ((103, 1), (106, 3)),
};

/// Key: Station Name,
/// Value: ((south/east index, LEDs since previous stop), (north/west index, LEDs since previous stop))
pub const LN_2_STN_NAME_TO_LED_MAP_IDX: phf::Map<&'static str, ((usize, usize), (usize, usize))> = phf_map! {
    "Lynnwood City Center"=>    ((103, 1), (106, 3)),
    "Mountlake Terrace"=>       ((99, 3), (110, 2)),
    "Shoreline North/185th"=>   ((94, 4), (113, 2)),
    "Shoreline South/148th"=>   ((91, 2), (116, 1)),
    "NE 130th" =>               ((89, 1), (118, 4)),
    "Northgate"=>               ((84, 4), (123, 5)),
    "Roosevelt"=>               ((78, 5), (129, 3)),
    "U District"=>              ((73, 4), (133, 3)),
    "Univ of Washington"=>      ((70, 2), (137, 5)),
    "Capitol Hill"=>            ((66, 3), (143, 2)),
    "Westlake"=>                ((62, 3), (146, 1)),
    "Symphony"=>                ((60, 1), (148, 1)),
    "Pioneer Square"=>          ((58, 1), (150, 1)),
    "Int'l Dist/Chinatown"=>    ((56, 1), (152, 6)),
    // note the split in indices here due to the 2 line splitting from the 1 at CID
    "Judkins Park" =>           ((261, 6), (248, 7)),
    "Mercer Island" =>          ((269, 7), (240, 3)),
    // unused above here until late 2025
    "South Bellevue" =>         ((276, 6), (236, 1)),
    "East Main" =>              ((278, 1), (234, 2)),
    "Bellevue Downtown" =>      ((280, 1), (231, 1)),
    "Wilburton" =>              ((282, 1), (229, 2)),
    "Spring District" =>        ((285, 2), (226, 2)),
    "BelRed" =>                 ((288, 3), (223, 2)),
    "Overlake Village" =>       ((291, 2), (220, 2)),
    "Redmond Technology" =>     ((294, 2), (217, 2)),
    // unused below here until May 10, 2025
    "Marymoor Village" =>       ((298, 3), (214, 1)),
    "Downtown Redmond" =>       ((300, 1), (212, 1)),
};

// size of station map * 2 for one LED in between, plus one more for beginning buffer.
pub const PIXELS_FOR_STATIONS: usize = (LN_1_STN_NAME_TO_LED_IDX.len() * 2) - 1;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Destination {
    #[default]
    LynnwoodCC,
    AngleLake,
    RedmondTech,
}

pub const LED_OFF: Led = Led::off();
pub const LED_RED: Led = Led::red();
pub const STAGING_LED: Led = Led::purple();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// test that shared stops on the 1 and 2 line (except CID) match values
    fn test_line_map_vals_match() {
        for (k, v) in LN_1_STN_NAME_TO_LED_MAP_IDX.entries() {
            if LN_2_STN_NAME_TO_LED_MAP_IDX.contains_key(k) && *k != "Int'l Dist/Chinatown" {
                let val = LN_2_STN_NAME_TO_LED_MAP_IDX[k];
                assert_eq!(*v, val);
            }
        }
    }

}