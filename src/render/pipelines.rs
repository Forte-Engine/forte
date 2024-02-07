use wgpu::BindGroupLayout;

use crate::{primitives::textures::depth_textures::DepthTexture, render::render_engine::RenderEngine};

/// The forte representation of a render pipeline.  This is effectively a shader with some necessary WGPU layouts.
#[derive(Debug)]
pub struct Pipeline {
    pub render_pipeline: wgpu::RenderPipeline
}

impl Pipeline {
    /// Create a new shader with a given name, render engine, shader code, buffer layouts, and bind group layouts.
    /// 
    /// Arguments:
    /// * name: &str - The name of the pipeline for debugging purposes.
    /// * engine: &RenderEngine - The render engine that will be used to create the pipeline.
    /// * shader_code: &str - The WGSL shader code for this pipeline.
    /// * buffers: &[wgpu::VertexBufferLayout] - An array of vertex buffer layouts for the shader.
    /// * layouts: &[wgpu::BindGroupLayout] - An array of bind group layouts for the shader.
    /// 
    /// Returns the new pipeline that is generated from the above arguments.
    /// 
    /// Example:
    /// ```rust
    /// Pipeline::new(
    ///     "shader", 
    ///     engine, 
    ///     include_str!("path_to_file"),
    ///     &[
    ///         Vertex::desc(), 
    ///         TransformRaw::desc()
    ///     ],
    ///     &[
    ///         &engine.device.create_bind_group_layout(&Camera::BIND_LAYOUT),
    ///         &engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT),
    ///         &engine.device.create_bind_group_layout(&LightUniform::BIND_LAYOUT)
    ///     ]
    /// );
    /// ```
    pub fn new(
        name: &str, 
        engine: &RenderEngine, 
        shader_code: &str, 
        buffers: &[wgpu::VertexBufferLayout], 
        layouts: &[&wgpu::BindGroupLayout],
        use_depth: bool
    ) -> Self {
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
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
                depth_stencil: if !use_depth { None } else {
                    Some(wgpu::DepthStencilState {
                        format: DepthTexture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default()
                    })
                },
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false
                },
                multiview: None
            })
        }
    }

    /// Get a bind group at a given index
    /// 
    /// Arguments:
    /// * index: u32 - the index of the bind group
    /// 
    /// Returns the bind group layout
    pub fn get_layout(&self, index: u32) -> BindGroupLayout {
        self.render_pipeline.get_bind_group_layout(index)
    }

    /// Binds this pipeline to the given render pass.
    /// 
    /// Arguments:
    /// * &self - The pipeline to bind.
    /// * pass: &mut RenderPass - The render pass to bind this pipeline too.
    pub fn bind<'rpass>(
        &'rpass self,
        pass: &mut wgpu::RenderPass<'rpass>
    ) {
        pass.set_pipeline(&self.render_pipeline);
    }
}