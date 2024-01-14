use std::time::SystemTime;

use wgpu::{TextureView, CommandEncoder, SurfaceTexture};

use super::render_engine::RenderEngine;

/// A container struct for the necessary resources for WGPU to render.
/// 
/// Arguments:
/// * output: SurfaceTexture - The WGPU surface texture to render too.
/// * view: TextureView - The WGPU texture view of the output texture.
/// * encoder: CommandEncoder - The command encoder WGPU uses to actually render too.
pub struct RenderResources {
    pub output: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder
}

/// Call this function to begin a render and get a set of render resources for that frame.
/// 
/// Arguments:
/// * engine: &RenderEngine - The render engine to be rendered too.
/// 
/// Returns a result
/// * ok: RenderResources - The render resources to render too.
/// * error: SurfaceError - A WGPU error that occured when creating the RenderResources.
pub fn prepare_render(
    engine: &RenderEngine
) -> Result<RenderResources, wgpu::SurfaceError> {
    // create output, view, and encoder
    let output = engine.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let encoder = engine.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        }
    );

    // compile and return
    Ok(
        RenderResources {
            output, 
            view, 
            encoder
        }
    )
}

/// Finalizes a render for a RenderEngine and RenderResources.
/// 
/// Arguments:
/// * engine: &mut RenderEngine - The render engine being rendered too.
/// * resources: RenderResources - The render resources that where rendered too.
pub fn finalize_render(
    engine: &mut RenderEngine,
    resources: RenderResources
) {
    // render the queue and wrap up
    engine.queue.submit(std::iter::once(resources.encoder.finish()));
    resources.output.present();

    // update time since start and delta time
    let now = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    let old_time = engine.time_since_start;
    let ms_since_start = if now > engine.start_time { now - engine.start_time } else { 0 };
    engine.time_since_start = ms_since_start as f32 / 1000.0;
    engine.delta_time = engine.time_since_start - old_time;
}