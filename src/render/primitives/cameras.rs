use cgmath::*;
use wgpu::util::DeviceExt;
use winit::event::{ElementState, VirtualKeyCode};

use crate::render::{OPENGL_TO_WGPU_MATRIX, render_engine::*, input::EngineInput};

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
    pub const BIND_LAYOUT: wgpu::BindGroupLayoutDescriptor<'_> = wgpu::BindGroupLayoutDescriptor {
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
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = Matrix4::from(self.rotation) * Matrix4::from_translation(-self.position);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    /// Updates the camera and its WGPU buffers with the given render engine.
    /// 
    /// Arguments:
    /// * engine: &mut RenderEngine - The render engine to which the buffers will be updated.
    pub fn update(&mut self, engine: &mut RenderEngine) {
        self.uniform.view_position = [self.position.x, self.position.y, self.position.z, 0.0];
        self.uniform.view_proj = self.build_view_projection_matrix().into();
        engine.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
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

/// A basic flying camera controller.
#[derive(Debug)]
pub struct CameraController {
    pub speed: f32,
    is_forward_pressed: bool, is_backward_pressed: bool,
    is_left_pressed: bool, is_right_pressed: bool,
    is_up_pressed: bool, is_down_pressed: bool
}

impl CameraController {
    /// Creates a new flying camera controller.
    /// 
    /// Arguments:
    /// * speed: f32 - The speed of the flying camera.
    /// 
    /// Returns a new camera controller.
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false, is_backward_pressed: false,
            is_left_pressed: false, is_right_pressed: false,
            is_up_pressed: false, is_down_pressed: false
        }
    }

    /// Processes the given input into changes for the controller.
    /// 
    /// Arguments:
    /// * input: &RenderEngineInput - The input to be processed.
    pub fn input(&mut self, input: &EngineInput) {
        match input {
            EngineInput::KeyInput(key, state) => {
                let pressed = state == &ElementState::Pressed;
                match key {
                    VirtualKeyCode::W => self.set_forward(pressed),
                    VirtualKeyCode::S => self.set_backward(pressed),
                    VirtualKeyCode::A => self.set_left(pressed),
                    VirtualKeyCode::D => self.set_right(pressed),
                    VirtualKeyCode::LShift => self.set_up(pressed),
                    VirtualKeyCode::LControl => self.set_down(pressed),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Set if the camera is moving forward.
    pub fn set_forward(&mut self, pressed: bool) { self.is_forward_pressed = pressed; }
    /// Set if the camera is moving backward.
    pub fn set_backward(&mut self, pressed: bool) { self.is_backward_pressed = pressed; }
    /// Set if the camera is rotating left.
    pub fn set_left(&mut self, pressed: bool) { self.is_left_pressed = pressed; }
    /// Set if the camera is rotating right.
    pub fn set_right(&mut self, pressed: bool) { self.is_right_pressed = pressed; }
    /// Set if the camera is rotating up.
    pub fn set_up(&mut self, pressed: bool) { self.is_up_pressed = pressed; }
    /// Set if the camera is rotating down.
    pub fn set_down(&mut self, pressed: bool) { self.is_down_pressed = pressed; }

    /// Update the given camera with this camera controller
    /// 
    /// Arguments:
    /// * camera: &mut Camera - The camera to be updated.
    pub fn update_camera(&self, camera: &mut Camera) {
        let mut forward = camera.rotation * Vector3::unit_z();
        forward.x *= -1.0;
        forward.y *= -1.0;

        if self.is_forward_pressed { camera.position += forward * -self.speed; }
        else if self.is_backward_pressed { camera.position += forward * self.speed; }
        if self.is_up_pressed { camera.rotation = Quaternion::from_angle_x(cgmath::Deg(-5.0 * self.speed)) * camera.rotation; }
        else if self.is_down_pressed { camera.rotation = Quaternion::from_angle_x(cgmath::Deg(5.0 * self.speed)) * camera.rotation; }
        if self.is_left_pressed { camera.rotation = Quaternion::from_angle_y(cgmath::Deg(-5.0 * self.speed)) * camera.rotation; }
        else if self.is_right_pressed { camera.rotation = Quaternion::from_angle_y(cgmath::Deg(5.0 * self.speed)) * camera.rotation; }
    }
}
