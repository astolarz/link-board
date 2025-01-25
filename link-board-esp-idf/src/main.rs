use anyhow::Result;
use data_retriever::get_data_retriever;
use dotenvy_macro::dotenv;
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::{delay, prelude::Peripherals}};
use link_board::display;
use spi_adapter::spi::SpiAdapter;
use wifi::wifi;

mod spi_adapter;
mod data_retriever;
mod wifi;

#[tokio::main]
async fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let wifi_ssid = dotenv!("WIFI_SSID");
    let password = dotenv!("WIFI_PASSWORD");

    log::info!("trying to join {} wifi network with password {}", wifi_ssid, password);

    let _wifi = wifi(
        wifi_ssid,
        password,
        peripherals.modem,
        sysloop,
    )?;

    let mut i: u64 = 0;
    let spi_adapter = SpiAdapter::new(
        peripherals.spi2,
        peripherals.pins.gpio6, // sclk
        peripherals.pins.gpio7, // serial_out
        peripherals.pins.gpio4  // serial_in
    );
    let mut display = display::get_display(spi_adapter);
    let data_retriever = get_data_retriever();
    let delay = delay::Delay::new_default();

    loop {
        display::render_trains(&mut display, &data_retriever).await;
        log::info!("loop {}", i);
        i += 1;
        delay.delay_ms(5000 as u32);
    }
}
