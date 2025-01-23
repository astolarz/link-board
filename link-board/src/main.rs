use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Duration, Instant}};
use link_board::{data_retriever::dr::get_data_retriever, display, error::Error};
use log::{error, info};
use dotenvy::{self, dotenv};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    simple_logger::init_with_env()?;

    let prog_start = Instant::now();

    let mut display = display::get_display();
    let data_retriever = get_data_retriever();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(_) => {},
            Err(e) => error!("failed to listen for shutdown signal: {}", e),
        };
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
        display::render_trains(&mut display, &data_retriever).await;
        info!("i_{} going to sleep after {} seconds", i, loop_time.elapsed().as_secs());
        i += 1;
    }

    info!("clearing LED strip");
    display.clear_trains();

    info!("exiting");
    Ok(())
}