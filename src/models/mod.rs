use crate::primitives::mesh::Mesh;

use self::material::Material;

pub mod gltf;
pub mod material;

#[derive(Debug)]
pub struct Model {
    pub nodes: Vec<Node>
}

impl Model {
    // simply call draws for all nodes in this model
    pub fn draw<'rpass>(&'rpass mut self, pass: &mut wgpu::RenderPass<'rpass>, instances: &'rpass wgpu::Buffer, num_instances: u32) {
        self.nodes.iter().for_each(|node| node.draw(pass, instances, num_instances));
    }
}

#[derive(Debug)]
pub struct Node {
    pub meshes: Option<Vec<(Mesh, Material)>>,
    pub children: Vec<Node>
}

impl Node {
    pub fn draw<'rpass>(&'rpass self, pass: &mut wgpu::RenderPass<'rpass>, instances: &'rpass wgpu::Buffer, num_instances: u32) {
        // if we have meshes in this node, draw all meshes w/ its materials
        if self.meshes.is_some() {
            self.meshes.as_ref().unwrap().iter().for_each(|drawn| {
                drawn.1.bind(pass, 1);
                drawn.0.draw(pass, instances, num_instances);
            });
        }

        // draw children
        self.children.iter().for_each(|node| node.draw(pass, instances, num_instances));
    }
}
