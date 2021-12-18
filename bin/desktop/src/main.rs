use env_logger::Builder;
use log::LevelFilter;
use renderer_core::start;

fn main() {
    Builder::new().filter_level(LevelFilter::Warn).init();
    start()
}
