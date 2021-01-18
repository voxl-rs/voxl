use super::CanvasMeta;
use wgpu::*;
use winit::{event_loop::EventLoop, window::WindowBuilder};

#[derive(Debug)]
/// Used for generating wgpu commands.
pub struct PaintBrush {
    device: Device,
    queue: Queue,
}

impl PaintBrush {
    pub fn submit_commands(&self, commands: Vec<CommandBuffer>) {
        self.queue.submit(commands);
    }

    pub fn create_depth_texture(&self, cvm: &CanvasMeta) -> super::Texture {
        super::Texture::create_depth_texture(&self.device, &cvm.sc_desc(), Some("Depth Texture"))
    }

    pub fn new(instance: &Instance) -> Self {
        use futures::executor::block_on;

        let adapter = {
            let win = WindowBuilder::default()
                .with_title("compatibility window")
                .with_visible(false)
                .build(&EventLoop::new())
                .expect("unable to create compat surface");

            let surface = { unsafe { instance.create_surface(&win) } };

            block_on(instance.request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
            }))
            .expect("unable to request adapter")
        };

        let (device, queue) = block_on(adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                shader_validation: true,
            },
            None,
        ))
        .expect("unable to request device");

        Self { device, queue }
    }

    /// ## Warning
    /// Recursive function, I don't know why I did this
    fn _frame(&self, sw: &mut SwapChain, cvm: &CanvasMeta) -> SwapChainFrame {
        if let Ok(frame) = sw.get_current_frame() {
            if !frame.suboptimal {
                return frame;
            }
        }

        *sw = self.device.create_swap_chain(cvm.surface(), cvm.sc_desc());
        log::debug!("recreated swap chain");

        self._frame(sw, cvm)
    }

    pub fn init_buffer<A: Default + bytemuck::Pod>(
        &self,
        label: &'static str,
        usage: BufferUsage,
    ) -> Buffer {
        self.init_buffer_slice(label, &[A::default()], usage)
    }

    pub fn init_buffer_val<A: bytemuck::Pod>(
        &self,
        label: &'static str,
        data: &A,
        usage: BufferUsage,
    ) -> Buffer {
        self.init_buffer_slice(label, &[*data], usage)
    }

    /// Create a buffer from a slice
    pub fn init_buffer_slice<A: bytemuck::Pod>(
        &self,
        label: &'static str,
        data: &[A],
        usage: BufferUsage,
    ) -> Buffer {
        use util::DeviceExt;

        self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage,
        })
    }
}
