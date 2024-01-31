use cgmath::{Vector2, Vector4};

#[derive(Default, Debug, Clone)]
pub struct Style {
    pub position_setting: PositionSetting,
    pub color: Color,
    pub position: SizingRect,
    pub border: SizingRect,
    pub corner_rounds: CornerRounds,
    pub width: Sizing,
    pub height: Sizing,
}

impl Style {
    pub fn min_size(&self, display_size: &Vector2<f32>) -> Vector2<f32> {
        Vector2 { 
            x: self.width.size(display_size), 
            y: self.height.size(display_size) 
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color { pub red: f32, pub green: f32, pub blue: f32, pub alpha: f32 }

impl Color {
    pub fn to_vec4(&self) -> Vector4<f32> { Vector4 { x: self.red, y: self.green, z: self.blue, w: self.alpha } }
    pub fn to_array(&self) -> [f32; 4] { [self.red, self.green, self.blue, self.alpha] }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum PositionSetting { 
    #[default]
    Parent, 
    Absolute 
}

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Sizing {
    #[default]
    Auto,
    Px(f32),
    PercentWidth(f32),
    PercentHeight(f32)
}

impl Sizing {
    pub fn size(&self, dimensions: &Vector2<f32>) -> f32 {
        match self {
            Sizing::Auto => 0.0,
            Sizing::Px(px) => *px,
            Sizing::PercentWidth(percent) => dimensions.x * percent,
            Sizing::PercentHeight(percent) => dimensions.y * percent,
        }
    }

    pub fn is_set(&self) -> bool { !matches!(self, Self::Auto) }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SizingRect {
    pub left: Sizing,
    pub right: Sizing,
    pub top: Sizing,
    pub bottom: Sizing,
}

impl SizingRect {
    pub fn left_set(&self) -> bool { self.left.is_set() }
    pub fn right_set(&self) -> bool { self.right.is_set() }
    pub fn top_set(&self) -> bool { self.top.is_set() }
    pub fn bottom_set(&self) -> bool { self.bottom.is_set() }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CornerRounds {
    pub top_left: Sizing,
    pub top_right: Sizing,
    pub bottom_left: Sizing,
    pub bottom_right: Sizing,
}

impl CornerRounds {
    pub fn top_left_set(&self) -> bool { self.top_left.is_set() }
    pub fn top_right_set(&self) -> bool { self.top_right.is_set() }
    pub fn bottom_left_set(&self) -> bool { self.bottom_left.is_set() }
    pub fn bottom_right_set(&self) -> bool { self.bottom_right.is_set() }
}
