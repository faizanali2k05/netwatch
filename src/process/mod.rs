// Process mapping will live here. For Phase 1 we provide a stub.
pub fn map_port_to_process(_ip: &str, _port: u16) -> Option<String> {
    // On Linux we'll later implement ss parsing to map port -> pid -> exe
    None
}
