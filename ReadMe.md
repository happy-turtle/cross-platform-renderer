# Cross-Platform Renderer

A cross platform renderer using wgpu, written in rust. A proof of concept to compile and run on desktop, mobile and the web in one project. 

## Current status

| Platform | Compilation        | BG Clear           | Triangle           |
| -------- | ------------------ | ------------------ | ------------------ |
| Desktop  | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| Web      | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| Android  | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |

![A red triangle rendered on blue background](screenshot.png)

## Requirements

For desktop platforms nothing special aside from the standard rust toolchain is needed.

### Android

Install AndroidSDK and NDK and setup environment variables (`ANDROID_SDK_ROOT` and `ANDROID_NDK_ROOT`).

Install cargo-apk with `cargo install cargo-apk`.

Add desired android targets with `rustup target add <triple>`.

### Web

Install trunk with `cargo install trunk`

Add wasm target with `rustup target add wasm32-unknown-unknown`

## Build instructions

### Desktop

Run `cargo run`. By default the desktop crate is build.

### Android

Run `cargo apk run -p android-build` optionally with the flag `--target <triple>` for explicit target selection.

### Web - WebGL2 Backend

Run `cd bin/web/`

Run `trunk serve`

Trunk is now serving app under `http://localhost:8080`

## Using WebGPU as the Web Backend

Change the limits in the device descriptor in `core/src/lib.rs` from `wgpu::Limits::downlevel_webgl2_defaults()` to `wgpu::Limits::default()`.

Remove the `webgl` feature from `core/Cargo.toml`.

Set the `--cfg=web_sys_unstable_apis` rust flag.
This can be done by setting `RUSTFLAGS=--cfg=web_sys_unstable_apis`, or by creating a `.cargo/config` file with
```
[build]
rustflags = [
    "--cfg=web_sys_unstable_apis"
]
```