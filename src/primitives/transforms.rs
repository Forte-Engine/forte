use cgmath::*;
use wgpu::util::DeviceExt;
use crate::{math::transforms::Transform, render::render_engine::RenderEngine};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformRaw {
    pub model: [[f32; 4]; 4],
    pub normal: [[f32; 3]; 3],
}

impl TransformRaw {
    /// Creates a new vertex buffer layout that describes on a pipline should use raw transforms.  This is here to promote consistency across implementations.
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TransformRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials, we'll
                    // be using 2, 3, and 4 for Vertex. We'll start at slot 5 to not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We don't have to do this in code, though.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }

    /// Create a new raw transform from the given transform.
    pub fn from_generic(transform: &Transform) -> Self {
        TransformRaw {
            model: transform.to_mat().into(),
            normal: Matrix3::from(transform.rotation).into()
        }
    }

    /// Creates an vector of raw transforms from an array of transforms
    pub fn from_generic_array(transforms: &[Transform]) -> Vec<TransformRaw> {
        transforms.iter().map(Self::from_generic).collect::<Vec<TransformRaw>>()
    }

    /// Creates a buffer from an array of `TransformRaw`s.
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine to create the buffer with.
    /// * inputs: &[TransformRaw] - The array to be converted into a buffer.
    /// 
    /// Returns a new buffer created from the inputs.
    pub fn buffer_from_raw(engine: &RenderEngine, inputs: &[Self]) -> wgpu::Buffer {
        engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(inputs),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        )
    }

    /// Creates a buffer from an array of generic `Transform`s.
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine to create the buffer with.
    /// * inputs: &[Transform] - The array to be converted into a buffer.
    /// 
    /// Returns a new buffer created from the inputs.
    pub fn buffer_from_generic(engine: &RenderEngine, inputs: &[Transform]) -> wgpu::Buffer {
        Self::buffer_from_raw(engine, &Self::from_generic_array(inputs))
    }

    /// Updates a given buffer with a given array of `TransformRaw`s.
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine to create the buffer with.
    /// * buffer: &wgpu::Buffer - The buffer to update.
    /// * inputs: &[TransformRaw] - The array to update the buffer with.
    pub fn update_buffer_raw(engine: &RenderEngine, buffer: &wgpu::Buffer, inputs: &[Self]) {
        engine.queue.write_buffer(buffer, 0, bytemuck::cast_slice(inputs));
    }

    /// Updates a given buffer with a given array of `Transform`s.
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine to create the buffer with.
    /// * buffer: &wgpu::Buffer - The buffer to update.
    /// * inputs: &[Transform] - The array to update the buffer with.
    pub fn update_buffer_generic(engine: &RenderEngine, buffer: &wgpu::Buffer, inputs: &[Transform]) {
        Self::update_buffer_raw(engine, buffer, &Self::from_generic_array(inputs));
    }
}
