use cgmath::{ortho, perspective, InnerSpace, Matrix4, Point3, Rad, Vector3};

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
    pub fn new<Y, P>(yaw: Y, pitch: P) -> Self
    where
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

    pub fn orient<Y, P>(&mut self, yaw: Y, pitch: P)
    where
        Y: Into<Rad<f32>>,
        P: Into<Rad<f32>>,
    {
        self.yaw += yaw.into();
        self.pitch += pitch.into();
        if self.pitch.0 > 1.3 {
            self.pitch.0 = 1.3;
        }
        if self.pitch.0 < -1.3 {
            self.pitch.0 = -1.3;
        }
    }
}

pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    re_aspect: f32,
    re_fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl ProjectionExt for Projection {
    #[inline(always)]
    fn zoom<T: Into<Rad<f32>>>(&mut self, factor: T) {
        self.re_fovy += factor.into();
    }

    #[inline(always)]
    fn aspect(&mut self, factor: f32) {
        self.re_aspect += factor;
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.re_aspect = 0.;
        self.re_fovy = Rad(0.);
    }
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            re_aspect: 0.,
            re_fovy: Rad(0.),
            znear,
            zfar,
        }
    }

    /// Produces the projection matrix
    pub fn perspective(&self) -> Matrix4<f32> {
        perspective(
            self.fovy + self.re_fovy,
            self.aspect + self.re_aspect,
            self.znear,
            self.zfar,
        )
    }

    pub fn ortho(&self, width: f32, height: f32) -> Matrix4<f32> {
        ortho(
            -width / 2.,
            width / 2.,
            -height / 2.,
            height / 2.,
            self.znear,
            self.zfar,
        )
    }

    /// Only use this when updating from changes in screen dimensions;
    /// If you want to modify the aspect ratio for a warping effect, use
    /// `ProjectionExt`'s provided methods instead
    #[inline(always)]
    pub fn re_size(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

/// Extra camera features
pub trait ProjectionExt {
    fn zoom<T: Into<Rad<f32>>>(&mut self, factor: T);
    ///
    fn aspect(&mut self, factor: f32);
    /// Revert modifiers back to 0
    fn reset(&mut self);
}
