use std::{io::Write, net::TcpStream};

use log::info;

/// Sends an error message to publisher and breaks up with it
pub fn send_nukes(stream: &mut TcpStream) {
    let error_message = "{{\"error\": \"received invalid json\"}}";
    let _ = stream.write_all(error_message.as_bytes());
    info!("Successfully got rid of seek connection {:?} 💀😵", stream)
}
