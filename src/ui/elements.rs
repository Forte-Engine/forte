use std::fmt::Debug;

use cgmath::Vector2;

use super::style::Style;

/// The possible states for different UI elements.
#[derive(Debug, Clone)]
pub enum ElementInfo {
    Container
}

/// A wrapper for a UI element info and style.
#[derive(Debug, Clone)]
pub struct UIElement {
    pub style: Style,
    pub info: ElementInfo
}

impl UIElement {
    pub fn min_size(&self, display_size: &Vector2<f32>) -> Vector2<f32> { self.style.min_size(display_size) }
}
