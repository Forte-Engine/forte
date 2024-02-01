use cgmath::*;
use winit::keyboard::KeyCode;

use crate::primitives::cameras::Camera;

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
    pub fn key_input(&mut self, key_code: KeyCode, pressed: bool) {
        match key_code {
            KeyCode::KeyW => self.set_forward(pressed),
            KeyCode::KeyS => self.set_backward(pressed),
            KeyCode::KeyA => self.set_left(pressed),
            KeyCode::KeyD => self.set_right(pressed),
            KeyCode::ShiftLeft => self.set_up(pressed),
            KeyCode::ControlLeft => self.set_down(pressed),
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
