use amq_protocol_types::FieldTable;
use lapin::{Channel, ConsumerDelegate, message::DeliveryResult, 
            options::{BasicAckOptions, BasicConsumeOptions}};
use super::ConsumerOptions;

#[derive(Clone, Debug)]
pub struct Consumer {
    pub options: ConsumerOptions
}

struct Subscriber {
    channel: Channel,
    message_handler: fn(Vec<u8>)
}

impl Consumer {
    pub fn new(consumer_options: ConsumerOptions) -> Self {
        Consumer {
            options: consumer_options
        }
    }

    pub fn consume(&self, message_handler: fn(Vec<u8>), keep_alive: bool) {
        let connection = super::connect(&self.options.broker_address);
        let channel = super::create_channel(&connection);

        super::create_exchange(&channel,
                               &self.options.exchange_name, 
                               self.options.exchange_type.clone(),
                               self.options.exchange_declare_options.clone());
        let queue = super::create_queue(&channel,
                                        &self.options.queue_name,
                                        self.options.queue_declare_options.clone());
        super::bind_queue(&channel,
                          &self.options.queue_name, 
                          &self.options.exchange_name,
                          &self.options.routing_key,
                          self.options.queue_bind_options.clone());
        channel
            .basic_consume(
                &queue, 
                &self.options.consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default()
            )
            .wait()
            .expect("Error consuming messages")
            .set_delegate(Box::new(Subscriber { channel, message_handler }));

        if keep_alive {
            connection.run().expect("Error running connection");
        }

        super::close_connection(&connection);
    }
}

impl ConsumerDelegate for Subscriber {
    fn on_new_delivery(&self, delivery: DeliveryResult) {
        if let Ok(Some(delivery)) = delivery {
            (self.message_handler)(delivery.data);

            self.channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .wait()
                .expect("basic_ack");
        }
    }
}