use std::{net::TcpListener, thread::spawn};
use tungstenite::{accept_hdr, handshake::server::{Request, Response}, Message};
use sysinfo::{System};

fn main () {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut sys = System::new_all();
            sys.refresh_all();

            let callback = |req: &Request, response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());

                Ok(response)
            };

            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();
            let welcome_message = Message::Text("Connected to Guard".to_string());
            websocket.send(welcome_message).unwrap();

            loop {
                sys.refresh_cpu();

                for cpu in sys.cpus() {
                    let cpu_usage = Message::Text(cpu.cpu_usage().to_string());
                    websocket.send(cpu_usage).unwrap();
                }

                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            }
        });
    }
}