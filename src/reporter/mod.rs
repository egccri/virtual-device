use kafka::kafka_reporter::create_produce;
use lazy_static::lazy_static;
use rdkafka::producer::FutureProducer;

pub mod kafka;

lazy_static! {
    pub static ref KAFKA_PRODUCER: FutureProducer = create_produce();
}
