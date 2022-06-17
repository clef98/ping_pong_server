use std::io;
fn main() {
    println!("Type TCP or UDP to select connection type: ");
    let mut protocal_in = String::new();
    io::stdin().read_line(&mut protocal_in).expect("Error with input.");
    println!("Input the address to listen on (e.g. 127.0.0.1:8000): ");
    let mut address_in = String::new();
    io::stdin().read_line(&mut address_in).expect("Error with input.");
    let protocal = protocal_in.trim();
    let address = address_in.trim();
    if protocal == "UDP"{
        println!("UDP selected. Establishing connection.");
    } else if protocal == "TCP" {
        println!("TCP selected. Establishing connection.");
    } else {
        println!("Please input a valid connection type.");
    }
}
