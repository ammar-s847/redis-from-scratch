use std::{
    thread,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    collections::{HashMap},
};

pub fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                match String::from_utf8_lossy(&buffer[0..n]).trim() {
                    "set" => {
            
                        let response = "Set request received";
                        stream.write(response.as_bytes()).unwrap();
                    },
                    "get" => {
                        
                        let response = "Get request received";
                        stream.write(response.as_bytes()).unwrap();
                    },
                    "del" => {
                        
                        let response = "Delete request received";
                        stream.write(response.as_bytes()).unwrap();
                    },
                    _ => {
                        let response = "Invalid request type";
                        stream.write(response.as_bytes()).unwrap();
                    },
                };
                // println!("{}", String::from_utf8_lossy(&buffer[0..n]));
                // stream.write(&buffer[0..n]).unwrap();
            },
            Err(e) => {
                eprintln!("Failed to read request -> error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    println!("Starting Rust Redis");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established");
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection -> error: {}", e)
            }
        }
    }
}
