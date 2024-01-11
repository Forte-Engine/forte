use wgpu::util::DeviceExt;

use crate::primitives::vertices::Vertex;

#[derive(Debug)]
pub struct Mesh {
    pub(crate) vertex_buf: wgpu::Buffer,
    pub(crate) index_buf: wgpu::Buffer,
    pub(crate) num_indices: u32,
    pub(crate) num_vertices: u32
}

impl Mesh {
    pub fn from_raw(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Self {
        Self {
            vertex_buf: device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX
                }
            ),
            index_buf: device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX
                }
            ),
            num_indices: indices.len() as u32,
            num_vertices: vertices.len() as u32
        }
    }
}
