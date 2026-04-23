/// contains functions for server communication using Tcp Protocol

use std::{
    io::{Read, Write},
    net::{TcpStream},
};

const MSG_SIZE: usize = 1000;

/// waits (loops) for a socket until a message is received
pub fn read_message_wait(socket: &mut TcpStream) -> String {
    // --- loop until we receive a response ---
    let message: String = loop {
        match read_message(socket) {
            Some(client_input) if !client_input.trim().is_empty() => break client_input,
            _ => continue,
        }
    };
    return message;
}

// read a message from a client socket
pub fn read_message(socket: &mut TcpStream) -> Option<String> {
    // buffer to read a message
    let mut buff: Vec<u8> = vec![0; MSG_SIZE];

    match socket.read_exact(&mut buff) {
        Ok(_) => {
            // remove whitespace from message
            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();

            //convert to string
            String::from_utf8(msg).ok()
        }
        Err(_) => None,
    }
}

// write to a client by getting their socket
pub fn write_message(socket: &mut TcpStream, msg: &str) -> Result<(), std::io::Error> {
    let mut buff = msg.to_string().into_bytes();
    buff.resize(MSG_SIZE, 0);
    socket.write_all(&buff)?;
    Ok(())
}
