mod connection;
mod models;
mod securities;
mod serializer;

use chrono::{TimeZone, Utc};
use rskafka::client::partition::{Compression, UnknownTopicHandling};
use rskafka::record::Record;
use std::error::Error;
use std::time::Instant;
use std::{collections::BTreeMap, env};

use crate::connection::get_kafka_client;
use crate::models::Order;
use crate::serializer::{JsonSerializer, MessagepackSerializer};
use crate::serializer::{Serialization, SerializationType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect();

    let serializer = arg_parse(args);

    let topic = "orders";

    println!("Connecting to Kafka...");

    let client = get_kafka_client().await;

    println!("Connection Established...");

    let mut counter: i32 = 0;

    let start = Instant::now();

    println!("Starting to send messages.");

    let partition_client = client
        .partition_client(
            topic.to_owned(),
            0, // partition
            UnknownTopicHandling::Retry,
        )
        .await
        .unwrap();

    let messagepack_serializer = MessagepackSerializer::new();
    let json_serializer = JsonSerializer::new();

    let serializer: &dyn Serialization = match serializer {
        SerializationType::JSON => &json_serializer,
        SerializationType::MESSAGEPACK => &messagepack_serializer,
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
                println!("Error occured");
                println!("{}", err)
            }
        }
    }

    let duration = start.elapsed();

    println!("Sending messages completed. Messages sent: {}", counter);

    println!("Time Taken: {:?}ms", duration.as_millis());

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
                "json" => SerializationType::JSON,
                "msgpk" => SerializationType::MESSAGEPACK,
                _ => SerializationType::JSON,
            }
        }
        _ => SerializationType::JSON,
    }
}

// endregion:       --- Argument Parsing
