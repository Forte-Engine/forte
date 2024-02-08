use cgmath::*;
use winit::{event::*, keyboard::PhysicalKey};

/// An enum that represents all inputs that the engine currently supports in an easy to reference manner.
/// 
/// Options:
/// * MouseMove(position: Point2<f32>) - Represents new mouse position.
/// * MouseButton(button: MouseButton, state: ElementState) - Represents a state change (pressed or released) of a mouse button.
/// * MouseWheel(delta: MouseScrollDelta) - Represents how much the mouse wheel was turned.
/// * KeyInput(key: VirtualKeyCode, state: ElementState) - Represents a state change (pressed or released) of a keyboard input.
#[derive(Clone, Copy, Debug)]
pub enum EngineInput {
    MouseMove(Point2<f32>),
    MouseButton(winit::event::MouseButton, winit::event::ElementState),
    MouseWheel(winit::event::MouseScrollDelta),
    KeyInput(winit::keyboard::KeyCode, winit::event::ElementState)
}

impl EngineInput {
    /// A function that quickly converts a winit WindowEvent into a `EngineInput`.
    /// 
    /// Arguments:
    /// * event: &WindowEvent - The winit `WindowEvent` that occured.
    pub fn from_winit_input(event: &WindowEvent) -> Option<Self> {
        match event {
            // habndle mouse move inputs
            WindowEvent::CursorMoved { position, .. } => Some(
                    Self::MouseMove(
                        Point2 { 
                            x: position.x as f32, 
                            y: position.y as f32 
                        }
                    )
                ),

            // handle mouse inputs
            WindowEvent::MouseInput { state, button, .. } => Some(Self::MouseButton(*button, *state)),
            
            // handle mouse wheel inputs
            WindowEvent::MouseWheel { delta, .. } => Some(Self::MouseWheel(*delta)),

            // handle keyboard inputs
            WindowEvent::KeyboardInput { event, .. } => {
                match &event.physical_key {
                    PhysicalKey::Code(code) => Some(Self::KeyInput(*code, event.state)),
                    PhysicalKey::Unidentified(_) => None,
                }
            }

            // all other inputs, return nothing
            _ => None
        }
    }
}
