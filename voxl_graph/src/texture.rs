use image::{load_from_memory, DynamicImage, GenericImageView};
use wgpu::*;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &Device,
        queue: &Queue,
        bytes: &[u8],
        label: Option<&str>,
    ) -> anyhow::Result<Self> {
        let image = &load_from_memory(bytes)?;
        Self::from_image(device, queue, image, label)
    }

    pub fn from_image(
        device: &Device,
        queue: &Queue,
        image: &DynamicImage,
        label: Option<&str>,
    ) -> anyhow::Result<Self> {
        let rgba = image
            .as_rgba8()
            .expect("image might not have an alpha component. Did you use a png?");

        let dimensions = image.dimensions();

        let size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });

        queue.write_texture(
            TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            rgba,
            TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
