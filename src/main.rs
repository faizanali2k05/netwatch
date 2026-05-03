mod capture;
mod parser;
mod process;
mod ui;
mod utils;

fn main() {
    println!("🚀 NetWatch starting...");
    capture::start_capture();
}
