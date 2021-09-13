use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
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

    let evt_loop = EventLoop::new();

    let window_size = PhysicalSize::new(1920, 1080);
    let window = WindowBuilder::new()
        .with_title("WGPU Rendering")
        .with_inner_size(window_size)
        .build(&evt_loop)
        .expect("Failed to create window!");

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().expect("Could not get window reference");
        let document = window.document().expect("Could not get document reference");
        let body = document.body().expect("Could not get body reference");

        body.append_child(&canvas)
            .expect("Append canvas to HTML body");
    }

    let mut render_state = State::new(&window).await;
    evt_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() && !render_state.input(event) => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(new_size) => render_state.resize(*new_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                render_state.resize(**new_inner_size)
            }
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        },
        Event::RedrawRequested(_) => {
            render_state.update();
            match render_state.render() {
                Ok(_) => {}
                //On swapchain lost, recreate
                Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size),
                // On OOM Exit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
