use wgpu::util::DeviceExt;

use crate::{render::render_engine::RenderEngine, ui::uniforms::UIInstance};

#[derive(Debug)]
pub struct UICanvas {
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) cur_size: usize
}

impl UICanvas {
    /// Creates new canvas
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The rener engine to create the canvas for.
    /// 
    /// Returns a instance of `UICanvas`
    pub fn new(engine: &RenderEngine) -> Self { Self::new_with_contents(engine, &[]) }

    /// Create new canvas with a starting contents of `UIInstance`'s
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine to create the canvas for.
    /// * contents: &[UIInstance] - An array of `UIInstance`'s to fill the canvas with.
    /// 
    /// Returns a instance of `UICanvas`
    pub fn new_with_contents(engine: &RenderEngine, contents: &[UIInstance]) -> Self {
        Self {
            buffer: engine.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("UICanvas Buffer"),
                    contents: bytemuck::cast_slice(contents),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                }
            ),
            cur_size: contents.len()
        }
    }

    /// Updates the contents of the canvas.
    /// 
    /// Arguments:
    /// * &mut self - An instance of `UICanvas` to update.
    /// * engine: &RenderEngine - The owning instance of `RenderEngine` of this canvas.
    /// * contents: &[UIInstance] - The new contents of this canvas.
    pub fn update(&mut self, engine: &RenderEngine, contents: &[UIInstance]) {
        // if contents is the same size, simply update the buffer
        if self.cur_size == contents.len() {
            engine.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(contents));
        }
        // otherwise, create new buffer
        else {
            self.buffer = engine.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("UICanvas Buffer"),
                    contents: bytemuck::cast_slice(contents),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                }
            );
            self.cur_size = contents.len();
        }
    }
}