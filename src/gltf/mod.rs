use gltf::Gltf;

use crate::{primitives::{mesh::Mesh, textures::Texture, vertices::Vertex}, render::render_engine::RenderEngine};

use self::model::{Model, Node};

pub mod model;

pub struct GLTFLoader;

impl GLTFLoader {
    pub fn unpack_static_gltf(engine: &RenderEngine, gltf: Gltf) -> Model {
        let buffers = Self::unpack_buffers(&gltf);

        let mut root_nodes: Vec<Node> = Vec::new();

        // load nodes
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                // load mesh
                let node = Self::unpack_node(engine, &buffers, &node);
                root_nodes.push(node);
            }
        }

        /*let mut textures = gltf.materials().map(|mat| {
            let pbr = mat.pbr_metallic_roughness();
            let color: Vec<u8> = pbr.base_color_factor().iter().map(|a| (a * 255.0) as u8).collect();
            println!("Color {:?} texture {:?}", pbr.base_color_factor(), color_texture);
            Texture::from_raw(
                &engine.device, 
                &engine.queue, 
                &color, 
                (1, 1), 
                None
            ).unwrap()
        });*/

        // let mut textures = gltf.textures().map(|texture| {
        //     // let buffer = &buffers[texture..buffer().index()][start..end];
        //     match texture.source().source() {
        //         gltf::image::Source::View { view, mime_type } => {
        //             let start = view.offset();
        //             let end = view.offset() + view.length();
        //             let buffer = &buffers[view.buffer().index()][start..end];
        //             Texture::from_bytes(&engine.device, &engine.queue, buffer, "").expect("Could not load gltf texture!")
        //         },
        //         gltf::image::Source::Uri { uri, mime_type } => todo!(),
        //     }
        // });

        // println!("We have {} textures.", textures.len());
        // let texture = textures.next().expect("Did not have atleast 1 texture in GLTF.");

        return Model { nodes: root_nodes };
    }

    fn unpack_node<'a>(engine: &RenderEngine, buffers: &Vec<Vec<u8>>, node: &gltf::Node<'a>) -> Node {
        Node { 
            meshes: if node.mesh().is_some() { Some(Self::unpack_mesh(engine, buffers, &node.mesh().unwrap())) } else { None }, 
            children: node.children().into_iter().map(|child| Self::unpack_node(engine, buffers, &child)).collect()
        }
    }

    fn unpack_mesh<'mesh>(engine: &RenderEngine, buffers: &Vec<Vec<u8>>, mesh: &gltf::Mesh<'mesh>) -> Vec<(Mesh, Texture)> {
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
            let pbr = primitive.material().pbr_metallic_roughness();
            let color: Vec<u8> = pbr.base_color_factor().iter().map(|a| (a * 255.0) as u8).collect();
            let texture = Texture::from_raw(
                &engine.device, 
                &engine.queue, 
                &color, 
                (1, 1), 
                None
            ).unwrap();

            (Mesh::from_raw(&engine.device, &vertices, &indices), texture)
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