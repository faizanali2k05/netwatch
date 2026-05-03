use pcap::{Capture, Device};
use crate::parser;

pub fn start_capture() {
    let device = match Device::lookup() {
        Ok(Some(d)) => d,
        Ok(None) => {
            eprintln!("No device found");
            return;
        }
        Err(e) => {
            eprintln!("Device lookup failed: {}", e);
            return;
        }
    };

    println!("Listening on device: {}", device.name);

    let mut cap = match Capture::from_device(device).and_then(|d| d.promisc(true).open()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open capture: {}", e);
            return;
        }
    };

    while let Ok(packet) = cap.next_packet() {
        parser::parse_packet(packet.data);
    }
}
