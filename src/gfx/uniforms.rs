use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Uniforms {
    view_proj: Matrix4<f32>,
}

impl Uniforms {
    #[inline(always)]
    pub fn update_view_proj(&mut self, cam: &Matrix4<f32>) {
        self.view_proj = *cam;
    }
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            view_proj: Matrix4::identity(),
        }
    }
}

unsafe impl Pod for Uniforms {}
unsafe impl Zeroable for Uniforms {}
