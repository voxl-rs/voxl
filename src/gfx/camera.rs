use crate::gfx::Resolution;
use cgmath::{ortho, perspective, Matrix4, Rad};
use legion::{storage::Component, *};

// use wgpu::BufferUsage;

/// A kind of 3D Projection
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Project3D {
    Orthographic,
    Perspective,
}

/// A screen projection that produces a matrix.
#[derive(Debug, Clone, Copy)]
pub struct Projection {
    projection: Project3D,
    fov_y: Rad<f32>,
    z_near: f32,
    z_far: f32,
}

impl Projection {
    /// Projection matrix based on `Resolution`.
    pub fn matrix(&self, res: &Resolution) -> Matrix4<f32> {
        use Project3D::*;
        match self.projection {
            Perspective => {
                let aspect = res.aspect();
                let &Self {
                    fov_y,
                    z_near,
                    z_far,
                    ..
                } = self;

                perspective(fov_y, *aspect, z_near, z_far)
            }

            Orthographic => {
                let (x, y) = *res.dimensions();
                let width = x as f32;
                let height = y as f32;

                ortho(
                    -width / 2.,
                    width / 2.,
                    -height / 2.,
                    height / 2.,
                    self.z_near,
                    self.z_far,
                )
            }
        }
    }
}

/// A trait that generates a view matrix.
pub trait View {
    fn matrix(&self) -> Matrix4<f32>;
}

use shrinkwraprs::*;
/// A Matrix representing a Projection * View matrix.
#[repr(C)]
#[derive(Debug, Clone, Copy, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Camera(pub Matrix4<f32>);

use bytemuck::*;
unsafe impl Pod for Camera {}
unsafe impl Zeroable for Camera {}

/// Generic camera update system.
pub fn camera_system<V: View + Component>() -> impl systems::Runnable {
    SystemBuilder::new("CameraSystem")
        .with_query(
            <(&mut Camera, &Resolution, &Projection, &V)>::query()
                .filter(maybe_changed::<V>())
                .filter(maybe_changed::<Resolution>())
                .filter(maybe_changed::<Projection>()),
        )
        .build(|_, world, _, query| {
            for (cam, res, proj, view) in query.iter_mut(world) {
                **cam = proj.matrix(res) * view.matrix();
            }
        })
}
