use std::fmt::Debug;

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

#[macro_export]
macro_rules! define_ui_functions {
    ($node:ident, $component:ident) => {
        use cgmath::Vector2;
        fn recr_ui_render(
            win_size: &Vector2<f32>, 
            ui: &Vec<$node>
        ) {

        }
    };
}
