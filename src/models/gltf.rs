use gltf::Gltf;

use crate::{create_pipeline, lights::lights::LightUniform, primitives::{cameras::Camera, mesh::Mesh, textures::Texture, transforms::TransformRaw, vertices::Vertex}, render::{pipelines::Pipeline, render_engine::RenderEngine}, ui::style::Color};
use crate::models::{Model, Node};

use super::material::{Material, MaterialBuilder};

#[include_wgsl_oil::include_wgsl_oil("../../shaders/gltf.wgsl")]
mod gltf_shader {}

pub struct GLTFLoader;

impl GLTFLoader {
    pub fn unpack_static_gltf(engine: &mut RenderEngine, gltf: Gltf) -> Model {
        let buffers = Self::unpack_buffers(&gltf);

        let mut root_nodes: Vec<Node> = Vec::new();

        // make sure render pipeline exists
        create_pipeline! {
            NAME => "forte.gltf",
            ENGINE => engine,
            SHADER => gltf_shader::SOURCE,
            BUFFER_LAYOUTS => [Vertex::desc(), TransformRaw::desc()],
            BIND_GROUPS => [Camera::BIND_LAYOUT, Material::BIND_LAYOUT, LightUniform::BIND_LAYOUT],
            HAS_DEPTH => true
        }

        // load nodes
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                // load mesh
                let node = Self::unpack_node(engine, &buffers, &node);
                root_nodes.push(node);
            }
        }

        return Model { nodes: root_nodes };
    }

    fn unpack_node<'a>(engine: &RenderEngine, buffers: &Vec<Vec<u8>>, node: &gltf::Node<'a>) -> Node {
        Node { 
            meshes: if node.mesh().is_some() { Some(Self::unpack_mesh(engine, buffers, &node.mesh().unwrap())) } else { None }, 
            children: node.children().into_iter().map(|child| Self::unpack_node(engine, buffers, &child)).collect()
        }
    }

    fn unpack_mesh<'mesh>(engine: &RenderEngine, buffers: &Vec<Vec<u8>>, mesh: &gltf::Mesh<'mesh>) -> Vec<(Mesh, Material)> {
        mesh.primitives().into_iter().map(|primitive| {
            // read everything from the primitive
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let mut positions = reader.read_positions().expect("Expecting positions in gltf.");
            let mut normals = reader.read_normals().expect("Expecting normals in gltf.");
            let mut tex_coords = reader.read_tex_coords(0).expect("Expecting TexCoords(0) in gltf.").into_f32();
            let indices: Vec<u16> = reader.read_indices().expect("Expecting indices in gltf.").into_u32().map(|a| a as u16).collect();

            // convert positions, normals and tex coords into a vertices array
            let mut vertices: Vec<Vertex> = Vec::with_capacity(positions.len());
            while let Some(position) = positions.next() {
                let normal = normals.next().unwrap();
                let tex_coord = tex_coords.next().unwrap();
                vertices.push(Vertex { position: position.into(), tex_coords: tex_coord.into(), normal: normal.into() })
            }

            // load basic color texture
            let material = primitive.material();
            let pbr = material.pbr_metallic_roughness();

            // create material
            let material = MaterialBuilder {
                albedo_texture: gltf_texture_to_texture(engine, pbr.base_color_texture(), buffers),
                roughness_texture: gltf_texture_to_texture(engine, pbr.metallic_roughness_texture(), buffers),
                emissive_texture: gltf_texture_to_texture(engine, material.emissive_texture(), buffers),
                normal_texture: gltf_texture_to_texture(engine, material.normal_texture(), buffers),
                occlusion_texture: gltf_texture_to_texture(engine, material.normal_texture(), buffers),
                albedo_color: color_from_4f32(pbr.base_color_factor()),
                emissive_color: color_from_3f32(material.emissive_factor()),
                metallic_factor: pbr.metallic_factor(),
                roughness_factor: pbr.roughness_factor(),
                alpha_mode: match material.alpha_mode() {
                    gltf::material::AlphaMode::Opaque => 1.0,
                    gltf::material::AlphaMode::Mask => 2.0,
                    gltf::material::AlphaMode::Blend => 3.0,
                },
                alpha_cutoff: if material.alpha_cutoff().is_some() { material.alpha_cutoff().unwrap() } else { 0.0 },
            }.build(engine);

            (Mesh::from_raw(&engine.device, &vertices, &indices), material)
        }).collect()
    }

    fn unpack_buffers(gltf: &Gltf) -> Vec<Vec<u8>> {
        let mut buffer_data = Vec::new();
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Uri(_uri) => todo!(),
                /*{
                    let uri = percent_encoding::percent_decode_str(uri)
                        .decode_utf8()
                        .unwrap();
                    let uri = uri.as_ref();
                    let buffer_bytes = match DataUri::parse(uri) {
                        Ok(data_uri) if VALID_MIME_TYPES.contains(&data_uri.mime_type) => {
                            data_uri.decode()?
                        }
                        Ok(_) => return Err(GltfError::BufferFormatUnsupported),
                        Err(()) => {
                            // TODO: Remove this and add dep
                            // let buffer_path = load_context.path().parent().unwrap().join(uri);
                            // load_context.read_asset_bytes(buffer_path).await?
                        }
                    };
                    buffer_data.push(buffer_bytes);
                }*/
                gltf::buffer::Source::Bin => {
                    if let Some(blob) = gltf.blob.as_deref() {
                        buffer_data.push(blob.into());
                    }
                }
            }
        }
        return buffer_data;
    }
}

fn gltf_texture_to_texture<'a, T: AsRef<gltf::texture::Texture<'a>>>(engine: &RenderEngine, gltf_texture: Option<T>, buffers: &Vec<Vec<u8>>) -> Option<Texture> {
    if gltf_texture.is_some() {
        let gltf_texture = gltf_texture.unwrap();
        let gltf_texture = gltf_texture.as_ref();
        match gltf_texture.source().source() {
            gltf::image::Source::View { view, .. } => {
                let start = view.offset();
                let end = view.offset() + view.length();
                let buffer = &buffers[view.buffer().index()][start..end];
                let texture = Texture::from_bytes(&engine.device, &engine.queue, buffer, "")
                    .expect("Could not load gltf texture!");
                Some(texture)
            },
            gltf::image::Source::Uri { .. } => todo!(),
        }
    } else { None }
}

fn color_from_4f32(input: [f32; 4]) -> Color {
    Color { red: input[0], green: input[1], blue: input[2], alpha: input[3] }
}

fn color_from_3f32(input: [f32; 3]) -> Color {
    Color { red: input[0], green: input[1], blue: input[2], alpha: 1.0 }
}
