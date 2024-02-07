use std::fmt::Debug;

use cgmath::Vector2;
use wgpu::util::DeviceExt;

use crate::{primitives::textures::Texture, render::render_engine::RenderEngine, utils::resources::Handle};

use super::{style::Style, UIEngine};

/// The possible states for different UI elements.
#[derive(Debug, Default)]
pub enum ElementInfo {
    #[default]
    Container,
    Image(Handle<Texture>),
    Text(glyphon::Buffer, glyphon::Color)
}

/// A wrapper for a UI element info and style.
#[derive(Debug)]
pub struct UIElement {
    pub style: Style,
    pub info: ElementInfo,
    pub buffer: wgpu::Buffer,
    pub children: Vec<UIElement>
}

/// Creates a default UI buffer
fn ui_buffer(render_engine: &RenderEngine) -> wgpu::Buffer {
    render_engine.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&[[0.0; 4]; 7]),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        }
    )
}

impl UIElement {
    /// Gets the min size of the internal style and the current given display size
    pub fn min_size(&self, display_size: &Vector2<f32>) -> Vector2<f32> { self.style.min_size(display_size) }

    /// Creates a new container with the given render engine and style.
    pub fn container(render_engine: &RenderEngine, style: Style) -> Self { Self { style, info: ElementInfo::Container, buffer: ui_buffer(render_engine), children: Vec::new() } }

    /// Creates a new image element with the given render engine, image and style.
    pub fn image(render_engine: &RenderEngine, style: Style, texture: Handle<Texture>) -> Self { Self { style, info: ElementInfo::Image(texture), buffer: ui_buffer(render_engine), children: Vec::new() } }

    /// Creates a new text element with the given render engine.
    pub fn text(
        render_engine: &RenderEngine, 
        ui_engine: &mut UIEngine,
        style: Style, 
        text: impl Into<String>,
        attrs: glyphon::Attrs,
        color: glyphon::Color,
        metrics: glyphon::Metrics
    ) -> Self { 
        // build text buffer from input
        let mut buffer = glyphon::Buffer::new(&mut ui_engine.font_system, metrics);
        buffer.set_size(&mut ui_engine.font_system, render_engine.size.width as f32, render_engine.size.height as f32);
        buffer.set_text(&mut ui_engine.font_system, &text.into(), attrs, glyphon::Shaping::Advanced);
        buffer.shape_until_scroll(&mut ui_engine.font_system);

        // return new element
        Self { 
            style, 
            info: ElementInfo::Text(buffer, color), 
            buffer: ui_buffer(render_engine), 
            children: Vec::new()
        }
    }
}
