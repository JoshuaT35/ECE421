//! # Struct and Trait for the Spi Module
//!
//! This module provides support for SPI communication. It defines the `Spi` struct
//! and implements the `Spi` trait for opening a communication link.
//!
//! ## Example Usage
//! ```
//! let mut spi = spi::Spi::open("/dev/spi0.0")?;
//! 
//! let spi_send_data: u8 = 10;
//! spi.send(&[spi_send_data])?;

//! let mut spi_receive_data = vec![0u8; 1];
//! spi.recv(&mut spi_receive_data)?;
//! ```
//!


use std::io;
use spidev::Spidev;

pub struct Spi {
    pub path: String,
    pub device: Spidev,
}

impl Spi {
    pub fn open(devpath: &str) -> io::Result<Spi> {
        let device = Spidev::open(devpath)?;
        Ok(Spi {
            path: devpath.to_string(),
            device,
        })
    }

    pub fn name(&self) -> &String {
        &self.path
    }
}
      