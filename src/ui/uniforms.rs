use cgmath::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RectUniform {
    matrix: Matrix4<f32>
}