use crate::{strip_display::get_strip_display, train::Train};

pub trait LinkBoardDisplay {
    fn update_trains(&mut self, trains: Vec<Train>) -> Result<(), String>;
    fn clear_trains(&mut self) -> Result<(), String>;
}

pub fn get_display() -> impl LinkBoardDisplay {
    get_strip_display()
}