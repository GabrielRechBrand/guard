use std::{net::TcpListener, thread::spawn};
use std::time::Duration;
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

            let system_info = SystemInfo {
                num_cores: sys.cpus().len(),
            };

            let system_info_json = serde_json::to_string(&system_info).unwrap();
            let system_info_message = Message::Text(system_info_json);

            websocket.send(system_info_message).unwrap();

            loop {
                sys.refresh_cpu();

                let cpu = sys.cpus().get(0).unwrap();
                let cpu_usage = Message::Text(((cpu.cpu_usage() * 100.0).round() / 100.0).to_string());
                websocket.send(cpu_usage).unwrap();

                std::thread::sleep(Duration::from_secs(1));
            }
        });
    }
}

#[derive(Debug, serde::Serialize)]
struct SystemInfo {
    num_cores: usize,
}