use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use chrono::{TimeZone, Utc};
use rskafka::record::Record;

use crate::{models::Order, serializer::Serializer, DEFAULT_BATCH_SIZE};

pub fn genrate_record_batch(
    thread_serializer: &Arc<Mutex<Box<dyn Serializer + Send>>>,
) -> Vec<Record> {
    // Access the serializer within the thread using a Mutex guard
    let guard = thread_serializer.lock().unwrap();

    (1..=DEFAULT_BATCH_SIZE)
        .map(|_| {
            let new_order = Order::new();

            let serialized = guard.serialize(new_order);

            Record {
                key: None,
                value: Some(serialized),
                headers: BTreeMap::from([]),
                timestamp: Utc.timestamp_millis_opt(42).unwrap(),
            }
        })
        .collect()
}
