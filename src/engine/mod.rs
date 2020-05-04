// use std::mem::ManuallyDrop;
use winit::{
    window::{Window},
    dpi::{PhysicalSize}
};


pub struct Engine {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
    pub swapchain: wgpu::SwapChain,
}

impl Engine {
    pub async fn init(window: &Window) -> Engine {
        let size = window.inner_size();
        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY
        ).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        }).await;

        let vs = include_bytes!("../../compiled_shaders/shader.vert.spv");
        let vs_module = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vs[..])).unwrap());

        let fs = include_bytes!("../../compiled_shaders/shader.frag.spv");
        let fs_module = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fs[..])).unwrap());

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[],
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let swapchain_description = create_swapchain_description(size);

        let swapchain = device.create_swap_chain(&surface, &swapchain_description);

        Engine {
            surface: surface,
            device: device,
            queue: queue,
            bind_group: bind_group,
            pipeline: render_pipeline,
            swapchain: swapchain
        }
    }

    pub fn render(engine: &mut Engine) {
        let frame = engine.swapchain.get_next_texture().expect("Timeout when aquiring next swapchain texture");
        let mut encoder = engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

            render_pass.set_pipeline(&engine.pipeline);
            render_pass.set_bind_group(0, &engine.bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        engine.queue.submit(&[encoder.finish()]);
    }

    pub fn recreate_swapchain(engine: &mut Engine, size: PhysicalSize<u32>) {
        let swapchain_description = create_swapchain_description(size);
        engine.swapchain = engine.device.create_swap_chain(&engine.surface, &swapchain_description);
    }
}


// Only needed if resources are not disposed of in wgpu
// struct EngineResourceHolder(ManuallyDrop<Engine>);

// impl Drop for EngineResourceHolder {
//     fn drop(&mut self) {
//         let Engine {
//             surface,
//             device,
//             queue,
//             bind_group,
//             pipeline,
//             swapchain,
//         } = ManuallyDrop::take(&mut self.0);

//         device.
//     }
// }

pub fn create_swapchain_description (size: PhysicalSize<u32>) -> wgpu::SwapChainDescriptor {
    wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    }
}