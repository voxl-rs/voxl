use bytemuck::{Pod, Zeroable};
use wgpu::{
    BufferAddress, InputStepMode, VertexAttributeDescriptor, VertexBufferDescriptor, VertexFormat,
};

pub trait Vertex {
    fn vb_desc<'a>() -> VertexBufferDescriptor<'a>;
}

/// Only contains texture coordinates
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Tex {
    tex_coords: [f32; 2],
}

unsafe impl Pod for Tex {}
unsafe impl Zeroable for Tex {}

impl Vertex for Tex {
    fn vb_desc<'a>() -> VertexBufferDescriptor<'a> {
        VertexBufferDescriptor {
            stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float2,
            }],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TexVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl TexVertex {
    #[allow(dead_code)]
    pub const fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }
}

impl Vertex for TexVertex {
    fn vb_desc<'a>() -> VertexBufferDescriptor<'a> {
        VertexBufferDescriptor {
            stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[
                VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float3,
                },
                VertexAttributeDescriptor {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float2,
                },
            ],
        }
    }
}
unsafe impl Pod for TexVertex {}
unsafe impl Zeroable for TexVertex {}
