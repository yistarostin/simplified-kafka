use log::error;
use serde::Deserialize;
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::structures::{connection_request::ConnectionRequest, request_type::REQUEST_TYPES};

/// Separator of messages. Each message is a valid json without any new lines.
const MESSAGE_SEPARATOR: u8 = b'\n';

/// Reads json from TcpStream Bufreader and tries to parse it to deserializable type `T`. Returns either a constructed object or and error
pub fn try_parse<T>(reader: &mut BufReader<TcpStream>) -> Result<T, serde_json::Error>
where
    T: for<'a> Deserialize<'a>,
{
    let mut buffer = vec![];
    BufReader::read_until(reader, MESSAGE_SEPARATOR, &mut buffer).unwrap();
    serde_json::from_slice(&buffer)
}

pub fn validate_request(stream: &TcpStream) -> Option<ConnectionRequest> {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let result: ConnectionRequest;
    if let Ok(request) = try_parse::<ConnectionRequest>(&mut reader) {
        result = request;
    } else {
        error!(
            "Got an invalid connection attempt from address {:?}, dropping current connection 🤐",
            stream
        );
        return None;
    }
    if !REQUEST_TYPES.contains_key(&result.method) {
        error!(
            "New connection {:?} is neither publisher nor subscriber, ignoring this connection",
            stream
        );
        return None;
    }
    Some(result)
}
