mod listen;
use std::env;

fn main() {
    println!("Type TCP or UDP to select connection type: ");
    let args: Vec<String> = env::args().collect();
    let protocol = &args[1];
    let address = &args[2];
    if protocol == "UDP"{
        println!("UDP selected. Establishing connection.");
        listen::udp(address).expect("UDP connection has terminted.");
    } else if protocol == "TCP" {
        println!("TCP selected. Establishing connection.");
        listen::tcp(address);
    } else {
        println!("Please input a valid connection type.");
    }
    println!("Thank you for using ping pong.");
}
