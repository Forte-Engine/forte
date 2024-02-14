use wgpu::util::DeviceExt;

use crate::{primitives::textures::Texture, render::render_engine::RenderEngine, ui::style::Color};


#[derive(Debug, Default)]
pub struct MaterialBuilder {
    pub albedo_texture: Option<Texture>,
    pub roughness_texture: Option<Texture>,
    pub emissive_texture: Option<Texture>,
    pub normal_texture: Option<Texture>,
    pub occlusion_texture: Option<Texture>,

    pub albedo_color: Color,
    pub emissive_color: Color,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub alpha_mode: f32,
    pub alpha_cutoff: f32,
}

impl MaterialBuilder {
    pub fn build(self, engine: &RenderEngine) -> Material {
        // create albedo buffer
        let albedo_buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("material_albedo_buffer"),
                contents: bytemuck::cast_slice(&[self.albedo_color.red, self.albedo_color.green, self.albedo_color.blue, self.albedo_color.alpha]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        println!("Albedo {:?}", self.albedo_color);

        // create emissive buffer
        let emissive_buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("material_emissive_buffer"),
                contents: bytemuck::cast_slice(&[self.emissive_color.red, self.emissive_color.green, self.emissive_color.blue, self.emissive_color.alpha]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        // create metadata buffer
        let metadata_buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("material_metadata_buffer"),
                contents: bytemuck::cast_slice(&[self.metallic_factor, self.roughness_factor, self.alpha_mode, self.alpha_cutoff]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        // unpack textures
        let empty_texture = create_empty_texture(engine);
        let diffuse_texture = if self.albedo_texture.is_some() { self.albedo_texture.as_ref().unwrap() } else { &empty_texture };
        let roughness_texture = if self.roughness_texture.is_some() { self.roughness_texture.as_ref().unwrap() } else { &empty_texture };
        let emissive_texture = if self.albedo_texture.is_some() { self.emissive_texture.as_ref().unwrap() } else { &empty_texture };
        let normal_texture = if self.albedo_texture.is_some() { self.normal_texture.as_ref().unwrap() } else { &empty_texture };
        let occlusion_texture = if self.albedo_texture.is_some() { self.occlusion_texture.as_ref().unwrap() } else { &empty_texture };

        // create material with bind group
        Material {
            bind_group: engine.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("material_bind_group"),
                layout: &engine.device.create_bind_group_layout(&Material::BIND_LAYOUT),
                entries: &[
                    // diffuse texture
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view)
                    },

                    // diffuse sampler
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler)
                    },
                    
                    // roughness texture
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&roughness_texture.view)
                    },

                    // roughness sampler
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&roughness_texture.sampler)
                    },
                    
                    // emissive texture
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(&emissive_texture.view)
                    },

                    // emissive sampler
                    wgpu::BindGroupEntry {
                        binding: 5,
                        resource: wgpu::BindingResource::Sampler(&emissive_texture.sampler)
                    },
                    
                    // normal texture
                    wgpu::BindGroupEntry {
                        binding: 6,
                        resource: wgpu::BindingResource::TextureView(&normal_texture.view)
                    },

                    // normal sampler
                    wgpu::BindGroupEntry {
                        binding: 7,
                        resource: wgpu::BindingResource::Sampler(&normal_texture.sampler)
                    },
                    
                    // occlusion texture
                    wgpu::BindGroupEntry {
                        binding: 8,
                        resource: wgpu::BindingResource::TextureView(&occlusion_texture.view)
                    },

                    // occlusion sampler
                    wgpu::BindGroupEntry {
                        binding: 9,
                        resource: wgpu::BindingResource::Sampler(&occlusion_texture.sampler)
                    },
                    
                    // diffuse color buffer
                    wgpu::BindGroupEntry {
                        binding: 10,
                        resource: albedo_buffer.as_entire_binding()
                    },

                    // emissive color buffer
                    wgpu::BindGroupEntry {
                        binding: 11,
                        resource: emissive_buffer.as_entire_binding()
                    },

                    // metadata buffer
                    wgpu::BindGroupEntry {
                        binding: 12,
                        resource: metadata_buffer.as_entire_binding()
                    }
                ]
            })
        }
    }
}

#[derive(Debug)]
pub struct Material {
    bind_group: wgpu::BindGroup
}

impl Material {
    pub const BIND_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
        entries: &[
            // diffuse_texture
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

            // diffuse_sampler
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },

            // roughness_texture
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                count: None,
                ty: wgpu::BindingType::Texture { 
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                    view_dimension: wgpu::TextureViewDimension::D2, 
                    multisampled: false 
                }
            },

            // roughness_sampler
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            
            // emissive_texture
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::FRAGMENT,
                count: None,
                ty: wgpu::BindingType::Texture { 
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                    view_dimension: wgpu::TextureViewDimension::D2, 
                    multisampled: false 
                }
            },

            // emissive_sampler
            wgpu::BindGroupLayoutEntry {
                binding: 5,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },

            // normal_texture
            wgpu::BindGroupLayoutEntry {
                binding: 6,
                visibility: wgpu::ShaderStages::FRAGMENT,
                count: None,
                ty: wgpu::BindingType::Texture { 
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                    view_dimension: wgpu::TextureViewDimension::D2, 
                    multisampled: false 
                }
            },

            // normal_sampler
            wgpu::BindGroupLayoutEntry {
                binding: 7,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },

            // occlusion_texture
            wgpu::BindGroupLayoutEntry {
                binding: 8,
                visibility: wgpu::ShaderStages::FRAGMENT,
                count: None,
                ty: wgpu::BindingType::Texture { 
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                    view_dimension: wgpu::TextureViewDimension::D2, 
                    multisampled: false 
                }
            },

            // occlusion_sampler
            wgpu::BindGroupLayoutEntry {
                binding: 9,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },

            // diffuse color
            wgpu::BindGroupLayoutEntry {
                binding: 10,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform, 
                    has_dynamic_offset: false, 
                    min_binding_size: None 
                },
                count: None
            },

            // emissive color
            wgpu::BindGroupLayoutEntry {
                binding: 11,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform, 
                    has_dynamic_offset: false, 
                    min_binding_size: None 
                },
                count: None
            },

            // metadata
            wgpu::BindGroupLayoutEntry {
                binding: 12,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform, 
                    has_dynamic_offset: false, 
                    min_binding_size: None
                },
                count: None
            }
        ],
        label: Some("texture_bind_group_layout")
    };

    pub fn bind<'rpass>(&'rpass self, pass: &mut wgpu::RenderPass<'rpass>, idx: u32) { 
        pass.set_bind_group(idx, &self.bind_group, &[]) 
    }
}

fn create_empty_texture(engine: &RenderEngine) -> Texture {
    Texture::from_bytes(
        &engine.device, 
        &engine.queue, 
        include_bytes!("empty.png"), 
        "forte.material.blank_texture", 
    ).expect("Empty texture for Material did not create properly.")
}