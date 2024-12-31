use tokio::time::{interval, Duration};

use config_fly::CONFIG;
use enigo::{Coordinate, Enigo, Mouse, Settings};
use log::error;
// use rustlib::log_fly;
pub mod config_fly;
use tokio::{
    self, select,
    sync::mpsc::{self, channel},
    time::sleep,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize logger
    // log_fly::log_init();

    // Setup Ctrl-C handler
    let (tx, rx) = channel::<()>(1);
    ctrlc2::set_async_handler(async move {
        if let Err(e) = tx.send(()).await {
            error!("Error sending message: {}", e);
        }
    })
    .await;

    // Setup Enigo to move mouse
    let mut enigo = match Enigo::new(&Settings::default()) {
        // if ok, continue
        Ok(enigo) => enigo,
        // If error, exit
        Err(e) => {
            error!("Error creating Enigo: {}", e);
            return;
        }
    };

    // Run the loop, if error, print error
    if let Err(e) = run_loop(&mut enigo, rx).await {
        println!("Error running loop: {}", e);
    }
}

/// Run the loop
async fn run_loop(enigo: &mut Enigo, mut rx: mpsc::Receiver<()>) -> anyhow::Result<()> {
    // Get interval from config
    let mut loop_interval = interval(Duration::from_secs(CONFIG.interval));
    //interval的第一次会立即返回，所以在loop前先执行一次
    loop_interval.tick().await;
    // Check if interval is 0
    if loop_interval.period().is_zero() {
        return Err(anyhow::anyhow!("The interval can not be 0 in config.toml"));
    }
    // Count for how many times the mouse has moved
    let mut count = 1u64;
    // Loop
    loop {
        // Print the time, count and interval
        println!(
            "{} --> The {}-th time move. Every {} seconds. Press CTRL+C to exit this program.",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            count,
            loop_interval.period().as_secs()
        );
        // Increase the count
        count += 1;

        // Move mouse
        enigo.move_mouse(1, 1, Coordinate::Rel).unwrap();
        // Wait for a while
        sleep(Duration::from_millis(10)).await;
        // Move mouse back
        enigo.move_mouse(-1, -1, Coordinate::Rel).unwrap();

        // Check if Ctrl-C was pressed
        select! {
            // Wait for a while for next move
            _=loop_interval.tick()=>{},
            // Check if Ctrl-C was pressed
            _=rx.recv()=>{
                println!("Ctrl-C pressed, exiting...");
                break;
            }
        }
    }
    Ok(())
}
