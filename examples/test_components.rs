use cgmath::Quaternion;
use forte_engine::{component_app::EngineComponent, math::{quaternion::QuaternionExt, transforms::Transform}, primitives::{cameras::Camera, mesh::Mesh, textures::Texture, transforms::TransformRaw, vertices::Vertex}, render::pipelines::Pipeline, run_app, utils::resources::Handle};

const VERTICES: &[Vertex] = &[
    Vertex { position: [ 0.5, -0.5, -0.5], tex_coords: [0.4131759, 0.00759614], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], tex_coords: [0.0048659444, 0.43041354], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [0.28081453, 0.949397], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.85967, 0.84732914], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], tex_coords: [0.9414737, 0.2652641], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], tex_coords: [0.28081453, 0.949397], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [0.85967, 0.84732914], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [0.9414737, 0.2652641], normal: [0.0, 0.0, 0.0] },
];

const INDICES: &[u16] = &[
    1, 2, 3,
    4, 7, 6,
    4, 5, 1,
    1, 5, 6,
    6, 7, 3,
    4, 0, 3,
    0, 1, 3,
    5, 4, 6,
    0, 4, 1,
    2, 1, 6,
    2, 6, 3,
    7, 4, 3
];

pub struct TestComponent {
    pipeline: Pipeline,
    mesh: Handle<Mesh>, 
    texture: Handle<Texture>, 
    camera: Camera,

    instances: Vec<Transform>,
    instance_buffer: wgpu::Buffer
}

// impl EngineComponent<App> for TestComponent {
//     fn create(engine: &mut RenderEngine) -> Self {
//         // create render pipeline
//         let pipeline = Pipeline::new(
//             "std", &engine, include_str!("rotating_cube.wgsl"),
//             &[Vertex::desc(), TransformRaw::desc()],
//             &[
//                 &engine.device.create_bind_group_layout(&Camera::BIND_LAYOUT),
//                 &engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT),
//             ]
//         );

//         // generate camera
//         let mut camera = Camera::new(
//             &engine, 
//             engine.config.width as f32 / engine.config.height as f32,
//             45.0, 0.1, 100.0
//         );
//         camera.position = (0.0, 0.0, 5.0).into();
//         camera.update(engine);

//         // create instances
//         let instances = vec![Transform {
//             position: cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 },
//             rotation: cgmath::Quaternion::euler_deg_z(0.0),
//             scale: (1.0, 1.0, 1.0).into()
//         }];

//         Self {
//             mesh: engine.create_mesh("test", VERTICES, INDICES), 
//             texture: engine.create_texture("test", include_bytes!("rotating_cube.png")),
//             instance_buffer: TransformRaw::buffer_from_generic(engine, &instances),
//             instances, camera, pipeline
//         }
//     }

//     fn start(_: &mut App) {}

//     fn update(components: &mut App) {
//         components.test.camera.update(&mut components.render_engine);
//         TransformRaw::update_buffer_generic(&components.render_engine, &components.test.instance_buffer, &[
//             Transform {
//                 rotation: Quaternion::euler_deg(0.0, components.render_engine.time_since_start * 45.0, components.render_engine.time_since_start * 45.0),
//                 ..Default::default()
//             }
//         ]);
//     }

//     fn render<'rpass>(&'rpass self, render_engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>) {
//         self.pipeline.bind(pass);
//         self.camera.bind(pass, 0);
//         render_engine.draw_textured_mesh(pass, &self.mesh, &self.texture, &self.instance_buffer, self.instances.len() as u32);
//     }

//     fn exit(_: &mut App) {}
// }

// create_app!(
//     COMPONENTS => [
//         test => TestComponent
//     ]

//     PASSES => [
//         0 => [test]
//     ]
// );

fn main() { /* run_app::<App>() */ }