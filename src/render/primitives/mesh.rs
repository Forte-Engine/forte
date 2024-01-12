use wgpu::util::DeviceExt;

use crate::render::primitives::vertices::Vertex;

/// A simple struct that contains the information of mesh in a way that can be used by WGPU.
#[derive(Debug)]
pub struct Mesh {
    pub(crate) vertex_buf: wgpu::Buffer,
    pub(crate) index_buf: wgpu::Buffer,
    pub(crate) num_indices: u32,
    pub(crate) num_vertices: u32
}

impl Mesh {
    /// Create a new mesh from a WGPU device with vertices and indices arrays.
    /// 
    /// Arguments:
    /// * device: &wgpu::Device - The WGPU device to be used to create the buffers for this mesh.
    /// * vertices: &[Vertex] - The array of vertices for this mesh.
    /// * indices: &[u16] - The indices of this mesh.
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
