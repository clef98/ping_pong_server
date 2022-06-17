use std::io::Read;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub fn tcp(address: &str){
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = "ping";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
/*
pub fn udp(address: &str){
    let socket = UdpSocket(address).unwrap();
    let mut message_buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut message_buf)?;
}*/