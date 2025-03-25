mod rplidar;
use crate::rplidar::driver::get_info;

fn main() {
    let mut driver = rplidar::driver::new_connection();
    let read_buf = get_info(&mut driver).expect("Read failed");
    // Print read_buf as hex:
    for byte in &read_buf {
        print!("{:02X} ", byte);
    }
}
