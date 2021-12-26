use log::Level;
use renderer_core::start;

fn main() {
    console_log::init_with_level(Level::Warn).expect("Log init failed");
    start();
}
