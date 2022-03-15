use log::Level;
use renderer_core::run;
use wasm_bindgen_futures;

fn main() {
    console_log::init_with_level(Level::Warn).expect("Log init failed");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_bindgen_futures::spawn_local(run());
}
