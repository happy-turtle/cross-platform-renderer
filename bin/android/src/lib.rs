use renderer_core::create_window;

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "info", tag = "rust3d"))
)]
fn main() {
    create_window()
}
