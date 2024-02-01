use crate::{primitives::{mesh::Mesh, textures::Texture, vertices::Vertex}, render::{pipelines::Pipeline, render_engine::RenderEngine}, utils::resources::Handle};

use self::{elements::UIElement, uniforms::UIInstance};

pub mod canvas;
pub mod elements;
pub mod groups;
pub mod uniforms;
pub mod style;

/// The vertices of a rectangle.
const VERTICES: &[Vertex] = &[
    Vertex { position: [ -1.0, -1.0, 0.0 ], tex_coords: [ 0.0, 1.0 ], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [  1.0, -1.0, 0.0 ], tex_coords: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [ -1.0,  1.0, 0.0 ], tex_coords: [ 0.0, 0.0 ], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [  1.0,  1.0, 0.0 ], tex_coords: [ 1.0, 0.0 ], normal: [0.0, 0.0, 0.0] }
];

/// The indices of a rectangle.
const INDICES: &[u16] = &[
    0, 1, 2,
    1, 3, 2
];

// The engine for rendering UI.
#[derive(Debug)]
pub struct UIEngine {
    pipeline: Pipeline,
    mesh: Handle<Mesh>
}

/// The UI shader.
#[include_wgsl_oil::include_wgsl_oil("ui.wgsl")]
mod ui_shader {}

impl UIEngine {
    pub fn new(engine: &mut RenderEngine) -> Self {
        let pipeline = Pipeline::new(
            "ui", engine, ui_shader::SOURCE,
            &[Vertex::desc(), UIInstance::desc()],
            &[
                &engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT)
            ]
        );

        let mesh = engine.create_mesh("ui_engine_mesh", VERTICES, INDICES);

        Self { pipeline, mesh }
    }
}

pub trait DrawUI<'a, 'b> where 'b: 'a {
    fn prepare_ui(
        &mut self,
        ui_engine: &'b UIEngine
    );

    fn draw_element(
        &mut self,
        render_engine: &'b RenderEngine,
        ui_engine: &'b UIEngine,
        element: &'b UIElement
    );
}

impl<'a, 'b> DrawUI<'a, 'b> for wgpu::RenderPass<'a> where 'b: 'a {
    fn prepare_ui(
        &mut self,
        ui_engine: &'b UIEngine
    ) {
        self.set_pipeline(&ui_engine.pipeline.render_pipeline);
    }

    fn draw_element(
        &mut self,
        render_engine: &'b RenderEngine,
        ui_engine: &'b UIEngine,
        element: &'b UIElement
    ) {
        let mesh = render_engine.mesh(&ui_engine.mesh);
        self.set_vertex_buffer(0, mesh.vertex_buf.slice(..));
        self.set_vertex_buffer(1, element.buffer.slice(..));
        self.set_index_buffer(mesh.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        self.draw_indexed(0 .. mesh.num_indices, 0, 0 .. 1);
    }
}
