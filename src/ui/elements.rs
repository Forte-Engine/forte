use std::fmt::Debug;

use cgmath::Vector2;

use crate::render::{resources::Handle, textures::textures::Texture};

use super::style::Style;

/// The possible states for different UI elements.
#[derive(Debug)]
pub enum ElementInfo {
    Container,
    Image(Handle<Texture>)
}

/// A wrapper for a UI element info and style.
#[derive(Debug)]
pub struct UIElement {
    pub style: Style,
    pub info: ElementInfo,
    pub buffer: wgpu::Buffer
}

impl UIElement {
    pub fn min_size(&self, display_size: &Vector2<f32>) -> Vector2<f32> { self.style.min_size(display_size) }
}
