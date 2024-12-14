use crate::{
    constants::{Direction, STN_NAME_TO_LED_IDX},
    led::Led
};
use log::debug;

const AT_STATION: Led = Led::green();
const BTW_STATION: Led = Led::dull_yellow();

#[derive(Debug, Clone)]
pub struct Train {
    pub next_stop_name: String,
    direction: Direction,
    pub at_station: bool,
}

impl Train {
    pub fn new(
        next_stop_name: String,
        direction: Direction,
        at_station: bool) -> Self {
        Self {
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
                if raw_idx > 0 {
                    (raw_idx * 2) - 1
                } else {
                    raw_idx * 2
                }
            } else {
                (raw_idx * 2) + 1
            }
        };
        debug!("idx is {:?} because train.at_station is {}, heading ", idx, self.at_station);
    
        idx
    }

    pub fn get_led_rgb(&self) -> Led {
        if self.at_station {
            AT_STATION
        } else {
            BTW_STATION
        }
    }
}