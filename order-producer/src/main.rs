mod connection;
mod models;
pub mod producer;
mod securities;
mod serializer;
mod utils;

use env_logger::Env;
use log::warn;

use tokio::signal;

use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};

const ORDERS_TOPIC: &str = "orders";

const DEFAULT_BATCH_SIZE: i8 = 50;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = utils::args_parser::parse(env::args().collect());

    // Wrap it in Arc<Mutex<>> for thread safety
    let shared_counter = Arc::new(Mutex::new(1));

    // Spawn a thread and move the cloned counter into it
    let shared_counter_clone = Arc::clone(&shared_counter);

    let join_handle = tokio::spawn(async move { producer::load(args, shared_counter_clone).await });

    signal::ctrl_c().await.expect("failed to listen for event");

    warn!("Shutting down grcaefully...");

    join_handle.abort();

    // Continue using the original serializer in the main thread
    let final_value = shared_counter.lock().unwrap();

    warn!(
        "Total records produced: {:?}",
        (*final_value) * (DEFAULT_BATCH_SIZE as i32)
    );

    warn!("Graceful shutdown completed!");

    Ok(())
}
