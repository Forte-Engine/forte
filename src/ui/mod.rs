use cgmath::{Quaternion, Vector2, Vector3, Zero};
use glyphon::*;
use wgpu::MultisampleState;

use crate::{component_app::EngineComponent, create_pipeline, math::{quaternion::QuaternionExt, transforms::Transform}, primitives::{mesh::Mesh, textures::Texture, transforms::TransformRaw, vertices::Vertex}, render::{pipelines::Pipeline, render_engine::RenderEngine}, utils::resources::Handle};

use self::{elements::{ElementInfo, UIElement}, style::PositionSetting, uniforms::UIInstance};

pub mod elements;
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
pub struct UIEngine {
    mesh: Handle<Mesh>,
    default_texture: Handle<Texture>,
    pub elements: Vec<UIElement>,

    // text
    font_system: FontSystem,
    font_cache: SwashCache,
    text_atlas: TextAtlas,
    text_renderer: TextRenderer
}

// Some info used for rendering
#[derive(Debug)]
pub struct UIRenderInfo {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub display_size: Vector2<f32>
}

/// The UI shader.
#[include_wgsl_oil::include_wgsl_oil("ui.wgsl")]
mod ui_shader {}

impl EngineComponent<&mut RenderEngine<'static>> for UIEngine {
    fn create(engine: &mut RenderEngine) -> Self {
        create_pipeline! {
            NAME => "forte.ui",
            ENGINE => engine,
            SHADER => ui_shader::SOURCE,
            BUFFER_LAYOUTS => [Vertex::desc(), UIInstance::desc()],
            BIND_GROUPS => [Texture::BIND_LAYOUT],
            HAS_DEPTH => false
        }

        let mesh = engine.create_mesh("ui_engine_mesh", VERTICES, INDICES);
        let default_texture = engine.create_texture("ui.blank", include_bytes!("empty.png"));

        // setup text resources
        let font_system = FontSystem::new();
        let font_cache = SwashCache::new();
        let mut text_atlas = TextAtlas::new(&engine.device, &engine.queue, engine.config.format);
        let text_renderer = TextRenderer::new(&mut text_atlas, &engine.device, MultisampleState::default(), None);
        
        Self { 
            mesh, 
            default_texture, elements: Vec::new(), 
            font_system, font_cache, 
            text_atlas, text_renderer
        }
    }

    fn update(&mut self, render_engine: &mut RenderEngine) {
        let size = Vector2 { x: render_engine.size.width as f32, y: render_engine.size.height as f32 };
        let mut text_areas = Vec::<TextArea>::new();
        update_ui(render_engine, &UIRenderInfo { position: Vector2::zero(), size, display_size: size }, &self.elements, &mut text_areas, 0.5);
        let _ = self.text_renderer.prepare(
            &render_engine.device,
            &render_engine.queue,
            &mut self.font_system,
            &mut self.text_atlas,
            Resolution { width: render_engine.config.width, height: render_engine.config.height },
            text_areas,
            &mut self.font_cache
        );
    }

    fn render<'rpass>(&'rpass mut self, render_engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>) {
        render_ui(render_engine, pass, render_engine.mesh(&self.mesh), render_engine.texture(&self.default_texture), &self.elements);
        let _ = self.text_renderer.render(&self.text_atlas, pass);
    }

    fn start(&mut self, _: &mut RenderEngine) {}
    fn exit(&mut self, _: &mut RenderEngine) {}
}

fn render_ui<'rpass>(engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>, mesh: &'rpass Mesh, default_texture: &'rpass Texture, elements: &'rpass [UIElement]) {
    elements.iter().for_each(|element| {
        let texture = match &element.info {
            ElementInfo::Image(texture) => engine.texture(texture),
            _ => default_texture
        };
        pass.set_bind_group(0, &texture.bind_group, &[]);
        pass.set_vertex_buffer(0, mesh.vertex_buf.slice(..));
        pass.set_vertex_buffer(1, element.buffer.slice(..));
        pass.set_index_buffer(mesh.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0 .. mesh.num_indices, 0, 0 .. 1);

        render_ui(engine, pass, mesh, default_texture, &element.children);
    });
}

