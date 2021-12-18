use std::iter;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn start() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let window = match WindowBuilder::new().build(&event_loop) {
        Ok(window) => window,
        Err(err) => panic!("{}", err),
    };

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        body.append_child(&canvas)
            .expect("Append canvas to HTML body");
    }

    // The instance is a handle to our GPU
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::Backends::all());

    // Wait for Resumed event on Android; the surface is only needed early to
    // find an adapter that can render to this surface.
    let mut surface = if cfg!(target_os = "android") {
        None
    } else {
        Some(unsafe { instance.create_surface(&window) })
    };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: surface.as_ref(),
            force_fallback_adapter: false,
        })
        .await
        .expect("adapter request failed");

    let (device, queue) = match adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
    {
        Ok(device_queue) => device_queue,
        Err(e) => panic!("{:?}", e),
    };

    let preferred_format = if let Some(surface) = &surface {
        surface.get_preferred_format(&adapter).unwrap()
    } else {
        // if Surface is none, we're guaranteed to be on android
        wgpu::TextureFormat::Rgba8UnormSrgb
    };

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: preferred_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Fifo,
    };
    if let Some(surface) = &mut surface {
        surface.configure(&device, &config);
    }

    event_loop.run(move |event, _, control_flow| match event {
        Event::Resumed => {
            surface = Some(unsafe { instance.create_surface(&window) });

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: preferred_format,
                width: window.inner_size().width,
                height: window.inner_size().height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            if let Some(surface) = &surface {
                surface.configure(&device, &config);
            }
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !input(event) {
                // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        if physical_size.width > 0 && physical_size.height > 0 {
                            config.width = physical_size.width;
                            config.height = physical_size.height;
                            if surface.is_some() {
                                surface.as_ref().unwrap().configure(&device, &config);
                            }
                        }
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        if new_inner_size.width > 0 && new_inner_size.height > 0 {
                            config.width = new_inner_size.width;
                            config.height = new_inner_size.height;
                            if surface.is_some() {
                                surface.as_ref().unwrap().configure(&device, &config);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(_) => {
            update();
            match render(&surface, &device, &queue) {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => {
                    if window.inner_size().width > 0 && window.inner_size().height > 0 {
                        config.width = window.inner_size().width;
                        config.height = window.inner_size().height;
                        if surface.is_some() {
                            surface.as_ref().unwrap().configure(&device, &config);
                        }
                    }
                }
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::RedrawEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    });
}

// fn resize(
//     surface: &Option<wgpu::Surface>,
//     config: &wgpu::SurfaceConfiguration,
//     device: &wgpu::Device,
//     new_size: winit::dpi::PhysicalSize<u32>,
// ) {
//     if new_size.width > 0 && new_size.height > 0 {
//         config.width = new_size.width;
//         config.height = new_size.height;
//         if surface.is_some() {
//             surface.as_ref().unwrap().configure(&device, &config);
//         }
//     }
// }

#[allow(unused_variables)]
fn input(event: &WindowEvent) -> bool {
    false
}

fn update() {}

fn render(
    surface: &Option<wgpu::Surface>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<(), wgpu::SurfaceError> {
    if surface.is_none() {
        return Err(wgpu::SurfaceError::Lost);
    }
    let output = surface.as_ref().unwrap().get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
    }

    queue.submit(iter::once(encoder.finish()));
    output.present();

    Ok(())
}
