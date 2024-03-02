use crate::serializer::SerializationType;

pub(crate) fn parse(args: Vec<String>) -> SerializationType {
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
