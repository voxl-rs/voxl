use super::{
    instance,
    model::*,
    texture,
    uniforms::Uniforms,
    vertex::{TexVertex, Vertex},
};

use cgmath::{Quaternion, Vector3};
use wgpu::{util::DeviceExt, *};
use winit::{dpi::PhysicalSize, window::Window};

pub const VERTICES_FACE: &[TexVertex] = &[
    // Top
    TexVertex::new([-0.5, 0.5, -0.5], [0., 0.]), // 0
    TexVertex::new([0.5, 0.5, -0.5], [1., 0.]),  // 1
    TexVertex::new([-0.5, 0.5, 0.5], [0., 1.]),  // 2
    TexVertex::new([0.5, 0.5, 0.5], [1., 1.]),   // 3
];

pub const INDICE_FACE: &[u16] = &[1, 0, 2, 2, 3, 1];

pub const VERTICES: &[TexVertex] = &[
    // Top
    TexVertex::new([-0.5, 0.5, -0.5], [0., 0.]), // 0
    TexVertex::new([0.5, 0.5, -0.5], [1., 0.]),  // 1
    TexVertex::new([-0.5, 0.5, 0.5], [0., 1.]),  // 2
    TexVertex::new([0.5, 0.5, 0.5], [1., 1.]),   // 3
    // Bottom
    TexVertex::new([-0.5, -0.5, -0.5], [1., 0.]), // 4
    TexVertex::new([0.5, -0.5, -0.5], [0., 0.]),  // 5
    TexVertex::new([-0.5, -0.5, 0.5], [1., 1.]),  // 6
    TexVertex::new([0.5, -0.5, 0.5], [0., 1.]),   // 7
    // Front
    TexVertex::new([-0.5, 0.5, 0.5], [0., 0.]),  // 8
    TexVertex::new([0.5, 0.5, 0.5], [1., 0.]),   // 9
    TexVertex::new([-0.5, -0.5, 0.5], [0., 1.]), // 10
    TexVertex::new([0.5, -0.5, 0.5], [1., 1.]),  // 11
    // Back
    TexVertex::new([-0.5, 0.5, -0.5], [1., 0.]),  // 12
    TexVertex::new([0.5, 0.5, -0.5], [0., 0.]),   // 13
    TexVertex::new([-0.5, -0.5, -0.5], [1., 1.]), // 14
    TexVertex::new([0.5, -0.5, -0.5], [0., 1.]),  // 15
    // Right
    TexVertex::new([0.5, 0.5, 0.5], [0., 0.]),   // 16
    TexVertex::new([0.5, 0.5, -0.5], [1., 0.]),  // 17
    TexVertex::new([0.5, -0.5, 0.5], [0., 1.]),  // 18
    TexVertex::new([0.5, -0.5, -0.5], [1., 1.]), // 19
    // Left
    TexVertex::new([-0.5, 0.5, -0.5], [0., 0.]),  // 20
    TexVertex::new([-0.5, 0.5, 0.5], [1., 0.]),   // 21
    TexVertex::new([-0.5, -0.5, -0.5], [0., 1.]), // 22
    TexVertex::new([-0.5, -0.5, 0.5], [1., 1.]),  // 23
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

#[derive(Debug)]
pub struct RenderBunch {
    pub pipeline: RenderPipeline, //
    pub diffuse_bg: BindGroup,    //
    pub uniform_bg: BindGroup,    //
    pub uniform_buff: Buffer,
    pub vertex_buff: Buffer, //
    pub index_buff: Buffer,  //
    pub instance_buff: Buffer,
    pub num_indices: u32, //
}

#[derive(Debug)]
pub struct Render {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
}

pub fn swap_chain(window_size: &PhysicalSize<u32>) -> SwapChainDescriptor {
    SwapChainDescriptor {
        usage: TextureUsage::OUTPUT_ATTACHMENT,
        format: TextureFormat::Bgra8UnormSrgb,
        width: window_size.width,
        height: window_size.height,
        present_mode: PresentMode::Mailbox,
    }
}

impl Render {
    pub fn new(backend: BackendBit, window: &Window) -> Self {
        use futures::executor::block_on;

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

    pub fn init_buffer<A: Default + bytemuck::Pod>(
        &self,
        label: Option<&'static str>,
        usage: BufferUsage,
    ) -> Buffer {
        self.device.create_buffer_init(&util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&[A::default()]),
            usage,
        })
    }

    pub fn bunch(&self, sc_desc: &SwapChainDescriptor) -> RenderBunch {
        let diffuse_bytes = include_bytes!("../../assets/hyper_cube_magma2.png");
        let diffuse_texture = texture::Texture::from_bytes(
            &self.device,
            &self.queue,
            diffuse_bytes,
            Some("diffuse_texture"),
        )
        .unwrap();

        // LAYOUT
        let texture_bind_group_layout =
            self.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
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
        let diffuse_bg = self.device.create_bind_group(&BindGroupDescriptor {
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

        let uniform_buff = self.init_buffer::<Uniforms>(
            Some("Uniform Buffer"),
            BufferUsage::UNIFORM | BufferUsage::COPY_DST,
        );

        let uniform_bind_group_layout =
            self.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        let uniform_bg = self.device.create_bind_group(&BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(uniform_buff.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });

        // Render Pipeline -------------------------------------------------------------
        let vs_module = self
            .device
            .create_shader_module(include_spirv!("../../shaders/shader.vert.spv"));

        let fs_module = self
            .device
            .create_shader_module(include_spirv!("../../shaders/shader.frag.spv"));

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
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
                depth_stencil_state: Some(DepthStencilStateDescriptor {
                    format: texture::Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less, // 1.
                    stencil: wgpu::StencilStateDescriptor::default(), // 2.
                }),
                vertex_state: VertexStateDescriptor {
                    index_format: IndexFormat::Uint16,
                    vertex_buffers: &[
                        TexVertex::vb_desc(),
                        instance::InstanceRaw::vb_desc(),
                        //ModelVertex::vb_desc(),
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        let vertex_buff = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsage::VERTEX,
        });

        let index_buff = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsage::INDEX,
        });

        let instance_data: Vec<instance::InstanceRaw> = create_chunk()
            .iter()
            .map(|i| instance::InstanceRaw::from(*i))
            .collect();

        let instance_buff = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: BufferUsage::VERTEX,
        });

        RenderBunch {
            pipeline,
            vertex_buff,
            index_buff,
            uniform_buff,
            instance_buff,
            diffuse_bg,
            uniform_bg,
            num_indices: INDICES.len() as u32,
        }
    }
}

