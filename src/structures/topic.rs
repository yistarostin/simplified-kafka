use log::error;
use std::{io::Write, net::TcpStream};

use super::report::Report;

#[derive(Debug, Default)]
pub struct Topic {
    pub subscribers: Vec<TcpStream>,
}
impl Topic {
    /// Sends message to each of the current `subscribers`. `Subscribers`, which we not able to receiver the message (e.g. have got disconnected), are removed from topic subscribers and will not receiver any new messages.
    pub fn send_news(&mut self, news: &Report) -> Result<(), std::io::Error> {
        self.subscribers
            .retain_mut(|stream| match stream.write(news.message.as_bytes()) {
                Ok(_) => true,
                Err(_) => {
                    error!(
                        "Could not sent message to subscriber {:?}, closing it's stream",
                        stream
                    );
                    false
                }
            });
        Ok(())
    }
    pub fn new() -> Topic {
        Topic {
            subscribers: vec![],
        }
    }
}
