use crate::{constants::{Direction, LED_OFF, STAGING_LED}, string_display::StringDisplay, strip_display::StripDisplay, train::Train};
use log::info;
use colored::Colorize;
use std::str::FromStr;

const DISPLAY_TYPE: &str = "LINK_BOARD_DISPLAY_TYPE";

pub trait LinkBoardDisplay {
    fn update_trains(&mut self, trains: Vec<Train>) -> Result<(), String>;
    fn clear_trains(&mut self) -> Result<(), String>;
    fn get_north_init_idx(&self) -> usize;
    fn get_north_staging_idx(&self) -> usize;
    fn get_south_init_idx(&self) -> usize;
    fn get_south_staging_idx(&self) -> usize;
}

#[derive(PartialEq)]
enum DisplayType {
    StripDisplay,
    StringDisplay
}

/// returns a StripDisplay or StringDisplay, defaulting to StripDisplay
pub fn get_display() -> Box<dyn LinkBoardDisplay> {
    if let Ok(display_type_string) = dotenvy::var(DISPLAY_TYPE) {
        if let Ok(display_type) = display_type_string.parse::<DisplayType>() {
            if display_type == DisplayType::StringDisplay {
                return Box::new(StringDisplay::new())
            }
        }
    }
    Box::new(StripDisplay::new())
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDisplayTypeErr;

impl FromStr for DisplayType {
    type Err = ParseDisplayTypeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(parsed) = s.parse::<u8>() {
            match parsed {
                // 0 is StripDisplay
                1 => Ok(DisplayType::StringDisplay),
                _ => Ok(DisplayType::StripDisplay)
            }
        } else {
            Ok(DisplayType::StripDisplay)
        }
    }
}

pub fn index_trains(display: &impl LinkBoardDisplay, led_strip: &mut Vec<(u8, u8, u8)>, trains: Vec<Train>) -> usize {
    let mut total = 0;

    for train in trains {
        total += 1;

        let idx = if train.direction() == Direction::N {
            display.get_north_init_idx() + train.get_relative_idx()
        } else {
            display.get_south_init_idx() + train.get_relative_idx()
        };

        let current_color = led_strip[idx];
        let final_color = if idx == display.get_north_staging_idx() || idx == display.get_south_staging_idx() {
            STAGING_LED
        } else {
            let mut new_color = train.get_led_rgb();
            if current_color == LED_OFF {
                new_color
            } else {
                new_color.2 += 10;
                new_color
            }
        };
        led_strip[idx] = final_color;

        let colorized_dir = if train.direction() == Direction::N {
            "(N)".red()
        } else {
            "(S)".blue()
        };
        info!("placing {} {} at index [{:3}]; next stop: {}", 
            colorized_dir,
            "train".truecolor(final_color.0, final_color.1, final_color.2),
            idx,
            train.next_stop_name);
    }

    info!("{} total trains", total);
    total
}