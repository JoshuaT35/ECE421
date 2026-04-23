//! # SerialIO Documentation
//! This is the documentation for the serialio library.
//! 
//! ## Usage
//! Depend on `serialio` in `Cargo.toml`.

use std::io;
use std::io::{Read, Write};
use std::os::fd::{AsFd, AsRawFd};
use nix::unistd::{read, write};


// module declaration
pub mod uart;
pub mod spi;

pub trait SendRecv {
    fn send(&mut self, data: &[u8]) -> io::Result<usize>;

    fn recv(&mut self, data: &mut [u8]) -> io::Result<usize>;
}

impl SendRecv for uart::Uart {
    fn send(&mut self, data: &[u8]) -> io::Result<usize> {
        Ok(write(self.fd.as_fd(), data)?)
    }

    fn recv(&mut self, data: &mut [u8]) -> io::Result<usize> {
        Ok(read(self.fd.as_raw_fd(), data)?)
    }
}

impl SendRecv for spi::Spi {
    fn send(&mut self, data: &[u8]) -> io::Result<usize> {
        Ok(self.device.write(data)?)
    }

    fn recv(&mut self, data: &mut [u8]) -> io::Result<usize> {
        Ok(self.device.read(data)?)
    }
}
