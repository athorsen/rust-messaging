use dotenv::dotenv;
use messaging::{Consumer, ConsumerOptions};
use std::env;
use std::str;

fn main() {
    dotenv().ok();

    let broker_address = env::var("RABBITMQ_URL").expect("'RABBITMQ_URL' environment variable");
    let consumer_options = ConsumerOptions::new(broker_address,
                                                "consumer_test".to_string(),
                                                "zoee.topic".to_string(),
                                                "rusty_queue".to_string(),
                                                "zoee.rust.test".to_string());
    let consumer = Consumer::new(consumer_options);
    consumer.consume(handle_message, true);
}

pub fn handle_message(data: Vec<u8>) {
    println!("Message received: {}", str::from_utf8(&data).unwrap());
}