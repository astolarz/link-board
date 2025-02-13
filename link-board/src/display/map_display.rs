use colored::Colorize;
use log::info;

use crate::{
    constants::{Destination, LED_OFF, LED_RED, LN_1_STN_NAME_TO_LED_MAP_IDX, LN_2_STN_NAME_TO_LED_MAP_IDX}, display::LinkBoardDisplay, led::Led, spi_adapter::SpiWriter, train::Train
};

use super::{leds_between_stops, Route};

const MAX_LEDS_FOR_STRIP: usize = 302;

pub struct MapDisplay {
    adapter: Box<dyn SpiWriter>
}

impl MapDisplay {
    pub fn new(adapter: impl SpiWriter + 'static) -> Self {
        Self {
            adapter: Box::new(adapter)
        }
    }

    #[allow(dead_code)]
    fn show_static_stations(&mut self) -> Result<(), String> {
        let mut led_strip: Vec<Led> = vec![LED_OFF; MAX_LEDS_FOR_STRIP];
        
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
}

impl LinkBoardDisplay for MapDisplay {
    fn update_trains(&mut self, trains: Vec<Train>) -> Result<(), String> {
        info!("updating map display");

        let mut led_strip: Vec<Led> = vec![LED_OFF; MAX_LEDS_FOR_STRIP];

        index_trains(&mut led_strip, trains);

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
        unimplemented!()
    }

    fn get_1n_staging_idx(&self) -> usize {
        unimplemented!()
    }

    fn get_1s_init_idx(&self) -> usize {
        unimplemented!()
    }

    fn get_1s_staging_idx(&self) -> usize {
        unimplemented!()
    }

    fn show_2_line(&self) -> bool {
        true
    }
}

fn index_trains(led_strip: &mut Vec<Led>, trains: Vec<Train>) -> usize {
    let mut total = 0;
    for train in trains {

        let mut final_idx = 0;
        let mut final_color = LED_OFF;

        let base_map_idx = train.get_map_idx();
        let current_color = led_strip[base_map_idx];

        if train.at_station() {
            final_idx = base_map_idx;
            if current_color == LED_OFF {
                final_color = train.get_led_rgb();
            } else if current_color != train.get_led_rgb() {
                final_color = Led::purple();
            }
        } else {
            if current_color == LED_OFF {
                final_idx = base_map_idx;
                final_color = train.get_led_rgb();
            } else {
                let leds_between_stops = leds_between_stops(train.route(), train.destination(), &train.next_stop_name);
                let mut done = false;
                let is_ln1 = train.route() == Route::Line1;
                // look for an open spot starting from the spot closest to the next stop
                let range_forward = if is_ln1 {
                    0..leds_between_stops
                } else {
                    leds_between_stops..0
                };
                for i in range_forward {
                    let idx = if is_ln1 { base_map_idx + i } else { base_map_idx - i };
                    if led_strip[idx] == LED_OFF {
                        final_idx = idx;
                        final_color = train.get_led_rgb();
                        done = true;
                        break;
                    } 
                }
                // if no spot was found, starting from the back, look for the first non-cyan spot and set it to cyan.
                // if all spots are already cyan, that's the best we can do.
                if !done {
                    let range_back = if is_ln1 {
                        leds_between_stops..0
                    } else {
                        0..leds_between_stops
                    };
                    for i in range_back {
                        let idx = if is_ln1 { base_map_idx + i } else { base_map_idx - i };
                        if led_strip[idx] != Led::dull_orange() {
                            final_idx = idx;
                            final_color = Led::dull_orange();
                        }
                    }
                }
            }
        }
        let colorized_dir = match train.destination() {
            Destination::LynnwoodCC => "(N)".red(),
            Destination::AngleLake => "(S)".blue(),
            Destination::SouthBellevue => "(W)".yellow(),
            Destination::RedmondTech => "(E)".green(),
        };
        info!("placing {} {} at index [{:3}]; next stop: {}", 
            colorized_dir,
            "train".truecolor(final_color.r(), final_color.g(), final_color.b()),
            final_idx,
            train.next_stop_name
        );

        led_strip[final_idx] = final_color;
        total += 1;
    }
    total
}