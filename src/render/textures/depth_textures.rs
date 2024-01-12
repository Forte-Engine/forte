/// A representation of a WGPU texture with a texture view and sampler for a texture that is meant to be used as a depth texture.
#[derive(Debug)]
pub struct DepthTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler
}

impl DepthTexture {
    /// The texture format to be used for depth textures.
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    /// Create a new depth texture for WGPU.
    /// 
    /// Arguments:
    /// * config: &wgpu::SurfaceConfiguration - A WGPU surface configuration to be used to create the depth texture.
    /// * device: &wgpu::Device - A WGPU device to be used to create the depth texture.
    /// * label: &str - The label for the depth texture.
    /// 
    /// Returns the created depth texture.
    pub fn new(
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device, 
        label: &str
    ) -> Self {
        // create size and description for depth texture
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[]
        };

        // create texture view and sampler
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        // create new instance of self
        Self { texture, view, sampler }
    }
}
