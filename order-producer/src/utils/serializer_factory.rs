use crate::serializer::{JsonSerializer, MessagepackSerializer};
use crate::serializer::{SerializationType, Serializer};

pub fn get_serializer(serializer_type: SerializationType) -> Box<dyn Serializer + Send> {
    match serializer_type {
        SerializationType::Json => Box::new(JsonSerializer::new()),
        SerializationType::Messagepack => Box::new(MessagepackSerializer::new()),
    }
}
