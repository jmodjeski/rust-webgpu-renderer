use futures::executor::block_on;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
};

use super::engine::{Engine};

pub fn main(title: &str) {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title(title);
    let window = window_builder.build(&event_loop).unwrap();
    let mut engine_resources = block_on(Engine::init(&window));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window.request_redraw(),

            // Window Resized
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                Engine::recreate_swapchain(&mut engine_resources, size);
            },

            // Draw
            Event::RedrawRequested(_) => {
                Engine::render(&mut engine_resources);
            },

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}