pub mod cli;
pub mod runner;
pub mod structures;
pub mod utils;
use clap::Parser;
use cli::CliArgs;
use log::{error, info};
use runner::KafkaServer;
use std::{net::IpAddr, str::FromStr};

pub fn run_pipeline() {
    let args = CliArgs::parse();
    info!(
        "Starting kafka at address {}:{} 🔥💣😳🤑🚀...",
        args.ip_address, args.port
    );
    if let Err(e) = KafkaServer::new(IpAddr::from_str(&args.ip_address).unwrap(), args.port).run() {
        error!("Got an error while running a server: {:?}", e);
        error!("Exiting...");
    } else {
        info!("Kafka finished with no errors 🍺🎅💰!")
    }
}
