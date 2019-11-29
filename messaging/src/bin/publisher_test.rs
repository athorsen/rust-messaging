use dotenv::dotenv;
use messaging::{Publisher, PublisherOptions};
use std::env;

fn main() {
    dotenv().ok();

    let broker_address = env::var("RABBITMQ_URL").expect("'RABBITMQ_URL' environment variable");
    let publisher_options = PublisherOptions::new(broker_address,
                                                "zoee.topic".to_string(),
                                                "zoee.rust.test".to_string());
    let publisher = Publisher::new(publisher_options);
    let message = b"Testing publish";
    publisher.publish(message.to_vec());
}