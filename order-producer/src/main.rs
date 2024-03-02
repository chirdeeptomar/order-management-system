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

    let serializer = utils::serializer_factory::get_serializer(args.serializer_type);

    // Wrap it in Arc<Mutex<>> for thread safety
    let serializer = Arc::new(Mutex::new(serializer));

    // Spawn a thread and move the serializer into it
    let thread_serializer = Arc::clone(&serializer);

    let join_handle = tokio::spawn(async move {
        producer::load(thread_serializer).await;
    });

    signal::ctrl_c().await.expect("failed to listen for event");

    warn!("Shutting down gracefully...");
    join_handle.abort();

    Ok(())
}
