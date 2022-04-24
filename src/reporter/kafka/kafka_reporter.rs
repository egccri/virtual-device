use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::boxed::Box;
use std::time::Duration;
use crate::reporter::KAFKA_PRODUCER;

/// 客户端配置
/// 正好一次、手动提交、使用key分区，保证顺序性

fn log_produce_result(
    topic: &str,
    result: Result<(i32, i64), (KafkaError, OwnedMessage)>,
) -> Result<(), ()> {
    result
        .and_then(|(p, o)| {
            println!(
                "Successfully produced record to topic {} partition [{}] @ offset {}",
                topic, p, o
            );
            Ok(())
        })
        .map_err(|(err, _)| eprintln!("kafka error: {}", err))
}

pub fn create_produce() -> FutureProducer {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "127.0.0.1:9092");
    config.set("acks", "all");
    // 重排序问题
    config.set("retries", "3");
    // 如果不开启幂等，又不想发生重排序，飞行窗口需要设置为1，如果开启幂等，这个配置不需要配置
    // device.set("max.in.flight.requests.per.connection", "1");
    // 幂等
    config.set("enable.idempotence", "true");

    let producer: FutureProducer = config.create().expect("客户端创建失败");
    producer
}

pub async fn push(message: String, mut topic: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {

    let topic_str = topic.unwrap_or("test");

    let message = KAFKA_PRODUCER
        .send(
            FutureRecord::to(topic_str)
                .payload(message.as_bytes())
                .key("1311120001"),
            Duration::from_secs(0),
        )
        .await;

    log_produce_result(topic_str, message);

    // let messages = (0..9)
    //     .map(|msg| {
    //         let value = msg.to_string();
    //         println!("Preparing to produce record: {} {}", "alice", value);
    //         producer.send(
    //             FutureRecord::to(topic)
    //                 .payload(value.as_bytes())
    //                 .key("alice"),
    //             0,
    //         )
    //     })
    //     .collect::<Vec<_>>();

    Ok(())
}
