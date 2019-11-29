use lapin::{BasicProperties, options::BasicPublishOptions};
use super::PublisherOptions;

#[derive(Clone, Debug)]
pub struct Publisher {
    options: PublisherOptions
}

impl Publisher {
    pub fn new(publisher_options: PublisherOptions) -> Self {
        Publisher {
            options: publisher_options
        }
    }

    pub fn publish(&self, message_bytes: Vec<u8>) {
        let connection = super::connect(&self.options.broker_address);
        let channel = super::create_channel(&connection);

        channel
            .basic_publish(
                &self.options.exchange_name,
                &self.options.routing_key,
                BasicPublishOptions::default(),
                message_bytes,
                BasicProperties::default(),
            )
            .wait()
            .expect("Error publishing message");

        super::close_channel(&channel);
        super::close_connection(&connection);
    }
}