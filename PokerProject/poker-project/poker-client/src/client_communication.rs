/// communication portion of the GUI

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};

const MSG_SIZE: usize = 1000;

pub struct TcpClient {
    sender: Sender<String>,
    messages: Arc<Mutex<Vec<String>>>,
}

impl TcpClient {
    pub fn new(address: &str) -> Self {
        let messages = Arc::new(Mutex::new(vec![]));
        let (tx, rx) = mpsc::channel::<String>();
        let messages_clone = Arc::clone(&messages);
        let address_clone = address.to_string();

        // spawn a thread for each client
        thread::spawn(move || {
            let mut client = TcpStream::connect(&address_clone).expect("Failed to connect");
            println!("[DEBUG] Connected to server at {}", &address_clone);

            client.set_nonblocking(true).expect("Failed to set non-blocking mode");

            let mut buffer = [0; MSG_SIZE];

            loop {
                // Read messages from server
                match client.read_exact(&mut buffer) {
                    Ok(_) => {
                        let raw_msg = buffer
                            .iter()
                            .take_while(|&&x| x != 0)
                            .cloned()
                            .collect::<Vec<u8>>();
            
                        if !raw_msg.is_empty() {
                            if let Ok(msg) = String::from_utf8(raw_msg) {
                                let trimmed = msg.trim();
                                if !trimmed.is_empty() {
                                    println!("[DEBUG] Received message: {}", trimmed);
                                    messages_clone.lock().unwrap().push(trimmed.to_string());
                                }
                            } else {
                                eprintln!("[WARN] Received invalid UTF-8 message");
                            }
                        }
                    }
                    Err(_) => {}
                }

                // Send user messages to server
                match rx.try_recv() {
                    Ok(msg) => {
                        let mut buff = msg.clone().into_bytes();
                        buff.resize(MSG_SIZE, 0);
                        client.write_all(&buff).expect("Writing to socket failed");
                    }
                    Err(TryRecvError::Empty) => (),
                    Err(TryRecvError::Disconnected) => break,
                }
            }
        });

        Self {
            sender: tx,
            messages,
        }
    }

    pub fn send_message(&self, msg: String) {
        if !msg.is_empty() {
            self.sender.send(msg).expect("Failed to send message");
        }
    }

    pub fn get_messages(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.messages)
    }
}
