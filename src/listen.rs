
use std::net::{Shutdown, TcpStream};
use std::net::TcpListener;
use std::io::{Read, Write, Error};
//use std::str::from_utf8;
use std::thread;

//MIGHT NEED SERVER ESTAVLISHED LOCALLY IN SAME PROJECT

pub fn tcp(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        println!("Successfully connected to server in port {}", address);
        match stream {
            Ok(stream) => {
                thread::spawn(move || { handle_connection(stream) });
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
}

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let mut count = 0;
        'reading_stream: while match stream.read(&mut buffer){
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
                count+=1;
                if count == 3{
                    stream.shutdown(Shutdown::Both);
                    println!("Maximum of three messages permitted on connection. Connection is terminated.");
                }
                true
            },
            Err(e) => {
                println!("Failed to process connection: {}", e);
                break 'reading_stream;
            }
        } {
            println!("Successful ping ping.");
        }
    }
