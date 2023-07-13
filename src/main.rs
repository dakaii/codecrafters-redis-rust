// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 2048];
                while let Ok(bytes_received) = stream.read(&mut buffer) {
                    if bytes_received == 0 {
                        break;
                    }
                    stream.write_all(b"+PONG\r\n").unwrap();
                    stream.flush().unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
