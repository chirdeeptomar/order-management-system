mod connection;
mod models;
mod securities;
mod serializer;

use chrono::{TimeZone, Utc};
use env_logger::Env;
use log::{error, info};
use rskafka::client::partition::{Compression, UnknownTopicHandling};
use rskafka::record::Record;
use std::error::Error;
use std::time::Instant;
use std::{collections::BTreeMap, env};

use crate::connection::get_kafka_client;
use crate::models::Order;
use crate::serializer::{JsonSerializer, MessagepackSerializer};
use crate::serializer::{Serialization, SerializationType};

const ORDERS_TOPIC: &str = "orders";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = env::args().collect();

    let serializer = arg_parse(args);

    info!("Connecting to Kafka...");

    let client = get_kafka_client().await;

    info!("Connection Established...");

    let mut counter: i32 = 0;

    let start = Instant::now();

    info!("Sending messages started. ");

    let partition_client = client
        .partition_client(
            ORDERS_TOPIC,
            0, // partition
            UnknownTopicHandling::Retry,
        )
        .await
        .unwrap();

    let messagepack_serializer = MessagepackSerializer::new();
    let json_serializer = JsonSerializer::new();

    let serializer: &dyn Serialization = match serializer {
        SerializationType::Json => &json_serializer,
        SerializationType::Messagepack => &messagepack_serializer,
    };

    let records = genrate_records(serializer);
    for data in records {
        let result = partition_client
            .produce(vec![data], Compression::default())
            .await;

        match result {
            Ok(_) => {
                counter += 1;
            }
            Err(err) => {
                error!("Error occured");
                error!("{}", err)
            }
        }
    }

    let duration = start.elapsed();

    info!("Sending messages completed. Total sent: {}", counter);

    info!("Time Taken: {:?}ms", duration.as_millis());

    Ok(())
}

// region:      --- Record Creation

fn genrate_records(serialization: &dyn Serialization) -> impl Iterator<Item = Record> + '_ {
    (1..=50).map(|_| {
        let new_order = Order::new();

        let serialized = serialization.serialize(new_order);

        Record {
            key: None,
            value: Some(serialized),
            headers: BTreeMap::from([]),
            timestamp: Utc.timestamp_millis_opt(42).unwrap(),
        }
    })
}

// endregion    --- Record Creation

// region:      --- Argument Parsing

fn arg_parse(args: Vec<String>) -> SerializationType {
    match args.len() {
        2 => {
            let cmd = &args[1];
            // parse the command
            match &cmd[..] {
                "json" => SerializationType::Json,
                "msgpk" => SerializationType::Messagepack,
                _ => SerializationType::Json,
            }
        }
        _ => SerializationType::Json,
    }
}

// endregion:       --- Argument Parsing
