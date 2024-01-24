use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    collections::{Hashmap, BinaryHeap},
};

use create::thread_pool::ThreadPool;

pub fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).expect("Failed to read request");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    match request.trim() {
        "set" => {

            let response = "Set request received";
            stream.write(response.as_bytes())?;
        },
        "get" => {
            
            let response = "Get request received";
            stream.write(response.as_bytes())?;
        },
        _ => {
            let response = "Invalid request type";
            stream.write(response.as_bytes())?;
        },
    };
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let _ = handle_client(stream?);
    }
    Ok(())

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             println!("Connection established");
    //             std::thread::spawn(|| handle_tcp_client(stream));
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to establish connection -> error: {}", e)
    //         }
    //     }
    //     // let stream = stream.unwrap();
    // }
}
