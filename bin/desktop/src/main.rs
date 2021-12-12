use env_logger::Builder;
use log::LevelFilter;
use renderer_core::create_window;

fn main() {
    Builder::new().filter_level(LevelFilter::Warn).init();
    create_window()
}
