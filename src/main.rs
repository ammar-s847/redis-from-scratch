use std::{
    thread,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    collections::{btree_set::Union, HashMap},
};


enum StorageMethod {
    Hashmap,
    AVLTree,
    TimeSeries,
    RBTree,
}

enum PersistType {
    NoSnapshot,
    AppendOnlyFile,
}


struct Storage<T> {
    data: T,
    default: bool,
    persist_type: PersistType,
    storage_method: StorageMethod,
}

impl Storage<HashMap<String, String>> {
    fn new() -> Storage<HashMap<String, String>> {
        Storage {
            data: HashMap::new(),
            default: true,
            persist_type: PersistType::NoSnapshot,
            storage_method: StorageMethod::Hashmap,
        }
    }

    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn del(&mut self, key: &str) {
        self.data.remove(key);
    }
}


pub fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                match String::from_utf8_lossy(&buffer[0..1]).trim() {
                    "0" => { // Ping
                        let response = "PONG";
                        stream.write(response.as_bytes()).unwrap();
                    }
                    "1" => { // Set
                        match String::from_utf8_lossy(&buffer[1..n])
                        .trim()
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .as_slice() {
                            [key, value] => {
                                let response = format!(
                                    "Set request received -> key: {} value: {}", 
                                    key,
                                    value
                                );
                                stream.write(response.as_bytes()).unwrap();
                            },
                            _ => {
                                let response = "Invalid set request";
                                stream.write(response.as_bytes()).unwrap();
                            }
                        }
                    },
                    "2" => { // Get
                        match String::from_utf8_lossy(&buffer[1..n])
                        .trim() {
                            key => {
                                let response = format!(
                                    "Get request received -> key: {}", 
                                    key
                                );
                                stream.write(response.as_bytes()).unwrap();
                            },
                            _ => {
                                let response = "Invalid get request";
                                stream.write(response.as_bytes()).unwrap();
                            }
                        }
                    },
                    "3" => { // Del
                        match String::from_utf8_lossy(&buffer[1..n])
                        .trim() {
                            key => {
                                let response = format!(
                                    "Del request received -> key: {}", 
                                    key
                                );
                                stream.write(response.as_bytes()).unwrap();
                            },
                            _ => {
                                let response = "Invalid del request";
                                stream.write(response.as_bytes()).unwrap();
                            }
                        }
                    },
                    _ => {
                        let response = "Invalid request type";
                        stream.write(response.as_bytes()).unwrap();
                    },
                };
            },
            Err(e) => {
                eprintln!("Failed to read request -> error: {}", e);
                break;
            }
        }
    }
}

fn do_set(key: &str, value: &str) -> Result<(), &'static str> {
    
    Ok(())
}

fn do_get(key: &str) -> Result<(), &'static str> {

    Ok(())
}

fn do_del(key: &str) -> Result<(), &'static str> {
    Ok(())
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
