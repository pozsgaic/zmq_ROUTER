use zmq;
use std::env;
use std::f64;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

//const ZMQ_PREFIX: &str = "tcp://127.0.0.1";
const ZMQ_PREFIX: &str = "tcp://0.0.0.0";

fn seconds(d: &Duration) -> f64 {
    d.as_secs() as f64 + (f64::from(d.subsec_nanos()) / 1e9)
}

fn run(ctx: &mut zmq::Context, port: u16) -> Result<(), zmq::Error> {
    let mut msg = zmq::Message::new();
    let listener_url = format!("{}:{}", ZMQ_PREFIX, port);
    let router_socket = ctx.socket(zmq::ROUTER).unwrap();
    router_socket.bind(&listener_url).unwrap();

    println!("Listening to {}", &listener_url);

    loop {
        {
            let mut items = [router_socket.as_poll_item(zmq::POLLIN)];
            zmq::poll(&mut items, -1)?;
            if !items[0].is_readable() {
                println!("ERROR - poll item unreadable!");
                return Ok(());
            }
            println!("Accepted connection - receiving message");
            router_socket.recv(&mut msg, 0)?;
            let sender = &msg.as_str().unwrap().to_string();
            println!("Received message from: {:?}", &sender);

            router_socket.recv(&mut msg, 0)?;
            println!("Message: {:?}", &msg.as_str());
            let message = format!("ACK: {:?}", &msg.as_str());
            router_socket.send(&sender, zmq::SNDMORE)?;
            router_socket.send("", zmq::SNDMORE)?;
            router_socket.send(message.as_str(), 0)?;
        }
    }
}

//  The ROUTER will take a listening address as input
fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let listener_port: u16 = args[1].parse().unwrap();
    let mut ctx = zmq::Context::new();

    match run(&mut ctx, listener_port) {
        Ok(_) => {
            println!("Server shut down");
        }
        Err(_) => {
            println!("Server crashed");
        }
    }
}
