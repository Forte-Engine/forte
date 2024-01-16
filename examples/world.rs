use cgmath::{Rotation3, Quaternion};
use forte_engine::{render::{render_engine::{RenderEngine, DrawMesh}, primitives::{cameras::{Camera, CameraController}, mesh::Mesh, vertices::Vertex, transforms::TransformRaw}, render_utils, resources::Handle, textures::textures::Texture, pipelines::Pipeline}, lights::{LightEngine, SetupLights}, EngineApp, run_app, define_world};
use wgpu::util::DeviceExt;

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



define_world!(
    TestApp,
    Component,
    Node,
    DrawNode,
    draw_node,
    [
        Cube => {
            DATA => (Handle<Mesh>, Handle<Texture>, Vec<Transform>, wgpu::Buffer),
            ADDED => |_: &mut Node| {},
            UPDATE => |_: &mut Node| {},
            RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestApp, data: &'b (Handle<Mesh>, Handle<Texture>, Vec<Transform>, wgpu::Buffer)| {
                // update instance buffer
                let instance_data = data.2.iter().map(TransformRaw::from_generic).collect::<Vec<_>>();
                app.render_engine.queue.write_buffer(&data.3, 0, bytemuck::cast_slice(&instance_data));

                // draw mesh
                pass.prepare_draw(&app.pipeline, &app.camera);
                pass.draw_mesh(&app.render_engine, &data.0, &data.1, &data.3, data.2.len() as u32);
            },
            REMOVED => |_: &mut Node| {}
        }
    ]
);

pub struct TestApp {
    render_engine: RenderEngine,
    light_engine: LightEngine,
    pipeline: Pipeline,
    camera: Camera,
    controller: CameraController,
    root: Node
}

impl EngineApp for TestApp {
    fn create(mut engine: RenderEngine) -> Self {
        // create render pipeline
        let pipeline = Pipeline::new(
            "std", &engine, include_str!("rotating_cube.wgsl"),
            &[Vertex::desc(), TransformRaw::desc()],
            &[
                &engine.device.create_bind_group_layout(&Camera::BIND_LAYOUT),
                &engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT),
            ]
        );

        // create engines
        let light_engine = LightEngine::new(&engine, [0.1, 0.1, 0.1]);
        // let cube_engine = CubeEngine::new(&mut engine);

        // create camera stuffs
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 0.0, 5.0).into();
        camera.update(&mut engine);
        let controller = CameraController::new(0.02);

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
        let instance_buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        // create nodes
        let mut root = Node::default();
        let mut model = Node::default();
        model.component = Component::Cube((
            engine.create_mesh("test", VERTICES, INDICES),
            engine.create_texture("test", include_bytes!("rotating_cube.png")),
            instances,
            instance_buffer
        ));
        model.rel_min_dimensions = Dimensions { from: Vector3 { x: -1.0, y: -1.0, z: -1.0 }, to: Vector3 { x: 1.0, y: 1.0, z: 1.0 } };
        root.add_child(model);

        // create final app
        Self {
            render_engine: engine,
            light_engine, pipeline, 
            root,
            camera, controller
        }
    }

    fn update(&mut self) {
        self.root.update(&Transform::default());
        recr_rotate(&mut self.root, self.render_engine.time_since_start);

        // start render
        let resources = render_utils::prepare_render(&self.render_engine);
        let mut resources = if resources.is_ok() { resources.unwrap() } else { return };

        {
            // create render pass
            let mut pass = resources.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &resources.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.render_engine.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store
                    }),
                    stencil_ops: None
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // setup environment
            pass.load_lights(&self.light_engine);

            // have nodes render to renderables
            pass.draw_node(&self, &self.root);
        }

        // end render
        render_utils::finalize_render(&mut self.render_engine, resources);
    }

    fn input(&mut self, input: forte_engine::render::input::EngineInput) {
        self.controller.input(&input);
        self.controller.update_camera(&mut self.camera);
        self.camera.update(&mut self.render_engine);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }
    fn events_cleared(&mut self) { self.render_engine.next_frame(); }
    fn exit(&mut self) {}
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

fn main() { run_app::<TestApp>(); }
