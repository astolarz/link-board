use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Duration, Instant}};
use log::info;
use dotenvy::{self, dotenv};

mod error;
mod constants;
mod led;
mod display;
mod train;
mod data_parser;
mod spi_adapter;
mod env;

#[tokio::main]
async fn main() -> Result<(), tokio::time::error::Error> {
    dotenv().ok();
    simple_logger::init_with_env().unwrap();

    let prog_start = Instant::now();

    let client = reqwest::Client::new();
    let mut display = display::get_display();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("ctrl-c interrupt");
        r.store(false, Ordering::SeqCst);
    });

    let mut i = 0;
    let mut wait_time = Instant::now() - Duration::new(15, 0);
    while running.load(Ordering::SeqCst) {
        let loop_time = Instant::now();

        if wait_time.elapsed().as_secs() < 15 {
            continue;
        } else {
            wait_time = Instant::now();
        }

        info!("{:?} secs since main loop started.", prog_start.elapsed().as_secs());
        display::render_trains(&client, &mut display).await;
        info!("i_{} going to sleep after {} seconds", i, loop_time.elapsed().as_secs());
        i += 1;
    }

    info!("clearing LED strip");
    display.clear_trains().unwrap();

    info!("exiting");
    Ok(())
}