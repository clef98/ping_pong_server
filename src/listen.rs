use std::net::UdpSocket;
use std::net::{Shutdown, TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

pub fn tcp(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Successfully connected to server in port {}", address);
                thread::spawn(move || { handle_connection(stream) });
            }
            Err(e) => {
                println!("Failed to receive messages: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut count = 0;
    match stream.read(&mut buffer) {
        Ok(_) => {
            let message = String::from_utf8_lossy(&buffer[..]);
            if message.contains("ping") {
                stream.write("pong\n".as_bytes()).unwrap();
            } else if message.contains("pong") {
                stream.write("ping\n".as_bytes()).unwrap();
            } else {
                stream.write("Message received.\n".as_bytes()).unwrap();
            }
            stream.flush().unwrap();
            count += 1;
            if count == 3 {
                stream.shutdown(Shutdown::Both).unwrap();
                println!("Maximum of three messages permitted on connection. Connection is terminated.");
            }
        }
        Err(e) => {
            println!("Failed to process connection: {}", e);
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
    {
        println!("Successful ping ping.");
    }
}

pub fn udp(address: &str) {
        println!("Begun UDP connection request.");
        let socket = UdpSocket::bind(address).expect("Address is not valid.");
        let mut buffer = [0; 1024];
        let (src) = socket.recv_from(&mut buffer).expect("error");
        let message = String::from_utf8_lossy(&buffer[..]);
        if message.contains("ping") {
            socket.send_to("pong\n".as_bytes(), &src);
        } else if message.contains("pong") {
            socket.send_to("ping\n".as_bytes(), &src);
        } else {
            socket.send_to("Message received.\n".as_bytes(), &src);
        }
}

