//! # Struct and Trait for the Uart Module
//!
//! This module provides support for Uart communication. It defines the `uart` struct
//! and implements the `uart` trait for opening a communication link.
//!
//! ## Example Usage
//! ```
//! let mut uart = uart::Uart::open("/dev/ttyUSB0")?;
//!     
//! let uart_send_data: u8 = 10;
//! uart.send(&[uart_send_data])?;
//! 
//! let mut uart_receive_data = vec![0u8; 1];
//! uart.recv(&mut uart_receive_data)?;
//! ```
//!


use std::io;
use std::fs::{File, OpenOptions};

pub struct Uart {
    pub path: String,
    pub fd: File,
}

impl Uart {
    pub fn open(devpath: &str) -> io::Result<Uart> {
        let fd = OpenOptions::new().read(true).write(true).open(devpath)?;
        Ok(Uart {
            path: devpath.to_string(),
            fd,
        })
    }

    pub fn name(&self) -> &String {
        &self.path
    }
}
