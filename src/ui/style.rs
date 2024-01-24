use cgmath::Vector2;

#[derive(Default, Debug, Clone)]
pub struct Style {
    pub position: Position,
    pub color: Color,
    pub left: Sizing,
    pub right: Sizing,
    pub top: Sizing,
    pub bottom: Sizing,
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
pub struct Color { red: f32, green: f32, blue: f32 }

#[derive(Default, Debug, Clone, Copy)]
pub enum Position { 
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
}