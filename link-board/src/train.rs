use crate::{
    constants::{Destination, LED_OFF, LN_1_STN_NAME_TO_LED_IDX, LN_1_STN_NAME_TO_LED_MAP_IDX, LN_2_STN_NAME_TO_LED_IDX, LN_2_STN_NAME_TO_LED_MAP_IDX},
    display::Route,
    env,
    led::Led
};
use log::{debug, warn};

const AT_STATION_1LN: Led = Led::green();
const AT_STATION_2LN: Led = Led::blue();
const BTW_STATION_1LN: Led = Led::dull_yellow();
const BTW_STATION_2LN: Led = Led::dull_cyan();

#[derive(Debug, Clone)]
pub struct Train {
    pub next_stop_name: String,
    route: Route,
    destination: Destination,
    next_stop_time_offset: i64,
    closest_stop_time_offset: i64,
}

impl Train {
    pub fn new(
        next_stop_name: String,
        route: Route,
        destination: Destination,
        next_stop_time_offset: i64,
        closest_stop_time_offset: i64) -> Self {
        Self {
            next_stop_name,
            route,
            destination,
            next_stop_time_offset,
            closest_stop_time_offset,
        }
    }

    pub fn route(&self) -> Route {
        self.route
    }

    pub fn destination(&self) -> Destination {
        self.destination
    }

    pub fn get_relative_idx(&self) -> usize {
        debug!("trying to get idx for {:?}", self.next_stop_name.as_str());
        let raw_idx = match self.route { 
            // Subtract index of Angle Lake to normalize at 0
            Route::Line1 => LN_1_STN_NAME_TO_LED_IDX[self.next_stop_name.as_str()] - LN_1_STN_NAME_TO_LED_IDX["Angle Lake"],
            // Subtract index of South Bellevue to normalize at 0
            Route::Line2 => LN_2_STN_NAME_TO_LED_IDX[self.next_stop_name.as_str()] - LN_2_STN_NAME_TO_LED_IDX["South Bellevue"],
        };
        debug!("raw_idx {:?}", raw_idx);
        // TODO: figure out logic for not at station, but next station is max or whatever.
        // will probably also need to adjust index logic in main.rs
        // maybe just actually reverse LEDs for southbound?
        let idx = if env::stations_only() {
            raw_idx
        } else {
            if self.at_station() {
                raw_idx * 2
            } else {
                if self.destination == Destination::LynnwoodCC {
                    if raw_idx > 0 {
                        (raw_idx * 2) - 1
                    } else {
                        warn!("Northbound train at Angle Lake but not at station?");
                        raw_idx * 2
                    }
                } else {
                    (raw_idx * 2) + 1
                }
            }
        };
        debug!("idx is {:?} because train.at_station is {}, heading ", idx, self.at_station());
    
        idx
    }

    pub fn get_map_idx(&self) -> usize {
        let next_stop_idx = self.next_stop_idx();
        if self.at_station() {
            next_stop_idx
        } else {
            match self.route {
                Route::Line1 => next_stop_idx + 1,
                Route::Line2 => next_stop_idx - 1, // TODO: need to accomodate part of 2 Line running on 1 Line section
            }
        }
    }

    fn next_stop_idx(&self) -> usize {
        match self.route {
            Route::Line1 => match self.destination {
                Destination::AngleLake => LN_1_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].0,
                Destination::LynnwoodCC => LN_1_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].1,
                _ => panic!("wrong destination ({:?}) for route ({:?})", self.destination, self.route),
            },
            Route::Line2 => match self.destination {
                Destination::SouthBellevue => LN_2_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].0,
                Destination::RedmondTech => LN_2_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].1,
                _ => panic!("wrong destination ({:?}) for route ({:?})", self.destination, self.route),
            }
        }
    }

    pub fn at_station(&self) -> bool {
        return self.next_stop_time_offset == 0 && self.closest_stop_time_offset == 0
    }

    pub fn get_led_rgb(&self) -> Led {
        if self.at_station() {
            match self.route {
                Route::Line1 => AT_STATION_1LN,
                Route::Line2 => AT_STATION_2LN,
            }
        } else {
            if env::stations_only() {
                LED_OFF
            } else {
                match self.route {
                    Route::Line1 => BTW_STATION_1LN,
                    Route::Line2 => BTW_STATION_2LN,
                }
            }
        }
    }
}