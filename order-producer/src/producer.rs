use std::sync::{Arc, Mutex};

use log::{error, info};
use rskafka::client::partition::{Compression, UnknownTopicHandling};

use crate::{
    connection::get_kafka_client,
    utils::{self, args_parser::Args},
    ORDERS_TOPIC,
};

pub(crate) async fn load(args: Args, shared_counter: Arc<Mutex<i32>>) {
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

    loop {
        let batch = utils::data_factory::genrate_record_batch(&serializer);

        let result = partition_client
            .produce(batch, Compression::default())
            .await;

        match result {
            Ok(_) => {
                *shared_counter.lock().unwrap() += 1;
                info!("Batch Saved: {}", *shared_counter.lock().unwrap());
            }
            Err(err) => {
                error!("Error occured");
                error!("{}", err)
            }
        }
    }
}
