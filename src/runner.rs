use log::{error, info};
use std::{
    collections::HashMap,
    io::BufReader,
    net::{IpAddr, SocketAddr, TcpListener},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    structures::{
        report::Report,
        request_type::{RequestType, REQUEST_TYPES},
        topic::Topic,
    },
    utils::{
        killer::send_nukes,
        parser::{try_parse, validate_request},
    },
};

pub struct KafkaServer {
    pub listener: TcpListener,
}

type TopicStorage = Arc<Mutex<HashMap<String, Topic>>>;

// The actual server controller. Gets `ip` and `port` to start in server at.
impl KafkaServer {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        KafkaServer {
            listener: TcpListener::bind(SocketAddr::new(ip, port)).unwrap(),
        }
    }
    pub fn run(&self) -> Result<(), std::io::Error> {
        let topics: TopicStorage = Arc::new(Mutex::new(HashMap::new()));
        for stream in self.listener.incoming() {
            let mut stream = stream?;
            let connection_request = validate_request(&stream);
            if connection_request.is_none() {
                continue;
            }
            let connection_request = connection_request.unwrap();
            let topic_name = connection_request.topic;
            let request_type = connection_request.method;
            {
                let mut topics = topics.lock().unwrap();
                if !topics.contains_key(&topic_name) {
                    topics.insert(topic_name.clone(), Topic::new());
                }
                if REQUEST_TYPES[&request_type] == RequestType::Subscribe {
                    topics
                        .get_mut(&topic_name)
                        .unwrap()
                        .subscribers
                        .push(stream.try_clone().unwrap());
                }
            }
            if REQUEST_TYPES[&request_type] == RequestType::Publish {
                let topics = Arc::clone(&topics);
                thread::spawn(move || {
                    let mut buffreader = BufReader::new(stream.try_clone().unwrap());
                    loop {
                        let parse_report_result = try_parse::<Report>(&mut buffreader);
                        if parse_report_result.is_err() {
                            error!("Got an invalid message from connection {:?}; dropping current publisher 🙃",stream);
                            send_nukes(&mut stream);
                            return;
                        }
                        let news = parse_report_result.unwrap();
                        {
                            let mut topics = topics.lock().unwrap();
                            let current_topic = topics.get_mut(&topic_name).unwrap();
                            let okay_send = current_topic.send_news(&news);
                            if okay_send.is_err() {
                                error!(
                                    "Got some error during sending message to topic {} ❌",
                                    topic_name
                                );
                            } else {
                                info!( "Successfully send message {} to all topic {} subscribers 🏆🥳🏆", news.message, topic_name)
                            }
                        }
                    }
                });
            }
        }
        Ok(())
    }
}
