use lapin::{
            Connection, ConnectionProperties, Channel, ExchangeKind, Queue, 
            options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions}
           };
use amq_protocol_types::FieldTable;

mod consumer_options;
mod consumer;
mod publisher_options;
mod publisher;

pub use consumer_options::ConsumerOptions;
pub use consumer::Consumer;
pub use publisher_options::PublisherOptions;
pub use publisher::Publisher;

pub fn connect(broker_address: &String) -> Connection {
    Connection::connect(&broker_address, ConnectionProperties::default())
                .wait()
                .expect(format!("Error connecting to '{}'", broker_address).as_str())
}

pub fn create_channel(connection: &Connection) -> Channel {
    connection.create_channel().wait().expect("Error creating connection")
}

pub fn create_exchange(channel: &Channel, 
                       exchange_name: &String, 
                       exchange_type: ExchangeKind,
                       exchange_options: ExchangeDeclareOptions,
                      ) 
{
    channel.exchange_declare(&exchange_name, exchange_type, exchange_options, FieldTable::default())
                .wait()
                .unwrap()
}

pub fn create_queue(channel: &Channel,
                    queue_name: &String,
                    queue_options: QueueDeclareOptions,
                   ) -> Queue
{
    channel.queue_declare(&queue_name, queue_options, FieldTable::default())
                .wait()
                .expect("Error creating queue")
}

pub fn bind_queue(channel: &Channel,
                  queue_name: &String,
                  exchange_name: &String,
                  routing_key: &String,
                  bind_options: QueueBindOptions,
                 )
{
    channel.queue_bind(&queue_name, &exchange_name, &routing_key, bind_options, FieldTable::default())
            .wait()
            .unwrap()
}

pub fn close_channel(channel: &Channel) {
    channel.close(200, "OK").wait().expect("Channel close");
}

pub fn close_connection(connection: &Connection) {
    connection.close(200, "OK").wait().expect("Connection close");
}