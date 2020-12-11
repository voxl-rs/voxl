/// System Bundles
pub mod bundles;
/// Camera component
pub mod camera;
/// TODO: Fix this
pub mod gfx;
/// GPU instancing utils
pub mod instance;
/// Chunk to model data
pub mod model;
/// GUI and Rendering
pub mod systems;
/// Render Pipeline Texture data
pub mod texture;
/// Render Pipeline Uniform data
pub mod uniforms;
/// Render Pipeline Vertex data
pub mod vertex;

pub use gfx::{swap_chain, Render, RenderBunch};

pub use image as img;
pub use wgpu as gpu;
pub use winit as win;

use shrinkwraprs::Shrinkwrap;

#[derive(Debug, Clone, Copy, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct DisplayFPS(pub f64);

impl Default for DisplayFPS {
    fn default() -> Self {
        Self(0.)
    }
}

#[derive(Debug, Clone, Copy, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct DrawFrame(pub bool);
