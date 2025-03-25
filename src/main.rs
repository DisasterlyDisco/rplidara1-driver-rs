use serialport::{Error, SerialPort};

fn get_info(port: &mut Box<dyn SerialPort>) -> Result<[u8; 27], Error> {
    const MSG: [u8; 2] = [0xA5, 0x50];
    let mut read_buf: [u8; 27] = [0; 27];
    port.write(&MSG)?;
    port.read(&mut read_buf)?;
    Ok(read_buf)
}

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 115200)
        .open()
        .expect("Unable to open");
    port.set_timeout(std::time::Duration::from_secs(10))
        .expect("Unable to set timeout");
    let read_buf = get_info(&mut port).expect("Read failed");
    // Print read_buf as hex:
    for byte in &read_buf {
        print!("{:02X} ", byte);
    }
}
