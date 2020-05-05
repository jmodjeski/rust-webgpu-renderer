use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow},
};

use super::engine::{Engine};

pub fn main(title: &str) {
    let (window, event_loop) = Engine::get_init(&title);
    let mut engine = Engine::new(&window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window.request_redraw(),

            // Window Resized
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                engine.window_resized(size);
            },

            // Draw
            Event::RedrawRequested(_) => {
                engine.render(event);         
            },

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}