use image::GenericImageView;
use anyhow::*;

pub mod depth_textures;

/// A representation of a WGPU texture, along with its bind group, view, and sampler for shaders.
#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup
}

impl Texture {
    /// The bind group layout for the texture so there is consistency across implementations that use this texture.
    pub const BIND_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                count: None,
                ty: wgpu::BindingType::Texture { 
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                    view_dimension: wgpu::TextureViewDimension::D2, 
                    multisampled: false 
                }
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            }
        ],
        label: Some("texture_bind_group_layout")
    };

    /// Create a new texture from the given bytes.
    /// 
    /// Arguments:
    /// * device: &wgpu::Device - A wgpu device used to create the texture.
    /// * queue: &wgpu::Queue - A wgpu queue that will be used to load the texture to the GPU.
    /// * bytes: &[u8] - The bytes of the image.
    /// * label: &str - A label for the texture.
    /// 
    /// Returns a result that will contain the texture if it was loaded properly, otherwise, an error will be thrown.
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8], 
        label: &str
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        // create texture
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            }
        );

        // write texture to the queue
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        // create view and sampler
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        // create bind group so the texture can be used by shaders
        let bind_layout = &device.create_bind_group_layout(&Self::BIND_LAYOUT);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_layout,
            label: Some("diffuse_bind_group"),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view)
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler)
                }
            ]
        });
        
        Ok(Self { texture, view, sampler, bind_group })
    }

    /// Binds a texture to the the given render pass at the given bind group index.
    /// 
    /// Arguments:
    /// * &self - The texture to bind.
    /// * pass: &mut wgpu::RenderPass - The render pass to bind too.
    /// * index: u32 - The bind group index to bind too.
    pub fn bind<'rpass>(
        &'rpass self,
        pass: &mut wgpu::RenderPass<'rpass>,
        index: u32
    ) {
        pass.set_bind_group(index, &self.bind_group, &[]);
    }
}