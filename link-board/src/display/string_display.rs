use crate::{
    constants::{LED_OFF, LED_RED, PIXELS_FOR_STATIONS},
    led::Led,
    display::{index_trains, LinkBoardDisplay},
    spi_adapter::SpiWriter,
    train::Train
};
use log::info;

const NORTH_TRAIN_INIT_IDX: usize = 0;
// Lynnwood City Center + 1
const NORTH_TRAIN_STAGING_IDX: usize = NORTH_TRAIN_INIT_IDX + PIXELS_FOR_STATIONS;

const SOUTH_TRAIN_INIT_IDX: usize = NORTH_TRAIN_STAGING_IDX + 1;
// Angle Lake - 1
const SOUTH_TRAIN_STAGING_IDX: usize = SOUTH_TRAIN_INIT_IDX + PIXELS_FOR_STATIONS;

const MAX_LEDS_NEEDED: usize = SOUTH_TRAIN_STAGING_IDX + 1;

pub struct StringDisplay {
    adapter: Box<dyn SpiWriter>
}

impl StringDisplay {
    pub fn new(adapter: impl SpiWriter + 'static) -> Self {
        Self {
            adapter: Box::new(adapter)
        }
    }
}

impl LinkBoardDisplay for StringDisplay {
    fn update_trains(&mut self, trains: Vec<Train>) -> Result<(), String> {
        let mut led_strip: Vec<Led> = vec![LED_OFF; MAX_LEDS_NEEDED];
        let mut count = 0;

        count += index_trains(self, &mut led_strip, trains);
        info!("expecting {} leds", count);

        self.adapter.write_rgb(led_strip)
    }

    fn clear_trains(&mut self) {
        self.adapter.clear(MAX_LEDS_NEEDED);
    }

    fn init_red(&mut self) -> Result<(), String> {
        let led_strip: Vec<Led> = vec![LED_RED; MAX_LEDS_NEEDED];
        self.adapter.write_rgb(led_strip)
    }

    fn get_1n_init_idx(&self) -> usize {
        NORTH_TRAIN_INIT_IDX
    }

    fn get_1n_staging_idx(&self) -> usize {
        NORTH_TRAIN_STAGING_IDX
    }

    fn get_1s_init_idx(&self) -> usize {
        SOUTH_TRAIN_INIT_IDX
    }

    fn get_1s_staging_idx(&self) -> usize {
        SOUTH_TRAIN_STAGING_IDX
    }

    fn show_2_line(&self) -> bool {
        false
    }
}