use log::debug;
use crate::constants::{Direction, STN_NAME_TO_LED_IDX};

const AT_STATION: (u8, u8, u8) = (0, 255, 0);
const BTW_STATION: (u8, u8, u8) = (100, 100, 0);

#[derive(Debug, Clone)]
pub struct Train {
    // id: String,
    // next_stop_id: String,
    pub next_stop_name: String,
    // time_until: std:time::duration,
    direction: Direction,
    pub at_station: bool,
}

impl Train {
    pub fn new(
        // id: String,
        // next_stop_id: String,
        next_stop_name: String,
        direction: Direction,
        at_station: bool) -> Self {
        Self {
            // id,
            // next_stop_id,
            next_stop_name,
            direction,
            at_station,
        }
    }

    pub fn add_dir(&mut self, direction: Direction) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn get_relative_idx(&self) -> usize {
        debug!("trying to get idx for {:?}", self.next_stop_name.as_str());
        let raw_idx = STN_NAME_TO_LED_IDX[self.next_stop_name.as_str()];
        debug!("raw_idx {:?}", raw_idx);
        // TODO: figure out logic for not at station, but next station is max or whatever.
        // will probably also need to adjust index logic in main.rs
        // maybe just actually reverse LEDs for southbound?
        let idx = if self.at_station {
            raw_idx * 2
        } else {
            if self.direction == Direction::N {
                (raw_idx * 2) - 1
            } else {
                (raw_idx * 2) + 1
            }
        };
        debug!("idx is {:?} because train.at_station is {}, heading ", idx, self.at_station);
    
        idx
    }

    pub fn get_led_rgb(&self) -> (u8, u8, u8) {
        if self.at_station {
            AT_STATION
        } else {
            BTW_STATION
        }
    }
}