use crate::chunk::{Accessor, Chunk};
fn create_chunk() -> Vec<instance::Instance> {
    const SEED: f64 = 347_510_572.;
    const SMOOTHING: f64 = 0.05;
    use noise::NoiseFn;
    let noise = noise::OpenSimplex::new();

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    struct Dimensions;
    impl Accessor for Dimensions {
        const SIDE_LEN: usize = 16;
    }

    let mut c: Chunk<Dimensions, bool, 4096> = Chunk::default();

    for i in 0..4096 {
        let [y, x, z] = Dimensions::from_index(i);
        print!("y: {}, x: {}, z: {} ", y, x, z);
        let val = noise
            .get([
                y as f64 * SMOOTHING,
                x as f64 * SMOOTHING,
                z as f64 * SMOOTHING,
                SEED,
            ])
            .abs();

        println!("val: {}", val);

        if val < 0.03 {
            c[[y, x, z]] = true;
        }
    }

    let mut res: Vec<instance::Instance> = Vec::with_capacity(4096);
    for i in 0..4096 {
        let [y, x, z] = Dimensions::from_index(i);

        if c[[y, x, z]] {
            res.push(instance::Instance {
                position: Vector3::new(x as f32, y as f32, z as f32),
                rotation: quat_identity(),
            });
            /*
            if y == 0 {
                if !c[[y + 1, x, z]] {
                    res.push(bottom([y, x, z]));
                }
                res.push(top([y + 1, x, z]));
            } else if y == 15 {
                if !c[[y - 1, x, z]] {
                    res.push(top([y, x, z]));
                }
            } else {
                if !c[[y + 1, x, z]] {
                    res.push(bottom([y, x, z]));
                }
                if !c[[y - 1, x, z]] {
                    res.push(top([y, x, z]));
                }
            }

            if x == 0 {
                if !c[[y, x + 1, z]] {
                    res.push(right([y, x, z]));
                }
            } else if x == 15 {
                if !c[[y, x - 1, z]] {
                    res.push(left([y, x, z]));
                }
            } else {
                if !c[[y, x + 1, z]] {
                    res.push(right([y, x, z]));
                }
                if !c[[y, x - 1, z]] {
                    res.push(left([y, x, z]));
                }
            }

            if z == 0 {
                if !c[[y, x, z + 1]] {
                    res.push(front([y, x, z]));
                }
            } else if z == 15 {
                if !c[[y, x, z - 1]] {
                    res.push(back([y, x, z]));
                }
            } else {
                if !c[[y, x, z + 1]] {
                    res.push(front([y, x, z]));
                }
                if !c[[y, x, z - 1]] {
                    res.push(back([y, x, z]));
                }
            }
            */
        }
    }

    log::info!("generated chunk: {:?}", c);
    /*
    let mut c = create_instances([0, 0, 0]);
    c.extend(create_instances([1, 0, 0]));
    c.extend(create_instances([-1, 0, 0]));
    */
    res
}

