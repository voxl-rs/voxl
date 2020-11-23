//pub mod camera;
pub mod gfx;
pub mod texture;
pub mod vertex;
pub use cgmath;

pub mod wgpu {
    pub use wgpu::*;

    #[derive(Debug)]
    pub struct RenderBunch {
        pub pipeline: RenderPipeline, //
        pub diffuse_bg: BindGroup,    //
        pub uniform_bg: BindGroup,    //
        pub uniform_buff: Buffer,
        pub vertex_buff: Buffer, //
        pub index_buff: Buffer,  //
        pub num_indices: u32,    //
    }
}
pub use winit;
pub mod uniforms;
pub use bytemuck;
