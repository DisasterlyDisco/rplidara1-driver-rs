use serialport::{Error, SerialPort};

pub fn get_info(port: &mut Box<dyn SerialPort>) -> Result<[u8; 27], Error> {
    const MSG: [u8; 2] = [0xA5, 0x50];
    let mut read_buf: [u8; 27] = [0; 27];
    port.write(&MSG)?;
    port.read(&mut read_buf)?;
    Ok(read_buf)
}