use cgmath::Rotation;

fn top([y, x, z]: [usize; 3]) -> instance::Instance {
    instance::Instance {
        position: Vector3::new(y as f32, x as f32, z as f32),
        rotation: quat_identity(),
    }
}

fn bottom([y, x, z]: [usize; 3]) -> instance::Instance {
    instance::Instance {
        position: Vector3::new(x as f32, y as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., 0., 1.), Vector3::new(0., -1., 0.)),
    }
}

fn right([y, x, z]: [usize; 3]) -> instance::Instance {
    instance::Instance {
        position: Vector3::new(y as f32, x as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., 0., 1.), Vector3::new(-1., 0., 0.)),
    }
}

fn left([y, x, z]: [usize; 3]) -> instance::Instance {
    instance::Instance {
        position: Vector3::new(y as f32, x as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., 0., 1.), Vector3::new(1., 0., 0.)),
    }
}

fn front([y, x, z]: [usize; 3]) -> instance::Instance {
    instance::Instance {
        position: Vector3::new(y as f32, x as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., 1., 0.), Vector3::new(0., 0., -1.)),
    }
}

fn back([y, x, z]: [usize; 3]) -> instance::Instance {
    instance::Instance {
        position: Vector3::new(y as f32, x as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., -1., 0.), Vector3::new(0., 0., 1.)),
    }
}

fn create_instances([y, x, z]: [i32; 3]) -> Vec<instance::Instance> {
    let mut res: Vec<instance::Instance> = Vec::with_capacity(1);

    // Top
    res.push(instance::Instance {
        position: Vector3::new(x as f32, y as f32, z as f32),
        rotation: quat_identity(),
    });

    // Botton
    res.push(instance::Instance {
        position: Vector3::new(x as f32, y as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., 0., 1.), Vector3::new(0., -1., 0.)),
    });

    // Front?
    res.push(instance::Instance {
        position: Vector3::new(x as f32, y as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., 1., 0.), Vector3::new(0., 0., -1.)),
    });

    // Back
    res.push(instance::Instance {
        position: Vector3::new(x as f32, y as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., -1., 0.), Vector3::new(0., 0., 1.)),
    });

    // Right
    res.push(instance::Instance {
        position: Vector3::new(x as f32, y as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., 0., 1.), Vector3::new(-1., 0., 0.)),
    });

    // Left
    res.push(instance::Instance {
        position: Vector3::new(x as f32, y as f32, z as f32),
        rotation: Quaternion::look_at(Vector3::new(0., 0., 1.), Vector3::new(1., 0., 0.)),
    });

    res
}

fn quat_identity() -> Quaternion<f32> {
    Quaternion::from_sv(1., Vector3::new(0., 0., 0.))
}
