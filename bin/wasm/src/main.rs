use log::Level;
use renderer_core::create_window;

fn main() {
    console_log::init_with_level(Level::Warn);
    create_window()
}
