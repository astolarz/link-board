use crate::{
    constants::{Destination, CID, JUDKINS_PARK, LED_OFF, LN_1_STN_NAME_TO_LED_IDX, LN_1_STN_NAME_TO_LED_MAP_IDX, LN_2_STN_NAME_TO_LED_IDX, LN_2_STN_NAME_TO_LED_MAP_IDX},
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
                Route::Line2 => {
                    if LN_1_STN_NAME_TO_LED_MAP_IDX.contains_key(&self.next_stop_name) {
                        next_stop_idx + 1
                    } else {
                        next_stop_idx - 1
                    }
                },
            }
        }
    }

    fn next_stop_idx(&self) -> usize {
        match self.destination {
            Destination::LynnwoodCC => match self.route {
                Route::Line1 => LN_1_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].1.0,
                Route::Line2 => LN_2_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].0.0,
            },
            Destination::AngleLake => LN_1_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].0.0,
            Destination::RedmondTech => LN_2_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].1.0,
        }
    }

    /// returns the index of the LED immediately before the next stop
    pub fn idx_before_next_stop(&self) -> usize {
        match self.destination {
            Destination::LynnwoodCC => {
                match self.route {
                    Route::Line1 => LN_1_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].1.0 + 1,
                    Route::Line2 => {
                        if self.next_stop_name == CID {
                            let idx = LN_2_STN_NAME_TO_LED_MAP_IDX[JUDKINS_PARK].1.0 + LN_2_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].1.1;
                            assert_eq!(idx, 254);
                            idx
                        } else {
                            LN_2_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].0.0
                        }
                    },
                }
            },
            Destination::AngleLake => LN_1_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].0.0 + 1,
            Destination::RedmondTech => {
                // Lynnwood to CID, add 1; Judkins Park to Redmond, subtract 1
                if LN_1_STN_NAME_TO_LED_MAP_IDX.contains_key(&self.next_stop_name) {
                    LN_2_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].1.0 + 1
                } else {
                    LN_2_STN_NAME_TO_LED_MAP_IDX[&self.next_stop_name].1.0 - 1
                }
            },
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

    pub fn next_stop_time_offset(&self) -> i64 {
        self.next_stop_time_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cid_idx_before_next_stop() {
        let train = Train {
            next_stop_name: String::from(CID),
            route: Route::Line2,
            destination: Destination::LynnwoodCC,
            next_stop_time_offset: 234,
            closest_stop_time_offset: 2134,
        };

        assert_eq!(train.idx_before_next_stop(), 254);
    }

}