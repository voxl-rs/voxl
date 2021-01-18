use super::vertex::Vertex;
use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Quaternion, Vector3};
use wgpu::{
    BufferAddress, InputStepMode, VertexAttributeDescriptor, VertexBufferDescriptor, VertexFormat,
};

#[derive(Debug, Clone, Copy)]
pub struct Instance {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl From<Instance> for InstanceRaw {
    fn from(instance: Instance) -> Self {
        Self {
            model: (Matrix4::from_translation(instance.position)
                * Matrix4::from(instance.rotation))
            .into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl Vertex for InstanceRaw {
    fn vb_desc<'a>() -> VertexBufferDescriptor<'a> {
        use std::mem::size_of;
        VertexBufferDescriptor {
            stride: size_of::<Self>() as BufferAddress,
            step_mode: InputStepMode::Instance,
            attributes: &[
                VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 5,
                    format: VertexFormat::Float4,
                },
                VertexAttributeDescriptor {
                    offset: size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 6,
                    format: VertexFormat::Float4,
                },
                VertexAttributeDescriptor {
                    offset: size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 7,
                    format: VertexFormat::Float4,
                },
                VertexAttributeDescriptor {
                    offset: size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 8,
                    format: VertexFormat::Float4,
                },
            ],
        }
    }
}

unsafe impl Pod for InstanceRaw {}
unsafe impl Zeroable for InstanceRaw {}
