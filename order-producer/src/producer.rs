use std::sync::{Arc, Mutex};

use log::{error, info};
use rskafka::client::partition::{Compression, UnknownTopicHandling};

use crate::{connection::get_kafka_client, serializer::Serializer, utils, ORDERS_TOPIC};

pub(crate) async fn load(thread_serializer: Arc<Mutex<Box<dyn Serializer + Send>>>) {
    let mut counter: i32 = 0;

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
        let batch = utils::data_factory::genrate_record_batch(&thread_serializer);

        let result = partition_client
            .produce(batch, Compression::default())
            .await;

        match result {
            Ok(_) => {
                counter += 1;
                info!("Batch Saved: {}", counter);
            }
            Err(err) => {
                error!("Error occured");
                error!("{}", err)
            }
        }
    }
}
