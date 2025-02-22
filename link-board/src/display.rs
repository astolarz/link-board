use crate::{
    constants::{Destination, LED_OFF, STAGING_LED},
    data_parser,
    data_retriever::DataRetriever,
    display::{string_display::StringDisplay, strip_display::StripDisplay},
    env,
    led::Led,
    spi_adapter::SpiWriter,
    train::Train
};
use log::{error, info, warn};
use colored::Colorize;
use map_display::MapDisplay;
use std::str::FromStr;

mod map_display;
mod string_display;
mod strip_display;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Route {
    #[default]
    Line1,
    Line2,
}

pub trait LinkBoardDisplay {
    fn update_trains(&mut self, trains: Vec<Train>) -> Result<(), String>;
    fn clear_trains(&mut self);
    fn init_red(&mut self) -> Result<(), String>;
    fn get_1n_init_idx(&self) -> usize;
    fn get_1n_staging_idx(&self) -> usize;
    fn get_1s_init_idx(&self) -> usize;
    fn get_1s_staging_idx(&self) -> usize;
    #[allow(dead_code)]
    fn show_2_line(&self) -> bool;
}

#[derive(PartialEq)]
enum DisplayType {
    StripDisplay,
    StringDisplay,
    MapDisplay,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDisplayTypeErr;

impl FromStr for DisplayType {
    type Err = ParseDisplayTypeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.parse::<u8>().unwrap_or(0); 
        match parsed {
            // 0 is StripDisplay
            1 => Ok(DisplayType::StringDisplay),
            2 => Ok(DisplayType::MapDisplay),
            _ => Ok(DisplayType::StripDisplay),
        }
    }
}

fn get_display_type() -> DisplayType {
    match env::display_type_int() {
        1 => DisplayType::StringDisplay,
        2 => DisplayType::MapDisplay,
        _ => DisplayType::StripDisplay
    }
}

/// returns a StripDisplay or StringDisplay, defaulting to StripDisplay
pub fn get_display(adapter: impl SpiWriter + 'static) -> Box<dyn LinkBoardDisplay> {
    let mut display: Box<dyn LinkBoardDisplay> = match get_display_type() {
        DisplayType::StripDisplay => Box::new(StripDisplay::new(adapter)),
        DisplayType::StringDisplay => Box::new(StringDisplay::new(adapter)),
        DisplayType::MapDisplay => Box::new(MapDisplay::new(adapter)),
    };
    display.init_red().unwrap();
    display
}

pub async fn render_trains(display: &mut Box<dyn LinkBoardDisplay>, data_retriever: &impl DataRetriever) {
    match data_parser::get_all_trains(data_retriever).await {
        Ok(trains) => {
            match display.update_trains(trains) {
                Err(e) => {
                    error!("Failed to update trains: {e}");
                },
                _ => {}
            }
        },
        Err(e) => {
            error!("failed to get trains: {e}");
        }
    }
}

fn index_trains(display: &impl LinkBoardDisplay, led_strip: &mut Vec<Led>, trains: Vec<Train>) -> usize {
    let mut total = 0;

    for train in trains {
        if train.route() == Route::Line2 {
            warn!("skipping 2 Line for now!");
            continue;
        }

        total += 1;
        if env::stations_only() && !train.at_station() {
            continue;
        }

        let idx = match train.destination() {
            Destination::LynnwoodCC => display.get_1n_init_idx() + train.get_relative_idx(),
            Destination::AngleLake => display.get_1s_init_idx() + train.get_relative_idx(),
            Destination::RedmondTech => todo!(),
        };

        let current_color = led_strip[idx];
        let final_color = if idx == display.get_1n_staging_idx() || idx == display.get_1s_staging_idx() {
            STAGING_LED
        } else {
            let mut new_color = train.get_led_rgb();
            if current_color == LED_OFF {
                new_color
            } else {
                new_color.add_tuple((0, 0, 100));
                new_color
            }
        };
        led_strip[idx] = final_color;

        let colorized_dir = if train.destination() == Destination::LynnwoodCC {
            "(N)".red()
        } else {
            "(S)".blue()
        };
        info!("placing {} {} at index [{:3}]; next stop: {}", 
            colorized_dir,
            "train".truecolor(final_color.r(), final_color.g(), final_color.b()),
            idx,
            train.next_stop_name);
    }

    info!("{} total trains", total);
    total
}