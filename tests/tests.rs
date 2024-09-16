use kafka::runner::KafkaServer;
use std::io::{self, Read, Write};

use std::net::{IpAddr, TcpStream};

use std::process::exit;
use std::str::{self, FromStr};
use std::thread;
use std::time::Duration;

const SUBSCRIBE: &str = "subscribe";
const PUBLISH: &str = "publish";
const ADDRESS: &str = "127.0.0.1";
const PORT_KAFKA: u16 = 1234;
const TOPIC: &str = "test";
const MSG: &str = "Rust is better than C++";
struct ConnectionWrap {
    stream: TcpStream,
}

impl ConnectionWrap {
    fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    fn add_to_topic(&mut self, request_type: &str, topic_name: &str) -> io::Result<()> {
        assert!(request_type == SUBSCRIBE || request_type == PUBLISH);
        let subscribe_command = format!(
            "{{\"method\": \"{}\", \"topic\": \"{}\"}}\n",
            request_type, topic_name,
        );

        self.stream.write_all(subscribe_command.as_bytes())?;
        Ok(())
    }

    fn receive_message(&mut self) -> Result<String, std::io::Error> {
        let mut buffer = [0; 1024];
        let bytes_read = self.stream.read(&mut buffer)?;

        if bytes_read == 0 {
            println!("Connection closed by server.");
            return Err(std::io::ErrorKind::InvalidData.into());
        }

        let incoming_message =
            str::from_utf8(&buffer[..bytes_read]).expect("Failed to convert to UTF-8");
        Ok(incoming_message.to_owned())
    }

    fn publish_message(&mut self, message: &str) -> io::Result<()> {
        let publish_command = format!("{{\"message\": \"{}\"}}\n", message);

        self.stream
            .write_all(publish_command.as_bytes())
            .expect("Not able to publish message");

        Ok(())
    }
}

// Test is a bit badly-written, because it has to be manually interrupted: in starts `kafka`` in separate thread and does not terminate it (as `kafka` doesn't have any API for being stopped easily). So the test actually tests the functionality, but than hangs forever (until manually killed).
#[test]
fn test_simple() {
    let runner = KafkaServer::new(IpAddr::from_str(ADDRESS).unwrap(), PORT_KAFKA);
    thread::spawn(move || {
        let _ = runner.run();
    });
    thread::sleep(Duration::from_secs(1)); // wait some time to let `runner` start the kafka server in an other thread
    let server_address = format!("{}:{}", ADDRESS, PORT_KAFKA);
    let mut publisher =
        ConnectionWrap::new(TcpStream::connect(server_address.to_string()).unwrap());
    publisher.add_to_topic(PUBLISH, TOPIC).unwrap();
    let mut subscriber =
        ConnectionWrap::new(TcpStream::connect(server_address.to_string()).unwrap());
    subscriber.add_to_topic(SUBSCRIBE, TOPIC).unwrap();
    publisher.publish_message(MSG).unwrap();
    let msg = subscriber.receive_message().unwrap();
    assert_eq!(MSG, msg);
    eprintln!("TEST PASSED SUCCESSFULLY, you can exit now");
    exit(0);
}

#[test]
fn test_2subscribers_in_one_topic() {
    let runner = KafkaServer::new(IpAddr::from_str(ADDRESS).unwrap(), PORT_KAFKA);
    thread::spawn(move || {
        let _ = runner.run();
    });
    thread::sleep(Duration::from_secs(1)); // wait some time to let `runner` start the kafka server in an other thread

    let server_address = format!("{}:{}", ADDRESS, PORT_KAFKA);
    let mut publisher =
        ConnectionWrap::new(TcpStream::connect(server_address.to_string()).unwrap());
    publisher.add_to_topic(PUBLISH, TOPIC).unwrap();

    let mut subscriber1 =
        ConnectionWrap::new(TcpStream::connect(server_address.to_string()).unwrap());
    let mut subscriber2 =
        ConnectionWrap::new(TcpStream::connect(server_address.to_string()).unwrap());
    subscriber1.add_to_topic(SUBSCRIBE, TOPIC).unwrap();

    subscriber2.add_to_topic(SUBSCRIBE, TOPIC).unwrap();
    publisher.publish_message(MSG).unwrap();
    let msg1 = subscriber1.receive_message().unwrap();
    let msg2 = subscriber2.receive_message().unwrap();
    assert_eq!(MSG, msg1);
    assert_eq!(MSG, msg2);
    eprintln!("TEST PASSED SUCCESSFULLY, you can exit now");
    exit(0);
}
