use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
};

const BIND_ADDR: &str = "0.0.0.0:48879";
const BUFF_SIZE: usize = 4096;

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; BUFF_SIZE];
        let num_bytes = stream.read(&mut buffer[..]).unwrap();

        if num_bytes > 0 {
            println!("Echo {} bytes", num_bytes);
            stream.write(&buffer[0..num_bytes]).unwrap();
        } else {
            stream.flush().unwrap();  
            break;
        }
    }
    println!("Disconnect client!");
}

fn main() {
    let listener = TcpListener::bind(BIND_ADDR).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        println!("Connection established!");
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}