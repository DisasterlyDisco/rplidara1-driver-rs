mod rplidar;
use crate::rplidar::driver::get_info;

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
