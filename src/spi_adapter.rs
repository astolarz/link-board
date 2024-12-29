use crate::led::Led;

pub trait SpiWriter {
    fn write_rgb(&mut self, rgb_vec: Vec<Led>) -> Result<(), String>;
    fn clear(&mut self, num_to_clear: usize) -> Result<(), String>;
}

pub fn get_adapter() -> spi::SpiAdapter {
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch="aarch64", target_os="linux", target_env="gnu"))] {
            spi::SpiAdapter::new()
        } else {
            spi::SpiAdapter::new()
        }
    }
}

#[cfg(all(target_arch="aarch64", target_os="linux", target_env="gnu"))]
pub mod spi {
    use crate::led::Led;
    use super::SpiWriter;
    use log::debug;
    use ws2818_rgb_led_spi_driver::{adapter_gen::WS28xxAdapter, adapter_spi::WS28xxSpiAdapter};
    use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

    pub struct SpiAdapter {
        adapter: ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter,
    }
    
    impl SpiAdapter {
        pub fn new() -> Self {
            debug!("running aarch64");
            Self {
                adapter: WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap()
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

        fn clear(&mut self, num_to_clear: usize) -> Result<(), String> {
            self.adapter.clear(num_to_clear);
            Ok(())
        }
    }
}

#[cfg(any(not(target_arch="aarch64"), not(target_os="linux"), not(target_env="gnu")))]
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

    impl SpiWriter for SpiAdapter {
        fn write_rgb(&mut self, rgb_vec: Vec<Led>) -> Result<(), String> {
            let line = rgb_vec.iter()
                .map(|rgb| format!("{}", "▊".truecolor(rgb.r(), rgb.g(), rgb.b())))
                .collect::<Vec<String>>()
                .join("");
            println!("{}", line);

            Ok(())
        }

        fn clear(&mut self, _num_to_clear: usize) -> Result<(), String> {
            Ok(())
        }
    }
}