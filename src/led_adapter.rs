pub trait LedAdapter {
    fn write_rgb(&mut self, rgb_vec: Vec<(u8, u8, u8)>) -> Result<(), String>;
    fn clear(&mut self) -> Result<(), String>;
}

pub fn get_adapter() -> impl LedAdapter {
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch="aarch64", target_os="linux", target_env="gnu"))] {
            aarch64::Aarch64LedAdapter::new()
        } else {
            emptyimpl::EmptyImplLedAdapter::new()
        }
    }
}

#[cfg(all(target_arch="aarch64", target_os="linux", target_env="gnu"))]
pub mod aarch64 {
    use log::debug;
    use ws2818_rgb_led_spi_driver::{adapter_gen::WS28xxAdapter, adapter_spi::WS28xxSpiAdapter};
    use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

    use crate::led_adapter::LedAdapter;

    pub struct Aarch64LedAdapter {
        adapter: ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter,
    }
    
    impl Aarch64LedAdapter {
        pub fn new() -> Self {
            debug!("running aarch64");
            Self {
                adapter: WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap()
            }
       }
    }

    impl LedAdapter for Aarch64LedAdapter {
        fn write_rgb(&mut self, rgb_vec: Vec<(u8, u8, u8)>) -> Result<(), String> {
            let mut spi_encoded_rgb_bits = vec![];
            for rgb in rgb_vec {
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(rgb.0, rgb.1, rgb.2));
            }
            self.adapter.write_encoded_rgb(spi_encoded_rgb_bits)
        }

        fn clear(&mut self) -> Result<(), String> {
            let mut spi_encoded_rgb_bits = vec![];
            for _ in 0..MAX_LEDS {
                spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 0, 0));
            }
            self.adapter.write_encoded_rgb(spi_encoded_rgb_bits)
        }
    }
}

#[cfg(any(not(target_arch="aarch64"), not(target_os="linux"), not(target_env="gnu")))]
pub mod emptyimpl {
    use log::debug;
    use crate::led_adapter::LedAdapter;

    pub struct EmptyImplLedAdapter {
    }

    impl EmptyImplLedAdapter {
        pub fn new() -> Self {
            debug!("running anything else");
            Self {
            }
        }
    }

    impl LedAdapter for EmptyImplLedAdapter {
        fn write_rgb(&mut self, _rgb_vec: Vec<(u8, u8, u8)>) -> Result<(), String> {
            Ok(())
        }

        fn clear(&mut self) -> Result<(), String> {
            Ok(())
        }
    }
}