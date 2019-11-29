use lapin::{ExchangeKind, options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions}};

#[derive(Clone, Debug)]
pub struct ConsumerOptions {
    pub broker_address: String,
    pub exchange_name: String,
    pub exchange_type: ExchangeKind, 
    pub exchange_declare_options: ExchangeDeclareOptions,
    pub queue_name: String,
    pub queue_declare_options: QueueDeclareOptions,
    pub queue_bind_options: QueueBindOptions,
    pub routing_key: String,
    pub consumer_tag: String
}

impl ConsumerOptions {
    pub fn new(
        broker_address: String, 
        consumer_tag: String, 
        exchange_name: String, 
        queue_name: String, 
        routing_key: String
    ) -> Self 
    {
        ConsumerOptions {
            broker_address,
            consumer_tag,
            exchange_name,
            exchange_type: ExchangeKind::Topic,
            exchange_declare_options: ExchangeDeclareOptions::default(),
            queue_name,
            queue_declare_options: QueueDeclareOptions::default(),
            queue_bind_options: QueueBindOptions::default(),
            routing_key
        }
    }
}