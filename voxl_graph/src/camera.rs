use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;
use winit::dpi::LogicalPosition;
use winit::event::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1., 0., 0., 0.,
    0., 1., 0., 0.,
    0., 0., 0.5, 0.,
    0., 0., 0.5, 1.,
);

#[derive(Debug)]
pub struct Camera {
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new<V, Y, P>(_position: V, yaw: Y, pitch: P) -> Self
    where
        V: Into<Point3<f32>>,
        Y: Into<Rad<f32>>,
        P: Into<Rad<f32>>,
    {
        Self {
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn matrix<T: Into<Point3<f32>>>(&self, pos: T) -> Matrix4<f32> {
        Matrix4::look_at_dir(
            pos.into(),
            Vector3::new(self.yaw.0.cos(), self.pitch.0.sin(), self.yaw.0.sin()).normalize(),
            Vector3::unit_y(),
        )
    }
}

pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }
}
