use renderer_core::start;

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "info", tag = "crusty"))
)]
fn main() {
    start()
}
