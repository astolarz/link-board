use log::info;
use phf::phf_map;

use crate::{
    constants::{LED_OFF, LED_RED, PIXELS_FOR_STATIONS}, display::LinkBoardDisplay, led::Led, spi_adapter::SpiWriter
};

// Key: Station Name, Value: (south index, north index)
const LN_1_STN_NAME_TO_LED_MAP_IDX:  phf::Map<&'static str, (usize, usize)> = phf_map! {
    "Federal Way Downtown" =>   (1, 209),
    "Star Lake" =>              (6, 204),
    "Kent Des Moines" =>        (13, 197),
    // unused above here until 2026
    "Angle Lake" =>             (17, 193),
    "SeaTac/Airport"=>          (23, 187),
    "Tukwila Int'l Blvd"=>      (31, 182),
    "Rainier Beach"=>           (37, 172),
    "Othello"=>                 (40, 169),
    "Columbia City"=>           (42, 167),
    "Mount Baker"=>             (44, 165),
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
    "NE 130th" =>               (89, 119),
    "Shoreline South/148th"=>   (91, 116),
    "Shoreline North/185th"=>   (94, 113),
    "Mountlake Terrace"=>       (99, 110),
    "Lynnwood City Center"=>    (103, 106),
};

const LN_2_STN_NAME_TO_LED_MAP_IDX: phf::Map<&'static str, (usize, usize)> = phf_map! {
    "Lynnwood City Center"=>    (103, 106),
    "Mountlake Terrace"=>       (99, 110),
    "Shoreline North/185th"=>   (94, 113),
    "Shoreline South/148th"=>   (91, 116),
    "NE 130th" =>               (89, 119),
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

const MAX_LEDS_FOR_STRIP: usize = 302;
const NORTH_1LN_TRAIN_INIT_IDX: usize = 0;
// South Bellevue + 1
const NORTH_1LN_TRAIN_STAGING_IDX: usize = NORTH_1LN_TRAIN_INIT_IDX + PIXELS_FOR_STATIONS;

const SOUTH_1LN_TRAIN_INIT_IDX: usize = NORTH_1LN_TRAIN_STAGING_IDX + 1;
// Angle Lake - 1
const SOUTH_1LN_TRAIN_STAGING_IDX: usize = SOUTH_1LN_TRAIN_INIT_IDX + PIXELS_FOR_STATIONS;

// const MAX_LEDS_NEEDED: usize = SOUTH_1LN_TRAIN_STAGING_IDX + 1;

pub struct MapDisplay {
    adapter: Box<dyn SpiWriter>
}

impl MapDisplay {
    pub fn new(adapter: impl SpiWriter + 'static) -> Self {
        Self {
            adapter: Box::new(adapter)
        }
    }
}

impl LinkBoardDisplay for MapDisplay {
    fn update_trains(&mut self, _trains: Vec<crate::train::Train>) -> Result<(), String> {
        info!("updating map display");
        let mut led_strip: Vec<Led> = vec![LED_OFF; MAX_LEDS_FOR_STRIP];
        // index_trains(self, &mut led_strip, trains);
        for (_, v) in LN_2_STN_NAME_TO_LED_MAP_IDX.entries() {
            led_strip[v.0] = Led::blue();
            led_strip[v.1] = Led::blue();
        }
        for (_, v) in LN_1_STN_NAME_TO_LED_MAP_IDX.entries() {
            led_strip[v.0] = Led::green();
            led_strip[v.1] = Led::green();
        }

        self.adapter.write_rgb(led_strip)
    }

    fn clear_trains(&mut self) {
        self.adapter.clear(MAX_LEDS_FOR_STRIP);
    }

    fn init_red(&mut self) -> Result<(), String> {
        let led_strip = vec![LED_RED; MAX_LEDS_FOR_STRIP];
        self.adapter.write_rgb(led_strip)
    }

    fn get_1n_init_idx(&self) -> usize {
        NORTH_1LN_TRAIN_INIT_IDX
    }

    fn get_1n_staging_idx(&self) -> usize {
        NORTH_1LN_TRAIN_STAGING_IDX
    }

    fn get_1s_init_idx(&self) -> usize {
        SOUTH_1LN_TRAIN_INIT_IDX
    }

    fn get_1s_staging_idx(&self) -> usize {
        SOUTH_1LN_TRAIN_STAGING_IDX
    }

    fn show_2_line(&self) -> bool {
        true
    }
}