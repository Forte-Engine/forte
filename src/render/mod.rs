pub mod files;
pub mod input;
pub mod pipelines;
pub mod primitives;
pub mod textures;
pub mod render_engine;
pub mod render_utils;
pub mod resources;

/// A useful matrix for converting opengl matrices to WGPU matrices.  Used in rendering to make our lives easy.
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
