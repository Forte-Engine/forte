use cgmath::*;
use wgpu::util::DeviceExt;

use crate::render::{OPENGL_TO_WGPU_MATRIX, render_engine::*};

/// A representation of all information needed to properly use a camera in WGPU.
#[derive(Debug)]
pub struct Camera {
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
    pub bind_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,

    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32
}

impl Camera {
    /// The bind group layout to be used to use this camera in any shader.  This is hear to promote consistency across implementations.
    pub const BIND_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
        label: Some("camera_bind_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                }
            }
        ]
    };

    /// Creates a new camera.
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine that this camera will belong too.
    /// * aspect: f32 - The aspect ratio of this camera
    /// * fovy: f32 - The field of view on the vertical axis of the camera.
    /// * znear: f32 - The distance to the nearest clipping plane where the rendering will stop.
    /// * zfar: f32 - The distance to the farthest clipping plane where the rendering will stop.
    /// 
    /// Note: only objects between the two mentioned clipping planes will be rendered.
    /// 
    /// Returns a new camera.
    pub fn new(
        engine: &RenderEngine, 
        aspect: f32, fovy: f32, 
        znear: f32, zfar: f32
    ) -> Self {
        let uniform = CameraUniform::new();

        // create camera uniform and buffer
        let buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        // create camera bind layout and group
        let bind_layout = engine.device.create_bind_group_layout(&Self::BIND_LAYOUT);
        let bind_group = engine.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ]
        });
        Self {
            uniform, buffer, bind_layout, bind_group,
            position: (0.0, 0.0, 0.0).into(), 
            rotation: (0.0, 0.0, 0.0, 1.0).into(),
            aspect, fovy, znear, zfar
        }
    }

    /// Builds a view projection matrix for this camera.
    /// 
    /// Returns the view and projection matrix for this camera.
    pub fn build_view_projection_matrix(&self, engine: &RenderEngine) -> cgmath::Matrix4<f32> {
        let view = Matrix4::from(self.rotation) * Matrix4::from_translation(-self.position);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), engine.config.width as f32 / engine.config.height as f32, self.znear, self.zfar);
        // println!("Build")
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    /// Updates the camera and its WGPU buffers with the given render engine.
    /// 
    /// Arguments:
    /// * engine: &mut RenderEngine - The render engine to which the buffers will be updated.
    pub fn update(&mut self, engine: &RenderEngine) {
        self.uniform.view_position = [self.position.x, self.position.y, self.position.z, 0.0];
        self.uniform.view_proj = self.build_view_projection_matrix(engine).into();
        engine.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }

    /// Binds this camera to the given render pass at the given bind group index.
    /// 
    /// Arguments:
    /// * &self - The camera to be bound.
    /// * pass: &mut wgpu::RenderPass - The render pass to bind too.
    /// * index: u32 - The index in the bind group to bind too.
    pub fn bind<'rpass>(
        &'rpass mut self,
        pass: &mut wgpu::RenderPass<'rpass>,
        engine: &RenderEngine,
        index: u32
    ) {
        self.update(engine);
        pass.set_bind_group(index, &self.bind_group, &[]);
    }
}

/// The camera uniform, this is the rust representation of the camera data is passed to the shaders.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform { 
    pub view_position: [f32; 4],
    pub view_proj: [[f32; 4]; 4] 
}

impl CameraUniform {
    /// Creates a new empty shader uniform with empty matrices.
    pub fn new() -> Self {
        Self { view_position: [0.0; 4], view_proj: cgmath::Matrix4::identity().into() }
    }
}
