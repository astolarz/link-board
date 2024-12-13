use crate::{link_board_display::LinkBoardDisplay, spi_adapter::{self, spi::SpiAdapter, SpiWriter}, train::Train};

#[allow(dead_code)]
pub fn get_string_display() -> StringDisplay {
    StringDisplay::new()
}

pub struct StringDisplay {
    adapter: SpiAdapter
}

impl StringDisplay {
    fn new() -> Self {
        Self {
            adapter: spi_adapter::get_adapter()
        }
    }
}

impl LinkBoardDisplay for StringDisplay {
    fn update_trains(&mut self, _trains: Vec<Train>) -> Result<(), String> {
        Ok(())
    }

    fn clear_trains(&mut self) -> Result<(), String> {
        self.adapter.clear(0)
    }
}