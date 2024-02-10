use crate::primitives::{mesh::Mesh, textures::Texture};

#[derive(Debug)]
pub struct Model {
    pub nodes: Vec<Node>
}

#[derive(Debug)]
pub struct Node {
    pub meshes: Option<Vec<(Mesh, Texture)>>,
    pub children: Vec<Node>
}