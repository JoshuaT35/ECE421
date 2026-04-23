use serialio::*;

fn main() -> std::io::Result<()> {
    // --- uart test start ---
    // handle Result properly with `?`
    let mut uart = uart::Uart::open("/dev/ttyUSB0")?;
    
    let uart_send_data: u8 = 10;
    uart.send(&[uart_send_data])?;

    let mut uart_receive_data = vec![0u8; 1];
    uart.recv(&mut uart_receive_data)?;
    // --- uart test end ---


    // --- spi test start ---
    // handle Result properly with `?`
    let mut spi = spi::Spi::open("/dev/spi0.0")?;
    
    let spi_send_data: u8 = 10;
    spi.send(&[spi_send_data])?;

    let mut spi_receive_data = vec![0u8; 1];
    spi.recv(&mut spi_receive_data)?;
    // --- spi test end ---

    // no error
    Ok(())
}
