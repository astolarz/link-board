pub mod spi {
    use esp_idf_hal::gpio::OutputPin;
    #[cfg(feature="rmt")]
    use esp_idf_hal::{ peripheral::Peripheral, rmt::RmtChannel};
    #[cfg(feature="spi")]
    use esp_idf_hal::{gpio::InputPin, spi::{config::Config, SpiBusDriver, SpiDriver, SpiDriverConfig, SPI2}};
    use link_board::led::Led;
    use link_board::spi_adapter::SpiWriter;
    use smart_leds::{SmartLedsWrite, RGB8};
    #[cfg(feature="rmt")]
    use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
    #[cfg(feature="spi")]
    use ws2812_spi::Ws2812;

    #[cfg(not(feature="rmt"))]
    use crate::CS;

    pub struct SpiAdapter {
        #[cfg(feature="spi")]
        spi_adapter: Ws2812<SpiBusDriver<'static, SpiDriver<'static>>>,
        #[cfg(feature="rmt")]
        rmt_adapter: Ws2812Esp32Rmt<'static>,
    }
    
    impl SpiAdapter {
        #[cfg(feature="spi")]
        pub fn new_spi(spi: SPI2, sclk: impl OutputPin, serial_out: impl OutputPin, serial_in: impl InputPin) -> Self {
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

        #[cfg(feature="rmt")]
        pub fn new_rmt<C: RmtChannel>(channel: impl Peripheral<P = C> + 'static, pin: impl OutputPin)  -> Self {
            let rmt_adapter = Ws2812Esp32Rmt::new(channel, pin).unwrap();
            Self {
                rmt_adapter: rmt_adapter,
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

            #[cfg(feature="spi")]
            let adapter = &mut self.spi_adapter;
            #[cfg(feature="rmt")]
            let adapter = &mut self.rmt_adapter;

            {
                #[cfg(not(feature="rmt"))]
                let _guard = CS.enter();
                
                match adapter.write(rgb8_leds) {
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

            #[cfg(feature="spi")]
            let adapter = &mut self.spi_adapter;
            #[cfg(feature="rmt")]
            let adapter = &mut self.rmt_adapter;

            adapter.write(clear_vec).unwrap();
        }
    }
}