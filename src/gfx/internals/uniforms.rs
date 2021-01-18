use bytemuck::{Pod, Zeroable};
use cgmath::Matrix4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViewProjection {
    view_proj: Matrix4<f32>,
}

impl ViewProjection {
    pub fn update(&mut self, cam: &Matrix4<f32>) {
        self.view_proj = *cam;
    }
}

unsafe impl Pod for ViewProjection {}
unsafe impl Zeroable for ViewProjection {}
