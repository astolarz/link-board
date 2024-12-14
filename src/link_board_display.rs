use crate::{strip_display::StripDisplay, string_display::StringDisplay, train::Train};
use std::str::FromStr;

const DISPLAY_TYPE: &str = "LINK_BOARD_DISPLAY_TYPE";

pub trait LinkBoardDisplay {
    fn update_trains(&mut self, trains: Vec<Train>) -> Result<(), String>;
    fn clear_trains(&mut self) -> Result<(), String>;
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