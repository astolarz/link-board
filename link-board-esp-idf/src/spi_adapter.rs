pub mod spi {
    use esp_idf_hal::gpio::OutputPin;
    use esp_idf_hal::{gpio::InputPin, spi::{config::Config, SpiBusDriver, SpiDriver, SpiDriverConfig, SPI2}};
    use link_board::led::Led;
    use link_board::spi_adapter::SpiWriter;
    use smart_leds::{SmartLedsWrite, RGB8};
    use ws2812_spi::Ws2812;
    use crate::CS;

    pub struct SpiAdapter {
        spi_adapter: Ws2812<SpiBusDriver<'static, SpiDriver<'static>>>,
    }
    
    impl SpiAdapter {
        pub fn new(spi: SPI2, sclk: impl OutputPin, serial_out: impl OutputPin, serial_in: impl InputPin) -> Self {
            log::info!("setting up spi driver");
            let driver = SpiDriver::new::<SPI2>(
                spi,
                sclk,
                serial_out,
                Some(serial_in),
                &SpiDriverConfig::new(),
            ).unwrap();

            log::info!("setting up spi bus");
            let config = Config::new().baudrate(3_000_000.into());
            let spi_bus = SpiBusDriver::new(driver, &config).unwrap();

            let adapter = Ws2812::new(spi_bus);
            log::info!("running esp32");
            Self {
                spi_adapter: adapter,
            }
        }
    }

    impl SpiWriter for SpiAdapter {
        fn write_rgb(&mut self, rgb_vec: Vec<Led>) -> Result<(), String> {
            log::info!("writing {} leds", rgb_vec.len());

            let mut rgb8_leds = vec![];
            for rgb in rgb_vec {
                rgb8_leds.push(RGB8::new(rgb.r(), rgb.g(), rgb.b()));
            }

            {
                let _guard = CS.enter();

                match self.adapter.write(rgb8_leds) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        log::error!("{}", e.to_string());
                        Err(e.to_string())
                    },
                }
            }
        }

        fn clear(&mut self, num_to_clear: usize) {
            let clear_vec = vec![RGB8::new(0, 0, 0); num_to_clear];

            self.adapter.write(clear_vec).unwrap();
        }
    }
}