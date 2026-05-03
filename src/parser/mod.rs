use etherparse::SlicedPacket;

pub fn parse_packet(data: &[u8]) {
    if let Ok(packet) = SlicedPacket::from_ethernet(data) {
        if let Some(ip) = packet.ip {
            match ip {
                etherparse::InternetSlice::Ipv4(header, _) => {
                    let src = header.source_addr();
                    let dst = header.destination_addr();

                    if let Some(transport) = packet.transport {
                        match transport {
                            etherparse::TransportSlice::Tcp(tcp) => {
                                let src_port = tcp.source_port();
                                let dst_port = tcp.destination_port();
                                println!("🌐 {}:{} -> {}:{} (TCP)", src, src_port, dst, dst_port);
                            }
                            etherparse::TransportSlice::Udp(udp) => {
                                let src_port = udp.source_port();
                                let dst_port = udp.destination_port();
                                println!("🌐 {}:{} -> {}:{} (UDP)", src, src_port, dst, dst_port);
                            }
                            _ => {
                                println!("🌐 {} -> {} (non-TCP/UDP)", src, dst);
                            }
                        }
                    } else {
                        println!("🌐 {} -> {} (no transport)", src, dst);
                    }
                }
                _ => {}
            }
        }
    }
}
