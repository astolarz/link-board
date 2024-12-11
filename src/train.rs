use crate::Direction;
use log::{debug, warn};

const AT_STATION: (u8, u8, u8) = (0, 25, 0);
const BTW_STATION: (u8, u8, u8) = (5, 5, 0);

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
        let raw_idx = crate::STN_NAME_TO_LED_IDX[self.next_stop_name.as_str()];
        debug!("raw_idx {:?}", raw_idx);
        let idx = if self.at_station {
            raw_idx * 2
        } else if !self.at_station && raw_idx == 0 {
            warn!("heading south to Angle Lake, right? {:?} {}", self.direction, self.next_stop_name);
            1
        } else {
            (raw_idx * 2) - 1
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