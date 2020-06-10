use std::time::{Instant};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow},
};

use super::engine::{Engine};

pub fn main(title: &str) {
    let (window, event_loop) = Engine::get_init(&title);
    let mut engine = Engine::new(&window);

    let mut new_time = Instant::now();
    let mut old_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        old_time = new_time;
        new_time = Instant::now();

        engine.update(&event, new_time.duration_since(old_time).as_secs_f32());

        match event {
            Event::MainEventsCleared => window.request_redraw(),

            // Window Resized
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                engine.window_resized(size);
            },

            // Window Close
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            // Draw
            Event::RedrawRequested(_) => {
                engine.render(event);         
            },
            _ => (),
        }
    });
}