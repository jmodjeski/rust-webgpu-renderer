use std::convert::TryInto;
use futures::executor::block_on;
use zerocopy::AsBytes;
use winit::{
    window::{Window, WindowBuilder},
    event::{Event},
    event_loop::{EventLoop},
    dpi::{PhysicalSize}
};
use glm::{mat4, Matrix4};

mod types;

const CLEAR_COLOR: wgpu::Color = wgpu::Color::BLACK;
const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

// PROJECTION/CAMERA
const F_NEAR: f32 = 0.1;
const F_FAR: f32 = 100.0;
const F_FOV: f32 = 95.0;

pub struct Engine {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    swapchain: wgpu::SwapChain,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_buffer_len: u32,
}

impl Engine {
    fn create_verticies() -> (Vec<types::Vertex>, Vec<u16>) {
        ([
            // top
            types::Vertex::new(-1.0, -1.0, 3.0, 1.0, 0.0, 0.0),  // 0
            types::Vertex::new(1.0, -1.0, 3.0, 0.0, 1.0, 0.0),   // 1
            types::Vertex::new(1.0, 1.0, 3.0, 0.0, 0.0, 1.0),    // 2
            types::Vertex::new(-1.0, 1.0, 3.0, 0.0, 0.0, 1.0),   // 3
            // bottom
            types::Vertex::new(-1.0, -1.0, 2.0, 1.0, 0.0, 0.0), // 4
            types::Vertex::new(1.0, -1.0, 2.0, 0.0, 1.0, 0.0),  // 5
            types::Vertex::new(1.0, 1.0, 2.0, 0.0, 0.0, 1.0),   // 6
            types::Vertex::new(-1.0, 1.0, 2.0, 0.0, 0.0, 1.0),  // 7
            // right
            types::Vertex::new(1.0, -1.0, 2.0, 1.0, 0.0, 0.0),  // 8
            types::Vertex::new(1.0, 1.0, 2.0, 0.0, 1.0, 0.0),   // 9
            types::Vertex::new(1.0, 1.0, 3.0, 0.0, 0.0, 1.0),    // 10
            types::Vertex::new(1.0, -1.0, 3.0, 0.0, 0.0, 1.0),   // 11
            // left
            types::Vertex::new(-1.0, -1.0, 3.0, 0.0, 0.0, 1.0),  // 12
            types::Vertex::new(-1.0, 1.0, 3.0, 0.0, 1.0, 0.0),   // 13
            types::Vertex::new(-1.0, 1.0, 2.0, 0.0, 0.0, 1.0),  // 14
            types::Vertex::new(-1.0, -1.0, 2.0, 1.0, 0.0, 0.0), // 15
            // front
            types::Vertex::new(1.0, 1.0, 2.0, 0.0, 0.0, 1.0),   // 16
            types::Vertex::new(-1.0, 1.0, 2.0, 0.0, 0.0, 1.0),  // 17
            types::Vertex::new(-1.0, 1.0, 3.0, 0.0, 1.0, 0.0),   // 18
            types::Vertex::new(1.0, 1.0, 3.0, 1.0, 0.0, 0.0),    // 19
            // back
            types::Vertex::new(1.0, -1.0, 3.0, 0.0, 0.0, 1.0),   // 20
            types::Vertex::new(-1.0, -1.0, 3.0, 0.0, 0.0, 1.0),  // 21
            types::Vertex::new(-1.0, -1.0, 2.0, 0.0, 1.0, 0.0), // 22
            types::Vertex::new(1.0, -1.0, 2.0, 1.0, 0.0, 0.0),  // 23
            
        ].to_vec(),
            [
                0, 1, 2, 2, 3, 0, // top
                4, 5, 6, 6, 7, 4, // bottom
                8, 9, 10, 10, 11, 8, // right
                12, 13, 14, 14, 15, 12, // left
                16, 17, 18, 18, 19, 16, // front
                20, 21, 22, 22, 23, 20, // back
            ].to_vec()
        )
    }

    pub fn get_init(title: &str) -> (Window, EventLoop<()>) {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new().with_title(title);
        let window = window_builder.build(&event_loop).unwrap();

        (window, event_loop)
    }

