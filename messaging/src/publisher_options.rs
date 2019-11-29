#[derive(Clone, Debug)]
pub struct PublisherOptions {
    pub broker_address: String,
    pub exchange_name: String,
    pub routing_key: String
}

impl PublisherOptions {
    pub fn new(
        broker_address: String, 
        exchange_name: String, 
        routing_key: String
    ) -> Self 
    {
        PublisherOptions {
            broker_address,
            exchange_name,
            routing_key
        }
    }
}