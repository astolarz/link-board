use std::collections::HashMap;
use colored::Colorize;
use log::info;
use priority_queue::PriorityQueue;

use crate::{
    constants::{Destination, CID, LED_OFF, LED_RED, LN_1_STN_NAME_TO_LED_MAP_IDX, LN_2_STN_NAME_TO_LED_MAP_IDX}, display::LinkBoardDisplay, led::Led, spi_adapter::SpiWriter, train::Train
};

use super::Route;

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
        write_stations_as_dim_white(&mut led_strip);
        self.adapter.write_rgb(led_strip)
    }
}

impl LinkBoardDisplay for MapDisplay {
    fn update_trains(&mut self, trains: Vec<Train>) -> Result<(), String> {
        info!("updating map display");

        let mut led_strip: Vec<Led> = vec![LED_OFF; MAX_LEDS_FOR_STRIP];

        // set stations to purple as a placemarker
        write_stations_as_dim_white(&mut led_strip);

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

    // map of `(stop name, Destination, index before next stop)` to `Destination prioritised by time offset to destination)`
    // the index is used to differentiate where to place Lynnwood-bound trains headed for the CID station,
    // which is where the 1 and 2 lines merge.
    let mut in_betweens: HashMap<(String, Destination, usize), PriorityQueue<(Route, Led), i64>> = Default::default();

    for train in trains {

        let mut final_idx = 0;
        let mut final_color = LED_OFF;

        let base_map_idx = train.get_map_idx();
        let current_color = if led_strip[base_map_idx] == Led::dull_white() {
            LED_OFF
        } else {
            led_strip[base_map_idx]
        };

        if train.at_station() {
            final_idx = base_map_idx;
            if current_color == LED_OFF {
                final_color = train.get_led_rgb();
            } else if current_color != train.get_led_rgb() {
                final_color = Led::purple();
            }
            log_train_placement(train.destination(), train.route(), &train.next_stop_name, final_idx, &final_color, None);
        } else {
            if let Some(pq) = in_betweens.get_mut(&(train.next_stop_name.clone(), train.destination(), train.idx_before_next_stop())) {
                pq.push((train.route(), train.get_led_rgb()), train.next_stop_time_offset());
            } else {
                let mut pq = PriorityQueue::new();
                pq.push((train.route(), train.get_led_rgb()), train.next_stop_time_offset());
                in_betweens.insert((train.next_stop_name.clone(), train.destination(), train.idx_before_next_stop()), pq.clone());
            }
        }

        // actually write the data
        led_strip[final_idx] = final_color;
        total += 1;
    }

    // handle trains in between stations
    for ((next_stop_name, destination, idx_before_next_stop), mut queue) in in_betweens {
        if let Some(((route, _), _)) = queue.peek() {
            let route = route.clone();
            let leds_between_stops = num_leds_between_stops(route, destination, &next_stop_name);

            if leds_between_stops >= queue.len() {
                // easy case: enough leds available to handle all trains
                let mut idx = idx_before_next_stop;
                while !queue.is_empty() {
                    if let Some(((route, led), _)) = queue.pop() {
                        led_strip[idx] = led;
                        log_train_placement(destination, route, &next_stop_name, idx, &led_strip[idx], None);
                        idx = get_next_inbetween_idx(idx, route, destination, &next_stop_name);
                    }
                }
            } else if queue.len() >= leds_between_stops * 2 {
                // other easy case: every spot will be at least doubled, so just color them all with the 
                // 'doubled spot' color
                for idx in idx_before_next_stop..(idx_before_next_stop + leds_between_stops) {
                    led_strip[idx] = Led::dull_orange();
                    log_train_placement(destination, route, &next_stop_name, idx, &led_strip[idx], Some(" [doubled]"));
                }
            } else {
                // hard case: there are more trains than available spots, but not all spots need to be doubled
                let mut num_trains = queue.len();
                let excess_trains = num_trains % leds_between_stops;
                let single_color_trains = leds_between_stops - excess_trains;
                assert_eq!(num_trains, excess_trains + single_color_trains);

                // fill up initial LEDs
                let range = match destination {
                    Destination::LynnwoodCC => match route {
                        Route::Line1 => idx_before_next_stop..(idx_before_next_stop + single_color_trains),
                        Route::Line2 => {
                            if LN_1_STN_NAME_TO_LED_MAP_IDX.contains_key(&next_stop_name) {
                                idx_before_next_stop..(idx_before_next_stop + single_color_trains)
                            } else {
                                idx_before_next_stop..(idx_before_next_stop - single_color_trains)
                            }
                        },
                    },
                    Destination::AngleLake => idx_before_next_stop..(idx_before_next_stop + single_color_trains),
                    Destination::RedmondTech => {
                        if LN_1_STN_NAME_TO_LED_MAP_IDX.contains_key(&next_stop_name) {
                            idx_before_next_stop..(idx_before_next_stop + single_color_trains)
                        } else {
                            idx_before_next_stop..(idx_before_next_stop - single_color_trains)
                        }
                    },
                };
                for idx in range {
                    if let Some(((_, led), _)) = queue.pop() {
                        led_strip[idx] = led;
                        num_trains = num_trains - 1;
                        log_train_placement(destination, route, &next_stop_name, idx, &led_strip[idx], None);
                    }
                }

                // fill the rest with the doubled-up color
                let range = match destination {
                    Destination::LynnwoodCC => match route {
                        Route::Line1 => (idx_before_next_stop + single_color_trains)..(idx_before_next_stop + leds_between_stops),
                        Route::Line2 => {
                            if LN_1_STN_NAME_TO_LED_MAP_IDX.contains_key(&next_stop_name) {
                                (idx_before_next_stop + single_color_trains)..(idx_before_next_stop + leds_between_stops)
                            } else {
                                (idx_before_next_stop - single_color_trains)..(idx_before_next_stop - leds_between_stops)
                            }
                        },
                    },
                    Destination::AngleLake => (idx_before_next_stop + single_color_trains)..(idx_before_next_stop + leds_between_stops),
                    Destination::RedmondTech => {
                        if LN_1_STN_NAME_TO_LED_MAP_IDX.contains_key(&next_stop_name) {
                            (idx_before_next_stop + single_color_trains)..(idx_before_next_stop + leds_between_stops)
                        } else {
                            (idx_before_next_stop - single_color_trains)..(idx_before_next_stop - leds_between_stops)
                        }
                    },
                };
                for idx in range {
                    led_strip[idx] = Led::dull_orange();
                    log_train_placement(destination, route, &next_stop_name, idx, &led_strip[idx], Some(" [doubled]"));
                }
            }

        }
    }

    info!("placed {} trains total", total);

    total
}

fn num_leds_between_stops(route: Route, destination: Destination, next_stop_name: &str) -> usize {
    match destination {
        Destination::LynnwoodCC => match route {
            Route::Line1 => LN_1_STN_NAME_TO_LED_MAP_IDX[next_stop_name].1.1,
            Route::Line2 => LN_2_STN_NAME_TO_LED_MAP_IDX[next_stop_name].1.1,
        },
        Destination::AngleLake => LN_1_STN_NAME_TO_LED_MAP_IDX[next_stop_name].0.1,
        Destination::RedmondTech => LN_2_STN_NAME_TO_LED_MAP_IDX[next_stop_name].0.1,
    }
}

fn write_stations_as_dim_white(led_strip: &mut Vec<Led>) {
    for (_, v) in LN_2_STN_NAME_TO_LED_MAP_IDX.entries() {
        led_strip[v.0.0] = Led::dull_white();
        led_strip[v.1.0] = Led::dull_white();
    }
    for (_, v) in LN_1_STN_NAME_TO_LED_MAP_IDX.entries() {
        led_strip[v.0.0] = Led::dull_white();
        led_strip[v.1.0] = Led::dull_white();
    }
}

fn log_train_placement(
    destination: Destination,
    route: Route,
    next_stop_name: &String,
    idx: usize,
    color: &Led,
    optional_message: Option<&str>
) {
    let colorized_dir = match destination {
        Destination::LynnwoodCC => match route {
            Route::Line1 => "(N)".red(),
            Route::Line2 => "(W)".yellow(),
        },
        Destination::AngleLake => "(S)".blue(),
        Destination::RedmondTech => "(E)".green(),
    };
    let r = if color.r() == 0 { 0 } else { color.r().saturating_add(200) };
    let g = if color.g() == 0 { 0 } else { color.g().saturating_add(200) };
    let b = if color.b() == 0 { 0 } else { color.b().saturating_add(200) };

    info!("placing {} {} at index [{:3}]; next stop: {}{}", 
        colorized_dir,
        "train".truecolor(r, g, b),
        idx,
        next_stop_name,
        if let Some(msg) = optional_message {
            msg
        } else {
            ""
        }
    );
}

fn get_next_inbetween_idx(idx: usize, route: Route, destination: Destination, next_stop_name: &String) -> usize {
    match destination {
        Destination::LynnwoodCC => match route {
            Route::Line1 => idx + 1,
            Route::Line2 => {
                if LN_1_STN_NAME_TO_LED_MAP_IDX.contains_key(&next_stop_name) {
                    if next_stop_name == CID {
                        idx - 1
                    } else {
                        idx + 1
                    }
                } else {
                    idx - 1
                }    
            },
        },
        Destination::AngleLake => idx + 1,
        Destination::RedmondTech => {
            if LN_1_STN_NAME_TO_LED_MAP_IDX.contains_key(&next_stop_name) {
                idx + 1
            } else {
                idx - 1
            }
        },
    }
}