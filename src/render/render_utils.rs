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

/// A macro that starts a render and returns a render resources instance.
/// 
/// Arguments:
/// * engine: RenderEngine - The render engine to render too.
/// 
/// Returns a instance of `RenderResources` instance.
#[macro_export]
macro_rules! start_render {
    ($engine: expr) => {
        {
            let resources = render_utils::prepare_render(&$engine);
            let mut resources = if resources.is_ok() { resources.unwrap() } else { return };
            resources
        }
    }
}

/// Just a quick macro that calls `render_utils::finalize_render`.  This macro was created to match the start_render macro and pass macro.
#[macro_export]
macro_rules! end_render {
    ($engine: expr, $resources: expr) => {
        render_utils::finalize_render(&mut $engine, $resources);
    };
}

/// A macro that creates all the render pass boilerplate with color and depth attachments.
/// 
/// Arguments:
/// * engine: RenderEngine - The render engine to render too.
/// * resources: RenderResources - The render resources to render too.
/// 
/// Returns a `wgpu::RenderPass` instance.
#[macro_export]
macro_rules! pass {
    ($engine: expr, $resources: expr) => {
        pass!($engine, $resources, wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 })
    };

    ($engine: expr, $resources: expr, $color: expr) => {
        {
            let color_attachment = wgpu::RenderPassColorAttachment {
                view: &$resources.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear($color),
                    store: wgpu::StoreOp::Store,
                },
            };
            let mut pass = $resources.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &$engine.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store
                    }),
                    stencil_ops: None
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            pass
        }
    };
}
