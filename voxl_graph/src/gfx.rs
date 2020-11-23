use crate::{
    texture,
    uniforms::Uniforms,
    vertex::Vertex,
    wgpu::{util::DeviceExt, *},
};

use futures::executor::block_on;
use winit::window::Window;

pub struct GFX {
    surface: Surface,
    device: Device,
    queue: Queue,
    //-------------------------
    sc_desc: SwapChainDescriptor,
    render_pipeline: RenderPipeline,
    // Buffers
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    _num_vertices: u32,
    num_indices: u32,
    // Textures
    _diffuse_texture: texture::Texture,
    diffuse_bind_group: BindGroup,
    // Camera
    //uniforms: Uniforms,
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
}

pub const VERTICES: &[Vertex] = &[
    // Top
    Vertex::new([-1., 1., -1.], [0., 0.]), // 0
    Vertex::new([1., 1., -1.], [1., 0.]),  // 1
    Vertex::new([-1., 1., 1.], [0., 1.]),  // 2
    Vertex::new([1., 1., 1.], [1., 1.]),   // 3
    // Bottom
    Vertex::new([-1., -1., -1.], [1., 0.]), // 4 //0
    Vertex::new([1., -1., -1.], [0., 0.]),  // 5 //1
    Vertex::new([-1., -1., 1.], [1., 1.]),  // 6 //2
    Vertex::new([1., -1., 1.], [0., 1.]),   // 7 //3
    // Front
    Vertex::new([-1., 1., 1.], [0., 0.]),  // 8
    Vertex::new([1., 1., 1.], [1., 0.]),   // 9
    Vertex::new([-1., -1., 1.], [0., 1.]), // 10
    Vertex::new([1., -1., 1.], [1., 1.]),  // 11
    // Back
    Vertex::new([-1., 1., -1.], [1., 0.]),  // 12
    Vertex::new([1., 1., -1.], [0., 0.]),   // 13
    Vertex::new([-1., -1., -1.], [1., 1.]), // 14
    Vertex::new([1., -1., -1.], [0., 1.]),  // 15
    // Right
    Vertex::new([1., 1., 1.], [0., 0.]),   // 16
    Vertex::new([1., 1., -1.], [1., 0.]),  // 17
    Vertex::new([1., -1., 1.], [0., 1.]),  // 18
    Vertex::new([1., -1., -1.], [1., 1.]), // 19
    // Left
    Vertex::new([-1., 1., -1.], [0., 0.]),  // 20
    Vertex::new([-1., 1., 1.], [1., 0.]),   // 21
    Vertex::new([-1., -1., -1.], [0., 1.]), // 22
    Vertex::new([-1., -1., 1.], [1., 1.]),  // 23
];

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
    1, 0, 2, 2, 3, 1,
    6, 4, 5, 5, 7, 6,
    9, 8, 10, 10, 11, 9,
    14, 12, 13, 13, 15, 14,
    17, 16, 18, 18, 19, 17,
    21, 20, 22, 22, 23, 21,
];

pub struct Render {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
}

impl Render {
    fn _new(backend: BackendBit, window: &Window) -> Self {
        let instance = Instance::new(backend);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
        }))
        .expect("unable to create adapter");

        let (device, queue) = block_on(adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                shader_validation: true,
            },
            None,
        ))
        .expect("failed to request device");

        Self {
            surface,
            device,
            queue,
        }
    }
}

impl GFX {
    pub fn new(window: &Window) -> Self {
        let window_size = window.inner_size();
        let instance = Instance::new(BackendBit::VULKAN);

        // Surface is where the drawing takes place, in this case,
        // a winit window.
        let surface = unsafe { instance.create_surface(window) };

        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
        }))
        .expect("unable to create adapter");

        let (device, queue) = block_on(adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                shader_validation: true,
            },
            None,
        ))
        .expect("failed to request device");

        let sc_desc = SwapChainDescriptor {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::Fifo,
        };

        //
        let diffuse_bytes = include_bytes!("../../assets/hyper_cube::sand.png");
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, Some("diffuse_texture"))
                .unwrap();

        // LAYOUT
        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStage::FRAGMENT,
                        ty: BindingType::SampledTexture {
                            multisampled: false,
                            dimension: TextureViewDimension::D2,
                            component_type: TextureComponentType::Uint,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStage::FRAGMENT,
                        ty: BindingType::Sampler { comparison: false },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // Actual Bind group
        let diffuse_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&diffuse_texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

        //-----------------------------
        let uniforms = Uniforms::new();

        let uniform_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::VERTEX,
                    ty: BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(uniform_buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });
        //------------------------------------------------------------------------------
        // Render Pipeline -------------------------------------------------------------
        //
        let vs_module =
            device.create_shader_module(include_spirv!("../../shaders/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(include_spirv!("../../shaders/shader.frag.spv"));

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(RasterizationStateDescriptor {
                front_face: FrontFace::Ccw,
                cull_mode: CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.,
                depth_bias_clamp: 0.,
                clamp_depth: false,
            }),
            color_states: &[ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: BlendDescriptor::REPLACE,
                alpha_blend: BlendDescriptor::REPLACE,
                write_mask: ColorWrite::ALL,
            }],
            primitive_topology: PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: VertexStateDescriptor {
                index_format: IndexFormat::Uint16,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let vertex_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsage::INDEX,
        });

        Self {
            surface,
            device,
            queue,
            sc_desc,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            _num_vertices: VERTICES.len() as u32,
            num_indices: INDICES.len() as u32,
            _diffuse_texture: diffuse_texture,
            diffuse_bind_group,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn dump(self) -> (Render, RenderBunch, SwapChainDescriptor) {
        (
            Render {
                surface: self.surface,
                device: self.device,
                queue: self.queue,
            },
            RenderBunch {
                pipeline: self.render_pipeline,
                vertex_buff: self.vertex_buffer,
                index_buff: self.index_buffer,
                uniform_buff: self.uniform_buffer,
                diffuse_bg: self.diffuse_bind_group,
                uniform_bg: self.uniform_bind_group,
                num_indices: self.num_indices,
            },
            self.sc_desc,
        )
    }

    pub fn render(&mut self) {
        let frame = {
            self.device
                .create_swap_chain(&self.surface, &self.sc_desc)
                .get_current_frame()
                .expect("Timeout getting texture")
                .output
        };

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color {
                            r: 0.3, //0.39215686274,
                            g: 0.3, //0.58431372549,
                            b: 0.3, //0.9294117647,
                            a: 1.,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
