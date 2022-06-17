use std::io::Read;
use std::io::prelude::*;
use std::net::{TcpStream};
use std::net::TcpListener;
use std::str::from_utf8;
use std::thread;

//MIGHT NEED SERVER ESTAVLISHED LOCALLY IN SAME PROJECT

pub fn tcp(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        println!("Successfully connected to server in port {}", address);
        match stream {
            Ok(stream) => {
                thread::spawn(move || { handle_connection(stream) });
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
}

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        //let mut count = 0;
        stream.read(&mut buffer).unwrap();
        let response = "ping";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
