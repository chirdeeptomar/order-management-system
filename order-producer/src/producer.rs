use std::{
    env,
    sync::{Arc, Mutex},
    thread,
};

use log::{error, info};
use rskafka::client::partition::{Compression, UnknownTopicHandling};

use crate::{
    connection::get_kafka_client,
    utils::{self},
    ORDERS_TOPIC,
};

pub(crate) async fn load(should_terminate: Arc<Mutex<bool>>) -> i32 {
    let args = utils::args_parser::parse(env::args().collect());

    let serializer = utils::serializer_factory::get_serializer(args.serializer_type);

    // Wrap it in Arc<Mutex<>> for thread safety
    let serializer = Arc::new(Mutex::new(serializer));

    // Spawn a thread and move the serializer into it
    let serializer = Arc::clone(&serializer);

    info!("Connecting to Kafka...");

    let client = get_kafka_client().await;

    info!("Connection Established...");

    info!("Sending messages started. ");

    let partition_client = client
        .partition_client(
            ORDERS_TOPIC,
            0, // partition
            UnknownTopicHandling::Retry,
        )
        .await
        .unwrap();

    let mut counter = 0;

    while !*should_terminate.lock().unwrap() {
        let batch = utils::data_factory::genrate_record_batch(&serializer);

        let result = partition_client
            .produce(batch, Compression::default())
            .await;

        match result {
            Ok(_) => {
                counter += 1;
                info!("Batch Saved: {:?}:{}", thread::current().id(), counter);
            }
            Err(err) => {
                error!("Error occured");
                error!("{}", err)
            }
        }
    }

    counter
}
