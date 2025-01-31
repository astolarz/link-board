use anyhow::{Ok, Result};
use data_retriever::get_data_retriever;
use dotenvy_macro::dotenv;
#[cfg(not(feature="rmt"))]
use esp_idf_hal::interrupt::IsrCriticalSection;
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::{delay, prelude::Peripherals}};
use link_board::display;
use spi_adapter::spi::SpiAdapter;
use wifi::wifi;

mod spi_adapter;
mod data_retriever;
mod wifi;

const LOOP_PAUSE: u32 = 5000;

// IsrCriticalSection crashes when using RMT
#[cfg(not(feature="rmt"))]
static CS: IsrCriticalSection = IsrCriticalSection::new();

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let wifi_ssid = dotenv!("WIFI_SSID");
    let password = dotenv!("WIFI_PASSWORD");

    let _wifi = wifi(
        wifi_ssid,
        password,
        peripherals.modem,
        sysloop,
    )?;

    // required to prevent:
    // cannot initialize I/O event notification: Custom { kind: PermissionDenied, error: "failed to initialize eventfd for polling, try calling `esp_vfs_eventfd_register`" }
    esp_idf_svc::sys::esp!(unsafe {
        esp_idf_svc::sys::esp_vfs_eventfd_register(
            &esp_idf_svc::sys::esp_vfs_eventfd_config_t {
                max_fds: 5,
                ..Default::default()
            },
        )
    })?;

    let mut i: u64 = 0;

    #[cfg(all(feature="esp32", feature="spi"))]
    let spi_adapter = SpiAdapter::new_spi(
        peripherals.spi2,
        peripherals.pins.gpio14,       // sclk
        peripherals.pins.gpio12, // serial_out
        peripherals.pins.gpio13   // serial_in
    );

    #[cfg(all(feature="esp32s3", feature="spi"))]
    let spi_adapter = SpiAdapter::new_spi(
        peripherals.spi2,
        peripherals.pins.gpio12,       // sclk
        peripherals.pins.gpio11, // serial_out
        peripherals.pins.gpio13   // serial_in
    );

    #[cfg(all(feature="esp32s3", feature="rmt"))]
    let spi_adapter = SpiAdapter::new_rmt(peripherals.rmt.channel0, peripherals.pins.gpio11);

    let mut display = display::get_display(spi_adapter);
    let data_retriever = get_data_retriever();
    let delay = delay::Delay::new_default();

    smol::block_on(async {
        loop {
            log::info!("loop {}", i);
            
            display::render_trains(&mut display, &data_retriever).await;
            
            log::info!("sleeping...");
            delay.delay_ms(LOOP_PAUSE);
            i += 1;
        }
    });

    Ok(())
}
