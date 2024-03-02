use crate::models::Order;

pub enum SerializationType {
    Json,
    Messagepack,
}

pub trait Serializer {
    fn new() -> Self
    where
        Self: Sized;

    fn serialize(&self, new_order: Order) -> Vec<u8>;
}

pub struct JsonSerializer {}

impl Serializer for JsonSerializer {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, new_order: Order) -> Vec<u8> {
        serde_json::to_vec(&new_order).unwrap()
    }
}

pub struct MessagepackSerializer {}

impl Serializer for MessagepackSerializer {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, new_order: Order) -> Vec<u8> {
        rmp_serde::to_vec(&new_order).unwrap()
    }
}
