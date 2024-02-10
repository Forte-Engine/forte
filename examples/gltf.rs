use cgmath::Quaternion;
use forte_engine::{component_app::EngineComponent, create_app, gltf::{model::{Model, Node}, GLTFLoader}, lights::{lights::LightUniform, LightEngine}, math::{quaternion::QuaternionExt, transforms::Transform}, primitives::{cameras::Camera, textures::Texture, transforms::TransformRaw, vertices::Vertex}, render::pipelines::Pipeline, run_app};
use gltf::Gltf;

pub struct TestComponent {
    camera: Camera,
    model: Model,
    instance_buffer: wgpu::Buffer
}

#[include_wgsl_oil::include_wgsl_oil("../shaders/gltf.wgsl")]
mod gltf_shader {}

impl EngineComponent<(&mut RenderEngine, &mut LightEngine)> for TestComponent {

    fn create(engine: &mut RenderEngine) -> Self { 
        // generate camera
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 0.0, 5.0).into();
        camera.update(engine);

        // create instances
        let instances = vec![Transform {
            position: cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: cgmath::Quaternion::euler_deg_z(0.0),
            scale: (1.0, 1.0, 1.0).into()
        }];

        engine.verify_pipeline_exists("forte.gltf", |engine| {
            Pipeline::new(
                "std", &engine, gltf_shader::SOURCE,
                &[Vertex::desc(), TransformRaw::desc()],
                &[
                    &engine.device.create_bind_group_layout(&Camera::BIND_LAYOUT),
                    &engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT),
                    &engine.device.create_bind_group_layout(&LightUniform::BIND_LAYOUT),
                ],
                true
            )
        });

        let gltf = GLTFLoader::unpack_static_gltf(&engine, Gltf::open("examples/mine.gltf.glb").expect("Could not load gltf!"));

        Self {
            instance_buffer: TransformRaw::buffer_from_generic(engine, &instances),
            model: gltf,
            camera
        }
    }

    fn start(&mut self, (_, light_engine): (&mut RenderEngine, &mut LightEngine)) {
        light_engine.set_ambient_color([0.5, 0.5, 0.5]);
        light_engine.add_light(0, LightUniform::new(
            [
                5.0, 
                5.0, 
                5.0
            ], 
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            f32::MAX, 1.0, 1000.0
        ));
    }

    fn update(&mut self, (engine, _): (&mut RenderEngine, &mut LightEngine)) {
        // update rotation
        TransformRaw::update_buffer_generic(
            engine, &self.instance_buffer, 
            &[Transform {
                position: [0.0, -1.0, 0.0].into(),
                rotation: Quaternion::euler_deg(0.0, engine.time_since_start * 45.0, 0.0),
                ..Default::default()
            }]
        );
    }
    
    fn render<'rpass>(&'rpass mut self, _: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>) {
        self.camera.bind(pass, 0);
        self.model.nodes.iter().for_each(|node| render_nodes(pass, node, &self.instance_buffer));
    }

    fn exit(&mut self, _: (&mut RenderEngine, &mut LightEngine)) {}
}

fn render_nodes<'rpass>(pass: &mut wgpu::RenderPass<'rpass>, node: &'rpass Node, instances: &'rpass wgpu::Buffer) {
    if let Some(meshes) = &node.meshes {
        meshes.iter().for_each(|drawn| {
            drawn.1.bind(pass, 1);
            drawn.0.draw(pass, instances, 1);
        });
    }
}

create_app!(
    CLEAR_COLOR = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },

    APP {
        light_engine: LightEngine[render_engine],
        test: TestComponent[render_engine, light_engine]
    },

    PASSES {
        0: {
            PIPELINE: "forte.gltf",
            PREPARE: [light_engine],
            RENDER: test,
            DEPTH: true
        }
    }
);

fn main() { run_app::<App>() }