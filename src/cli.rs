use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "kafka",
    author = "yistarostin",
    long_about = r"Simplified version of `Kafka` utility for managing multiple-sender multiple-receiver queues.
 * Implements an a basic abstraction layer called *topic*, which can have multiple publishers and multiple subscribers.
 * After connecting, subscriber gets all the new messages sent by any of the publishers connected to a specific topic.
 * On the other side, new publishers can connect to topics and start sending messages. Each message has to be 2 valid Json object in binary with exactly 2 fields -- `topic` name and a `message`."
)]
pub struct CliArgs {
    /// Ip address of the server for running kafka
    pub ip_address: String,

    /// port to run kafka at
    pub port: u16,
}
