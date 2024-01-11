use cgmath::*;
use wgpu::util::DeviceExt;
use winit::event::{ElementState, VirtualKeyCode};

use crate::{OPENGL_TO_WGPU_MATRIX, render_engine::{RenderEngine, RenderEngineInput}};

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

    pub fn build_view_projection_matrx(&self) -> cgmath::Matrix4<f32> {
        let view = Matrix4::from(self.rotation) * Matrix4::from_translation(-self.position);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn update(&mut self, engine: &mut RenderEngine) {
        self.uniform.view_position = [self.position.x, self.position.y, self.position.z, 0.0];
        self.uniform.view_proj = self.build_view_projection_matrx().into();
        engine.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform { 
    pub view_position: [f32; 4],
    pub view_proj: [[f32; 4]; 4] 
}

impl CameraUniform {
    pub fn new() -> Self {
        Self { view_position: [0.0; 4], view_proj: cgmath::Matrix4::identity().into() }
    }
}

#[derive(Debug)]
pub struct CameraController {
    pub speed: f32,
    is_forward_pressed: bool, is_backward_pressed: bool,
    is_left_pressed: bool, is_right_pressed: bool,
    is_up_pressed: bool, is_down_pressed: bool
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false, is_backward_pressed: false,
            is_left_pressed: false, is_right_pressed: false,
            is_up_pressed: false, is_down_pressed: false
        }
    }

    pub fn input(&mut self, input: &RenderEngineInput) {
        match input {
            RenderEngineInput::KeyInput(key, state) => {
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

    pub fn set_forward(&mut self, pressed: bool) { self.is_forward_pressed = pressed; }
    pub fn set_backward(&mut self, pressed: bool) { self.is_backward_pressed = pressed; }
    pub fn set_left(&mut self, pressed: bool) { self.is_left_pressed = pressed; }
    pub fn set_right(&mut self, pressed: bool) { self.is_right_pressed = pressed; }
    pub fn set_up(&mut self, pressed: bool) { self.is_up_pressed = pressed; }
    pub fn set_down(&mut self, pressed: bool) { self.is_down_pressed = pressed; }

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
