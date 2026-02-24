use crate::led::Led;

pub trait SpiWriter {
    fn write_rgb(&mut self, rgb_vec: Vec<Led>) -> Result<(), String>;
    fn clear(&mut self, num_to_clear: usize);
}

#[cfg(feature="rpi")]
pub mod spi {
    use crate::led::Led;
    use super::SpiWriter;
    use log::debug;
    use ws2818_rgb_led_spi_driver::{adapter_gen::WS28xxAdapter, adapter_spi::WS28xxSpiAdapter};
    use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

    pub struct SpiAdapter {
        adapter: ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter,
    }

    pub fn get_adapter() -> impl SpiWriter {
        SpiAdapter::new()
    }
    
    impl SpiAdapter {
        pub fn new() -> Self {
            debug!("running aarch64");
            let adapter = match WS28xxSpiAdapter::new("/dev/spidev0.0") {
                Ok(adapter) => adapter,
                Err(e) => panic!("failed to get spi adapter: {}", e),
            };
            Self {
                adapter
            }
       }
    }

    impl SpiWriter for SpiAdapter {
        fn write_rgb(&mut self, rgb_vec: Vec<Led>) -> Result<(), String> {
            let mut spi_encoded_rgb_bits = vec![];
            for rgb in rgb_vec {
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(rgb.r(), rgb.g(), rgb.b()));
            }
            self.adapter.write_encoded_rgb(&spi_encoded_rgb_bits)
        }

        fn clear(&mut self, num_to_clear: usize) {
            self.adapter.clear(num_to_clear);
        }
    }
}

#[cfg(all(not(feature="rpi"), not(feature="esp32")))]
pub mod spi {
    use crate::led::Led;
    use super::SpiWriter;
    use log::debug;
    use colored::Colorize;

    pub struct SpiAdapter {
    }

    impl SpiAdapter {
        pub fn new() -> Self {
            debug!("running anything else");
            Self {
            }
        }
    }

    pub fn get_adapter() -> impl SpiWriter {
        SpiAdapter::new()
    }

    impl SpiWriter for SpiAdapter {
        fn write_rgb(&mut self, rgb_vec: Vec<Led>) -> Result<(), String> {
            let line = rgb_vec.iter()
                .map(|rgb| format!("{}", "▊".truecolor(rgb.r(), rgb.g(), rgb.b())))
                .collect::<Vec<String>>()
                .join("");
            println!("{}", line);

            Ok(())
        }

        fn clear(&mut self, _num_to_clear: usize) {
        }
    }
}

#[cfg(feature="esp32")]
pub mod spi {
    use esp_idf_hal::gpio::OutputPin;
    use esp_idf_hal::{gpio::InputPin, spi::{config::Config, SpiBusDriver, SpiDriver, SpiDriverConfig, SPI2}};
    use link_board::led::Led;
    use link_board::spi_adapter::SpiWriter;
    use smart_leds::{SmartLedsWrite, RGB8};
    use ws2812_spi::Ws2812;
    use crate::CS;

    pub struct SpiAdapter {
        adapter: Ws2812<SpiBusDriver<'static, SpiDriver<'static>>>,
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
                adapter,
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