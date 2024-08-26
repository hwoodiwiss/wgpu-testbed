use std::sync::Arc;

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::state::State;

mod camera;
mod file_reader;
mod instance;
mod light;
mod model;
mod pipeline;
mod state;
mod texture;
mod uniform;
mod vertex;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn run_wasm() {
    use std::panic;

    panic::set_hook(Box::new(console_error_panic_hook::hook));
    run().await;
}

pub async fn run() {
    env_logger::init();

    let evt_loop = EventLoop::new().expect("Failed to create event loop!");

    let window_size = PhysicalSize::new(1920, 1080);
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("WGPU Rendering")
            .with_inner_size(window_size)
            .build(&evt_loop)
            .expect("Failed to create window!"),
    );

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas().expect("Could not get canvas reference");

        let web_window = web_sys::window().expect("Could not get window reference");
        let document = web_window
            .document()
            .expect("Could not get document reference");
        let body = document.body().expect("Could not get body reference");
        web_sys::console::log_1(
            &format!(
                "Web Window Width: {}, Height: {}",
                web_window.inner_width().unwrap().as_f64().unwrap(),
                web_window.inner_height().unwrap().as_f64().unwrap()
            )
            .as_str()
            .into(),
        );

        body.append_child(&canvas)
            .expect("Append canvas to HTML body");

        canvas
            .set_attribute("style", "width: 100%; aspect-ratio: 16/9;")
            .expect("Set canvas style");

        web_sys::console::log_1(
            &format!(
                "Canvas Window Width: {}, Height: {}",
                canvas.width(),
                canvas.height()
            )
            .as_str()
            .into(),
        );
    }

    let mut render_state = State::new(window.clone()).await;
    evt_loop
        .run(move |event, target| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() && !render_state.input(event) => match event {
                WindowEvent::CloseRequested => target.exit(),
                WindowEvent::Resized(new_size) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        use winit::platform::web::WindowExtWebSys;

                        if (new_size.width > 0 && new_size.height > 0) {
                            let canvas = window.canvas().expect("Could not get canvas reference");
                            canvas
                                .set_attribute("style", "width: 100%; aspect-ratio: auto;")
                                .expect("Set canvas style");
                        }
                    }

                    render_state.resize(*new_size)
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    inner_size_writer: _,
                } => render_state.resize(window.inner_size()),
                WindowEvent::KeyboardInput {
                    event: key_event, ..
                } if key_event.state == ElementState::Pressed => {
                    if let Key::Named(key) = key_event.logical_key {
                        if key == NamedKey::Escape {
                            target.exit()
                        }
                    }

                    render_state.input(event);
                }
                WindowEvent::RedrawRequested => {
                    render_state.update();
                    match render_state.render() {
                        Ok(_) => {}
                        //On swapchain lost, recreate
                        Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size),
                        // On OOM Exit
                        Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {}
            },
            Event::AboutToWait { .. } => window.request_redraw(),
            _ => {}
        })
        .expect("Failed to run event loop!");
}
