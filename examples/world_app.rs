use cgmath::{Quaternion, Rotation3};
use forte_engine::{run_world, render::{pipelines::Pipeline, primitives::{cameras::Camera, mesh::Mesh, transforms::TransformRaw, vertices::Vertex}, render_engine::RenderEngine, resources::Handle, textures::Texture}};
use wgpu::util::DeviceExt;

run_world!(
    TestWorldApp,
    [
        Cube => {
            DATA => (Handle<Mesh>, Handle<Texture>, Vec<Transform>, wgpu::Buffer),
            ADDED => |_: &mut TestWorldApp, _: &mut Node| {},
            UPDATE => |_: &mut TestWorldApp, _: &mut Node| {},
            RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestWorldApp, data: &'b (Handle<Mesh>, Handle<Texture>, Vec<Transform>, wgpu::Buffer)| {
                // update instance buffer
                let instance_data = data.2.iter().map(TransformRaw::from_generic).collect::<Vec<_>>();
                app.render_engine.queue.write_buffer(&data.3, 0, bytemuck::cast_slice(&instance_data));

                // draw mesh
                app.pipeline.bind(pass);
                app.camera.bind(pass, 0);
                app.render_engine.draw_textured_mesh(pass, &data.0, &data.1, &data.3, data.2.len() as u32);
            },
            REMOVED => |_: &mut TestWorldApp, _: &mut Node| {}
        }
    ]
);

pub struct TestWorldApp {
    render_engine: RenderEngine,
    pipeline: Pipeline,
    camera: Camera
}

impl WorldApp for TestWorldApp {
    fn render_engine(&self) ->  &RenderEngine { &self.render_engine }
    fn render_engine_mut(&mut self) ->  &mut RenderEngine { &mut self.render_engine }

    fn create(mut render_engine: RenderEngine) -> Self {
        // create render pipeline
        let pipeline = Pipeline::new(
            "std", &render_engine, include_str!("rotating_cube.wgsl"),
            &[Vertex::desc(), TransformRaw::desc()],
            &[
                &render_engine.device.create_bind_group_layout(&Camera::BIND_LAYOUT),
                &render_engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT),
            ]
        );

        // create camera stuffs
        let mut camera = Camera::new(
            &render_engine, 
            render_engine.config.width as f32 / render_engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 0.0, 5.0).into();
        camera.update(&mut render_engine);

        Self { 
            render_engine,
            pipeline,
            camera
        }
    }

    fn start(&mut self, root: &mut Node) {
        // create instance info
        let instances = vec![
            Transform {
                position: cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
                scale: (1.0, 1.0, 1.0).into()
            },
            Transform {
                position: cgmath::Vector3 { x: -1.0, y: 0.0, z: 0.0 },
                rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
                scale: (1.0, 1.0, 1.0).into()
            },
            Transform {
                position: cgmath::Vector3 { x: 1.0, y: 0.0, z: 0.0 },
                rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
                scale: (1.0, 1.0, 1.0).into()
            },
        ];
        let instance_data = instances.iter().map(TransformRaw::from_generic).collect::<Vec<_>>();
        let instance_buffer = self.render_engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        // create model node
        let mut model = Node::default();
        model.component = Component::Cube((
            self.render_engine.create_mesh("test", VERTICES, INDICES),
            self.render_engine.create_texture("test", include_bytes!("rotating_cube.png")),
            instances,
            instance_buffer
        ));
        model.rel_min_dimensions = Dimensions { from: Vector3 { x: -1.0, y: -1.0, z: -1.0 }, to: Vector3 { x: 1.0, y: 1.0, z: 1.0 } };
        root.add_child(self, model);
    }

    fn update(&mut self, root: &mut Node) {
        recr_rotate(root, self.render_engine.time_since_start);
    }

    fn exit(&mut self, _root: &mut Node) {
        println!("Exit");
    }
}

fn recr_rotate(node: &mut Node, time: f32) {
    match &mut node.component {
        Component::Cube((_, _, transforms, _)) => {
            for i in 0 .. transforms.len() {
                transforms[i].rotation = Quaternion::from_angle_y(cgmath::Deg(time * 45.0)) * Quaternion::from_angle_z(cgmath::Deg(time * 45.0));
            }
        }
        _ => {}
    }

    node.children.iter_mut().for_each(|build| recr_rotate(build, time));
}

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