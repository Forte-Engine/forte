use wgpu::BindGroupLayout;

use crate::{render_engine::RenderEngine, textures::depth_textures::DepthTexture};

#[derive(Debug)]
pub struct Pipeline {
    pub render_pipeline: wgpu::RenderPipeline
}

impl Pipeline {
    pub fn new(name: &str, engine: &RenderEngine, shader_code: &str, buffers: &[wgpu::VertexBufferLayout], layouts: &[&wgpu::BindGroupLayout]) -> Self {
        // create shader
        let shader = engine.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(shader_code.into())
        });

        // create layout
        let layout = engine.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some(format!("{}_layout", name).as_str()),
                bind_group_layouts: layouts,
                push_constant_ranges: &[]
            }
        );

        Self {
            render_pipeline: engine.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(format!("{}_pipeline", name).as_str()),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: engine.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })]
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DepthTexture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default()
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false
                },
                multiview: None
            })
        }
    }

    pub fn get_layout(&self, index: u32) -> BindGroupLayout {
        self.render_pipeline.get_bind_group_layout(index)
    }
}