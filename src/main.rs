use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Duration, Instant}};
use link_board_display::LinkBoardDisplay;
use log::{error, info};
use dotenvy::{self, dotenv};

mod constants;
mod strip_display;
mod string_display;
mod link_board_display;
mod train;
mod data_parser;
mod spi_adapter;

#[tokio::main]
async fn main() -> Result<(), tokio::time::error::Error> {
    dotenv().ok();
    simple_logger::init_with_env().unwrap();

    let prog_start = Instant::now();

    let client = reqwest::Client::new();
    let mut display = link_board_display::get_display();

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
        if wait_time.elapsed().as_secs() < 15 {
            continue;
        } else {
            wait_time = Instant::now();
        }

        let loop_time = Instant::now();

        info!("{:?} secs since main loop started.", prog_start.elapsed().as_secs());

        let res = data_parser::get_one_line(&client).await;
        if res.is_err() {
            error!("Failed to get 1 Line data: {:?}", res);
        }
        if let Ok(json) = res {
            info!("it_{}: get_one_line took {} seconds", i,  loop_time.elapsed().as_secs());
            i += 1;
            let trains_result = data_parser::parse_from_string(&json);
            if let Ok(trains) = trains_result {
                display.update_trains(trains).unwrap();
            } else {
                error!("json parse error 2")
            }
        } else {
            error!("json parse error 1");
        }
        info!("i_{} going to sleep after {} seconds", i, loop_time.elapsed().as_secs());
    }

    info!("clearing LED strip");
    display.clear_trains().unwrap();

    info!("exiting");
    Ok(())
}