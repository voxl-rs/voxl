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

/*
pub struct Camera {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(sc_desc: &SwapChainDescriptor) -> Self {
        Self {
            aspect: sc_desc.width as f32 / sc_desc.height as f32,
            fovy: 45.,
            znear: 0.1,
            zfar: 10000000.,
        }
    }

    pub fn build_view_projection(&self, position: Point3<f32>, dir: Vector3<f32>) -> Matrix4<f32> {
        let view = Matrix4::look_at(position, position + dir, Vector3::unit_y());
        let proj = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect: 16. / 9.,
            fovy: 45.,
            znear: 0.1,
            zfar: 10000000.,
        }
    }
}

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0., 0., 0.,
    0., 1., 0., 0.,
    0., 0., 0.5, 0.,
    0., 0., 0.5, 1.,
);
*/
