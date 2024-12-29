use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Duration, Instant}};
use link_board_display::LinkBoardDisplay;
use log::{error, info};
use dotenvy::{self, dotenv};

mod constants;
mod led;
mod link_board_display;
mod train;
mod data_parser;
mod spi_adapter;
mod env;

async fn render_trains(client: &reqwest::Client, display: &mut Box<dyn LinkBoardDisplay>, i: &mut i32) {
    match data_parser::get_one_line(&client).await {
        Ok(json) => {
            *i += 1;
            match data_parser::parse_from_string(&json) {
                Ok(trains) => {
                    match display.update_trains(trains) {
                        Err(e) => {
                            error!("Failed to update trains: {e}");
                        },
                        _ => {}
                    }
                },
                Err(e) => {
                    error!("Failed to parse 1 Line JSON: {e}");
                }
            }
        },
        Err(e) => {
            error!("Failed to get 1 Line data: {:?}", e);
        }
    }
}

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

        render_trains(&client, &mut display, &mut i).await;
        info!("i_{} going to sleep after {} seconds", i, loop_time.elapsed().as_secs());
    }

    info!("clearing LED strip");
    display.clear_trains().unwrap();

    info!("exiting");
    Ok(())
}