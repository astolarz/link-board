pub trait LedAdapter {
    fn write_encoded_rgb(&mut self, encoded_data: &[u8]) -> Result<(), String>;
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
        fn write_encoded_rgb(&mut self, encoded_data: &[u8]) -> Result<(), String> {
            self.adapter.write_encoded_rgb(encoded_data)
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
        fn write_encoded_rgb(&mut self, _encoded_data: &[u8]) -> Result<(), String> {
            Ok(())
        }
    }
}