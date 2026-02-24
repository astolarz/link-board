use crate::{
    constants::{LED_OFF, LED_RED, PIXELS_FOR_STATIONS},
    led::Led,
    display::{index_trains, LinkBoardDisplay},
    spi_adapter::SpiWriter,
    train::Train
};
use log::{info, warn};

const MAX_LEDS_FOR_STRIP: usize = 144;
const LED_BUFFER_COUNT: usize = 3;
const START_BUF_LED: Led = Led::red();
const MID_BUF_LED: Led = Led::orange();
const END_BUF_LED: Led = Led::blue();

// First three LEDs are start buffer (red).
//
// Next 45 LEDs are for northbound trains, starting with Angle Lake station,
// second to last is Lynnwood City Center, last is staging (purple) for train
// about to return south.
//
// Three LEDs for mid buffer (orange).
//
// Next 45 LEDs are for southbound trains, starting with staging (purple) for
// train about to return north, then Angle Lake, ending with Lynnwood City Center.
//
// Three LEDs for end buffer (blue).
//
// For north and southbound train sections, LEDs alternate between at station
// (green), and in between stations (yellow). If more than one train is in one
// of those sections, 10 blue is added, making at-station LEDs more teal, and in
// between stations more purple.
//
// Both north and southbound sections have Angle Lake (southernmost station) on
// the left, and Lynwood City Center (northernmost station) on the right.
const START_BUF_INIT_IDX: usize = 0;

const NORTH_TRAIN_INIT_IDX: usize = START_BUF_INIT_IDX + LED_BUFFER_COUNT;
// Lynnwood City Center + 1
const NORTH_TRAIN_STAGING_IDX: usize = NORTH_TRAIN_INIT_IDX + PIXELS_FOR_STATIONS;

const MID_BUF_INIT_IDX: usize = NORTH_TRAIN_STAGING_IDX + 1;

// Angle Lake - 1
const SOUTH_TRAIN_STAGING_IDX: usize = MID_BUF_INIT_IDX + LED_BUFFER_COUNT;
const SOUTH_TRAIN_INIT_IDX: usize = SOUTH_TRAIN_STAGING_IDX + 1;

const END_BUF_INIT_IDX: usize = SOUTH_TRAIN_INIT_IDX + PIXELS_FOR_STATIONS;

const MAX_LEDS_NEEDED: usize = END_BUF_INIT_IDX + LED_BUFFER_COUNT;

pub struct StripDisplay {
    adapter: Box<dyn SpiWriter>
}

impl StripDisplay {
    pub fn new(adapter: impl SpiWriter + 'static) -> Self {
        assert!(MAX_LEDS_NEEDED <= MAX_LEDS_FOR_STRIP);
        Self {
            adapter: Box::new(adapter)
        }
    }
}

impl LinkBoardDisplay for StripDisplay {
    fn update_trains(&mut self, trains: Vec<Train>) -> Result<(), String> {
        let mut led_strip: Vec<Led> = vec![LED_OFF; MAX_LEDS_FOR_STRIP];
        let mut count = 0;

        // write initial leds
        info!("START BUFFER");
        count += prepare_buffer_leds(&mut led_strip, START_BUF_INIT_IDX, START_BUF_LED);

        count += index_trains(self, &mut led_strip, trains);

        // write mid buffer LEDs
        info!("MID BUFFER");
        count += prepare_buffer_leds(&mut led_strip, MID_BUF_INIT_IDX, MID_BUF_LED);

        // write end buffer LEDs
        info!("END BUFFER");
        count += prepare_buffer_leds(&mut led_strip, END_BUF_INIT_IDX, END_BUF_LED);
        info!("expecting {} leds", count);
        
        self.adapter.write_rgb(led_strip)
    }

    fn clear_trains(&mut self) {
        self.adapter.clear(MAX_LEDS_FOR_STRIP);
    }

    fn init_red(&mut self) -> Result<(), String> {
        let led_strip: Vec<Led> = vec![LED_RED; MAX_LEDS_FOR_STRIP];
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
}

fn prepare_buffer_leds(led_strip: &mut Vec<Led>, init_idx: usize, led_val: Led) -> usize {
    let mut count_written = 0;
    for i in 0..LED_BUFFER_COUNT {
        let idx = init_idx + i;
        if led_strip[idx] != LED_OFF {
            warn!("multiple trains at index [{}]", idx);
        }
        led_strip[idx] = led_val;
        info!("placing buffer at index [{}]", idx);
        count_written += 1;
    }
    count_written
}