use std::time::Duration;

use config_fly::CONFIG;
use enigo::{Coordinate, Enigo, Mouse, Settings};
use log::info;
// use rustlib::log_fly;
pub mod config_fly;
use tokio::{self, select, sync::mpsc::channel, time::sleep};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize logger
    // log_fly::log_init();
    // Get interval from config
    let interval = CONFIG.interval;
    // Setup Ctrl-C handler
    let (tx, mut rx) = channel::<()>(1);
    ctrlc2::set_async_handler(async move {
        tx.send(())
            .await
            .expect("Could not send signal on channel.");
    })
    .await;
    // Setup Enigo
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut count = 1u64;
    loop {
        // Print the count
        println!("{} ~ The {}-th time move. Press CTRL+C to exit this program.", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),count);
        count += 1;
        // Move mouse
        enigo.move_mouse(1, 1, Coordinate::Rel).unwrap();
        // Wait for a while
        sleep(Duration::from_millis(10)).await;
        // Move mouse back
        enigo.move_mouse(-1, -1, Coordinate::Rel).unwrap();
        // Wait for a while for next move
        let sleep_future = sleep(Duration::from_secs(interval));
        // Check if Ctrl-C was pressed
        select! {
            // Wait for a while for next move
            _=sleep_future=>{},
            // Check if Ctrl-C was pressed
            _=rx.recv()=>{
                info!("Ctrl-C pressed, exiting...");
                break;
            }
        }
    }
}
