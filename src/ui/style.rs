use cgmath::{Vector2, Vector4};

/// Represents the style used to render a `UIElement`.
/// 
/// Arguments:
/// * position_setting: PositionSetting - Controls the relative positioning of the element.  See `PositionSetting` docs for more info.
/// * color: Color - The color used to fill the `UIElement`.
/// * border_color: Color - The color used on the border if a border is present.
/// * left: Sizing - The distance from the left of the whatever is defined by the above position_setting.
/// * right: Sizing - The distance from the right of the whatever is defined by the above position_setting.
/// * top: Sizing - The distance from the top of the whatever is defined by the above position_setting.
/// * bottom: Sizing - The distance from the bottom of the whatever is defined by the above position_setting.
/// * border: Sizing - Defines the size of the border.  Leave as Auto if no border.
/// * round: Sizing - Defines how much the corners should be rounded.  Leave as Auto if no corner round.
/// * width: Sizing - Defines the width of this `UIElement`.
/// * height: Sizing - Defines the height of this `UIElement`.
/// * rotation: f32 - Defines how much the `UIElement` is rotated in degrees.
#[derive(Default, Debug, Clone)]
pub struct Style {
    pub position_setting: PositionSetting,
    pub color: Color,
    pub border_color: Color,
    pub left: Sizing,
    pub right: Sizing,
    pub top: Sizing,
    pub bottom: Sizing,
    pub border: Sizing,
    pub round: Sizing,
    pub width: Sizing,
    pub height: Sizing,
    pub rotation: f32
}

impl Style {
    /// Gets the minimum size of this object using the given display size.  The display size should be what is the current window size unless you *really* know what you're doing.
    pub fn min_size(&self, display_size: &Vector2<f32>) -> Vector2<f32> {
        Vector2 { 
            x: self.width.size(display_size), 
            y: self.height.size(display_size) 
        }
    }

    /// Returns true if the left argument is not auto.
    pub fn left_set(&self) -> bool { self.left.is_set() }

    /// Returns true if the right argument is not auto.
    pub fn right_set(&self) -> bool { self.right.is_set() }

    /// Returns true if the top argument is not auto.
    pub fn top_set(&self) -> bool { self.top.is_set() }

    /// Returns true if the bottom argument is not auto.
    pub fn bottom_set(&self) -> bool { self.bottom.is_set() }
}

/// Defines a color for a `Style` object.
/// 
/// Arguments:
/// * red: f32 - The red component of this color in 0 -> 1.
/// * green: f32 - The green component of this color in 0 -> 1.
/// * blue: f32 - The blue component of this color in 0 -> 1.
/// * alpha: f32 - The alpha (transparency) component of this color 0 -> 1.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color { pub red: f32, pub green: f32, pub blue: f32, pub alpha: f32 }

impl Color {
    /// Converts the rgba components of a color into a `cgmath::Vector4`.
    pub fn to_vec4(&self) -> Vector4<f32> { Vector4 { x: self.red, y: self.green, z: self.blue, w: self.alpha } }

    // Converts the rgba components of a color into a 4 float array.
    pub fn to_array(&self) -> [f32; 4] { [self.red, self.green, self.blue, self.alpha] }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0
        }
    }
}

/// Defines the positioning of a `UIElement`.
/// 
/// Options:
/// * Parent - Positions relative to the `UIElement`s parent.
/// * Absolute - Positions relative to the window.
#[derive(Default, Debug, Clone, Copy)]
pub enum PositionSetting { 
    #[default]
    Parent, 
    Absolute 
}

/// Defines a size in a `Style`.
/// 
/// Options:
/// * Auto - Uses whatever the default for that element in `Style` is.
/// * Px(pixels: f32) - Returns the given pixels as this size.
/// * PercentWidth(percent: f32) - Returns the percent times the display width.
/// * PercentHeight(percent: f32) - Returns the percent times the display height.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Sizing {
    #[default]
    Auto,
    Px(f32),
    PercentWidth(f32),
    PercentHeight(f32)
}

impl Sizing {
    /// Gets the pixel size of this sizing using the given dimensions.
    /// 
    /// Arguments:
    /// * &self - The sizing to convert.
    /// * dimensions: &Vector2<f32> - The window dimensions (or any dimensions you want IF you know what you're doing).
    pub fn size(&self, dimensions: &Vector2<f32>) -> f32 {
        match self {
            Sizing::Auto => 0.0,
            Sizing::Px(px) => *px,
            Sizing::PercentWidth(percent) => dimensions.x * percent,
            Sizing::PercentHeight(percent) => dimensions.y * percent,
        }
    }

    /// Returns true if this is not Auto.
    pub fn is_set(&self) -> bool { !matches!(self, Self::Auto) }
}
