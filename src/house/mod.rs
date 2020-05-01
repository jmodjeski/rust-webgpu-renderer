use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
};

use super::engine::{Engine, create_swapchain_description};

pub async fn main(title: &str) {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title(title);
    let window = window_builder.build(&event_loop).unwrap();
    let mut i_engine = Engine::init(&window).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window.request_redraw(),

            // Window Resized
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                let swapchain_description = create_swapchain_description(size);
                i_engine.swapchain = i_engine.device.create_swap_chain(&i_engine.surface, &swapchain_description);
            },

            // Draw
            Event::RedrawRequested(_) => {
                let frame = i_engine.swapchain.get_next_texture().expect("Timeout when aquiring next swapchain texture");
                let mut encoder = i_engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: None,
                });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color::BLACK,
                        }],
                        depth_stencil_attachment: None,
                    });

                    render_pass.set_pipeline(&i_engine.pipeline);
                    render_pass.set_bind_group(0, &i_engine.bind_group, &[]);
                    render_pass.draw(0..3, 0..1);
                }

                i_engine.queue.submit(&[encoder.finish()]);
            },

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}