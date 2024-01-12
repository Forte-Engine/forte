/// The rust representation of all needed light information needed for shaders.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    position: [f32; 3],
    range: f32,
    color: [f32; 3],
    exponent: f32,
    direction: [f32; 3],
    cutoff: f32
}

impl LightUniform {
    /// The bind group layout to be used for LightUniform.  This is here to promote consistency across implementations.
    pub const BIND_LAYOUT: wgpu::BindGroupLayoutDescriptor<'_> = wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform, 
                    has_dynamic_offset: false, 
                    min_binding_size: None
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform, 
                    has_dynamic_offset: false, 
                    min_binding_size: None
                },
                count: None
            }
        ],
        label: Some("light_uniform_layout"),
    };

    /// Create a new light uniform.
    /// 
    /// Arguments:
    /// * position: [f32; 3] - The position of the light.
    /// * color: [f32; 3] - The color of the light.
    /// * direction: [f32; 3] - The direction of the light.  Only needed if cutoff is set, otherwise, leave as anything.
    /// * range: f32 - The range of the light in units.  If not needed, set to infinity.
    /// * exponent: f32 - The exponent of the light used to adjust how the light "bleeds off" over distance.  If not needed, leave as 0.
    /// * cutoff: f32 - The dot product at which a light will "cutoff".  This is usual for spot lights.  If no cut off needed, set this to higher than 100.
    /// 
    /// Returns a new light uniform.
    pub fn new(position: [f32; 3], color: [f32; 3], direction: [f32; 3], range: f32, exponent: f32, cutoff: f32) -> Self {
        Self { position, color, range, exponent, direction, cutoff }
    }
}
