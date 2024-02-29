use rskafka::client::{Client, ClientBuilder};

pub async fn get_kafka_client() -> Client {
    let hosts = vec!["localhost:19092".to_owned()];
    ClientBuilder::new(hosts).build().await.unwrap()
}
