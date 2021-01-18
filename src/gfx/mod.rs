mod internals;
pub use internals::texture::Texture;

mod camera;
pub use camera::*;

pub use wgpu::BackendBit;
pub use winit::{dpi::PhysicalSize, window::Window};

mod canvas;
pub use canvas::*;

mod paint_brush;
pub use paint_brush::*;

use shrinkwraprs::*;
#[derive(Debug, Clone, Copy)]
/// Represents 2D screen dimensions and its aspect ratio.
pub struct Resolution {
    xy: (u32, u32),
    ratio: f32,
}

impl Resolution {
    pub fn update<T: Into<Resolution>>(&mut self, r: T) {
        *self = r.into();
    }

    pub fn dimensions(&self) -> &'_ (u32, u32) {
        &self.xy
    }

    pub fn aspect(&self) -> &'_ f32 {
        &self.ratio
    }
}

impl From<PhysicalSize<u32>> for Resolution {
    fn from(size: PhysicalSize<u32>) -> Self {
        Self {
            xy: (size.width, size.height),
            ratio: size.height as f32 / size.width as f32,
        }
    }
}

impl From<[u32; 2]> for Resolution {
    fn from(xy: [u32; 2]) -> Self {
        Self {
            xy: (xy[0], xy[1]),
            ratio: xy[0] as f32 / xy[1] as f32,
        }
    }
}

impl From<(u32, u32)> for Resolution {
    fn from(xy: (u32, u32)) -> Self {
        Self {
            xy,
            ratio: xy.0 as f32 / xy.1 as f32,
        }
    }
}

#[derive(Debug, Clone, Copy, Shrinkwrap)]
#[shrinkwrap(mutable)]
/// Limit framerate to device specification.
// TODO: don't forget this
pub struct Vsync(pub bool);