    async fn get_adapter(surface: &wgpu::Surface) -> wgpu::Adapter {
        wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(surface),
            },
            wgpu::BackendBit::PRIMARY
        ).await.unwrap()
    }

    async fn get_device_queue(adapter: wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        }).await
    }

    pub fn new(window: &Window) -> Engine {
        let size = window.inner_size();
        let surface = wgpu::Surface::create(window);

        let adapter = block_on(Engine::get_adapter(&surface));
        let (device, queue) = block_on(Engine::get_device_queue(adapter));

        let (verticies, indicies) = Engine::create_verticies();
        let vertex_buffer = device.create_buffer_with_data(verticies.as_bytes(), wgpu::BufferUsage::VERTEX);
        let index_buffer = device.create_buffer_with_data(indicies.as_bytes(), wgpu::BufferUsage::INDEX);

        let mv_matrix = Engine::create_mvp_matrix(size.width as f32/ size.height as f32);
        let mv_data: &[f32; 16] = &matrix4_to_array(mv_matrix);
        let uniform_buffer = device.create_buffer_with_data(mv_data.as_bytes(), wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

        let vs = include_bytes!("../../compiled_shaders/shader.vert.spv");
        let vs_module = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vs[..])).unwrap());

        let fs = include_bytes!("../../compiled_shaders/shader.frag.spv");
        let fs_module = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fs[..])).unwrap());

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                }
            ],
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        range: 0..64,
                    }
                }
            ],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let vertex_buffer_descriptors = &[
            Engine::create_vertex_buffer(types::VERTEX_SIZE as wgpu::BufferAddress)
        ];

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
                format: TEXTURE_FORMAT,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: vertex_buffer_descriptors,
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
            swapchain: swapchain,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            index_buffer_len: indicies.len() as u32,
        }
    }

    pub fn render(&mut self, _event: Event<()>) {
        let frame = self.swapchain.get_next_texture().expect("Timeout when aquiring next swapchain texture");
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: CLEAR_COLOR,
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_index_buffer(&self.index_buffer, 0, 0);
            render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
            render_pass.draw_indexed(0..self.index_buffer_len, 0, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }

    pub fn window_resized(&mut self, size: PhysicalSize<u32>) {
        self.recreate_swapchain(size);
    }

    fn recreate_swapchain(&mut self, size: PhysicalSize<u32>) {
        let swapchain_description = create_swapchain_description(size);
        self.swapchain = self.device.create_swap_chain(&self.surface, &swapchain_description);
    }

    fn create_vertex_buffer<'a>(size: wgpu::BufferAddress) -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: size,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {       // position in vec2
                    format: wgpu::VertexFormat::Float3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float3,
                    offset: 4 * 3,
                    shader_location: 1,
                }
            ]
        }
    }

    fn create_mvp_matrix(aspect_ratio: f32) -> Matrix4<f32> {
        let fov_rad = F_FOV * 0.5 / 180.0 * std::f32::consts::PI;
        let fov = 1.0 / fov_rad.tan();
        let x = aspect_ratio * fov;
        let y = fov;
        let z = F_FAR / (F_FAR - F_NEAR);
        let w = (-F_FAR * F_NEAR) / (F_FAR - F_NEAR);
        println!("aspect ratio: {}", aspect_ratio);
        println!("fov: {}", fov);
        println!("x: {}, y: {}, z: {}", x, y, z);
        let projection = mat4(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 1.0,
            0.0, 0.0, w, 0.0,
        );
        let view = mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.3, 0.5, 0.0, 1.0,
        );
        // let opengl_fix = mat4(
        //     1.0, 0.0, 0.0, 0.0,
        //     0.0, 1.0, 0.0, 0.0,
        //     0.0, 0.0, 0.5, 0.0,
        //     0.0, 0.0, 0.5, 1.0,
        // );

        projection * view // * opengl_fix
    }
}

pub fn create_swapchain_description (size: PhysicalSize<u32>) -> wgpu::SwapChainDescriptor {
    wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: TEXTURE_FORMAT,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    }
}

fn matrix4_to_array(mat: Matrix4<f32>) -> [f32; 16] {
    let vecs = mat.as_array();
    let mut vals: Vec<[f32; 4]> = Vec::new();
    for v in vecs.iter() {
        vals.push(v.as_array().clone());
    }
    vals.concat()[..].try_into().expect("slice with incorrect length")
}

// impl<'a, T> TryFrom<&'a [T]> for &'a [T; $N] 
//     type Error = TryFromSliceError;
// impl<'a, T: Copy> TryFrom<&'a [T]> for &'a [T; $N] 
//     type Error = TryFromSliceError;

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