fn update_ui<'a>(engine: &RenderEngine, info: &UIRenderInfo, elements: &'a [UIElement], text_areas: &mut Vec<TextArea<'a>>, layer: f32) {
    elements.iter().for_each(|element| {
        // calculate size and position of this element
        let (position, size) = calculate_position_size(element, info);
        let new_info = UIRenderInfo { position, size, display_size: info.display_size };

        // generate transform of UI
        let pos_x = size.x * 0.5 + position.x;
        let pos_y = size.y * 0.5 + position.y;
        let transform = Transform {
            position: Vector3 { 
                x: 2.0 * (pos_x / info.display_size.x) - 1.0,
                y: 2.0 * (pos_y / info.display_size.y) - 1.0,
                z: layer
            },
            rotation: Quaternion::euler_deg_z(element.style.rotation),
            scale: Vector3 {
                x: size.x / info.display_size.x,
                y: size.y / info.display_size.y,
                z: 0.0
            }
        };

        // create instance
        let raw_transform = TransformRaw::from_generic(&transform).model;
        let instance = UIInstance([
            raw_transform[0],
            raw_transform[1],
            raw_transform[2],
            raw_transform[3],
            element.style.color.to_array(),
            element.style.border_color.to_array(),
            [
                element.style.round.size(&info.display_size) / f32::max(size.x, size.y),
                element.style.border.size(&info.display_size) / f32::max(size.x, size.y),
                0.0,
                0.0
            ]
        ]);

        // save instance info
        engine.queue.write_buffer(&element.buffer, 0, bytemuck::cast_slice(&instance.0));

        // if text, add text area
        match &element.info {
            ElementInfo::Text(buffer, color) => {
                text_areas.push(TextArea {
                    buffer,
                    left: position.x + 5.0,
                    top: info.display_size.y - position.y - size.y,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: size.x as i32,
                        bottom: size.y as i32,
                    },
                    default_color: *color
                });
            },
            _ => {}
        }

        // update children
        update_ui(engine, &new_info, &element.children, text_areas, layer - 0.05);
    });
}

// calculates the position and size of the given element by taking in its own node and some render info about its parent and display size
fn calculate_position_size(element: &UIElement, info: &UIRenderInfo) -> (Vector2<f32>, Vector2<f32>) {
    // calculate my size
    let size = element.min_size(&info.display_size);

    // calculate initial position
    let mut position =  Vector2 { 
        x: info.position.x + ((info.size.x - size.x) * 0.5), 
        y: info.position.y + ((info.size.y - size.y) * 0.5)
    };

    // if left positioning given, position based on above info, an offset, and the positioning type
    if element.style.left_set() {
        let offset = element.style.left.size(&info.display_size);
        match element.style.position_setting {
            PositionSetting::Parent => {
                position.x = info.position.x + offset;
            },
            PositionSetting::Absolute => {
                position.x = offset;
            }
        }
    } 
    // otherwise, do the same for the right
    else if element.style.right_set() {
        let offset = element.style.right.size(&info.display_size);
        match element.style.position_setting {
            PositionSetting::Parent => {
                position.x = info.position.x + info.size.x - size.x - offset;
            },
            PositionSetting::Absolute => {
                position.x = info.display_size.x - size.x - offset;
            }
        }
    }

    // do top bottom positioning
    if element.style.top_set() {
        let offset = element.style.top.size(&info.display_size);
        match element.style.position_setting {
            PositionSetting::Parent => {
                position.y = info.position.y + info.size.y - size.y - offset;
            },
            PositionSetting::Absolute => {
                position.y = info.display_size.y - size.y - offset;
            }
        }
    } else if element.style.bottom_set() {
        let offset = element.style.bottom.size(&info.display_size);
        position.y = offset;
        match element.style.position_setting {
            PositionSetting::Parent => {
                position.y = info.position.y + offset;
            },
            PositionSetting::Absolute => {
                position.y = offset;
            }
        }
    }

    (position, size)
}
