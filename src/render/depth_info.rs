use super::render_engine::RenderEngine;

/// An enum representing the use of the depth buffer during a render pass.
/// 
/// Options:
/// * NONE - Use no depth buffer.
/// * STANDARD - Use the standard depth buffer.
pub enum DepthInfo {
    None,
    Standard
}

impl DepthInfo {
    /// Converts this enum into a optional render pass depth stencil attachment which describes how the depth buffer should be used on a render pass.
    /// 
    /// Arguments:
    /// * &self - A depth info describing how the depth buffer should be used.
    /// * engine: &RenderEngine - The render engine being used to render.
    pub fn to_depth_stencil<'rpass>(&self, engine: &'rpass RenderEngine) -> Option<wgpu::RenderPassDepthStencilAttachment<'rpass>> {
        match self {
            DepthInfo::None => None,
            DepthInfo::Standard => Some(wgpu::RenderPassDepthStencilAttachment {
                view: &engine.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store
                }),
                stencil_ops: None
            })
        }
    }
}