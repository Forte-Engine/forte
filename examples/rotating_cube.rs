use cgmath::{Rotation3, Quaternion};
use forte_engine::{end_render, math::transforms::Transform, pass, render::{primitives::{vertices::Vertex, mesh::Mesh, cameras::{CameraController, Camera}, transforms::TransformRaw}, pipelines::Pipeline, resources::Handle, textures::textures::Texture, render_engine::*, input::EngineInput, render_utils}, run_app, start_render, EngineApp};
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

#[derive(Debug)]
pub struct MainApp { 
    render_engine: RenderEngine,
    pipeline: Pipeline,
    mesh: Handle<Mesh>, 
    texture: Handle<Texture>, 
    camera: Camera, 
    controller: CameraController,

    instances: Vec<Transform>,
    instance_buffer: wgpu::Buffer
}

impl EngineApp for MainApp {
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

        // generate camera
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 0.0, 5.0).into();
        camera.update(&mut engine);
        let camera_controller = CameraController::new(0.02);

        // create instances
        let instances = vec![Transform {
            position: cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
            scale: (1.0, 1.0, 1.0).into()
        }];

        // create instance buffer
        let instance_data = instances.iter().map(TransformRaw::from_generic).collect::<Vec<_>>();
        let instance_buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        let mesh = engine.create_mesh("test", VERTICES, INDICES);
        let texture = engine.create_texture("test", include_bytes!("rotating_cube.png"));

        // create instance of self
        Self {
            render_engine: engine,
            mesh, texture,
            camera, pipeline,
            controller: camera_controller,
            instances, instance_buffer
        }
    }

    fn input(&mut self, input: EngineInput) {
        self.controller.input(&input);

        // display all inputs except mouse move
        match input {
            EngineInput::MouseMove(..) => {}
            _ => println!("Received input {:?}", input)
        }
    }

    fn update(&mut self) {
        // update the camera and its controller
        self.controller.update_camera(&mut self.camera);
        self.camera.update(&mut self.render_engine);

        // start render and get resources
        let mut resources = start_render!(self.render_engine);

        // create rener pass
        {
            let mut pass = pass!(
                self.render_engine, 
                resources
            );

            let transform = self.instances.get_mut(0).unwrap();
            transform.rotation = Quaternion::from_angle_y(cgmath::Deg(self.render_engine.time_since_start * 45.0)) * Quaternion::from_angle_z(cgmath::Deg(self.render_engine.time_since_start * 45.0));
            let instance_data = self.instances.iter().map(TransformRaw::from_generic).collect::<Vec<_>>();
            self.render_engine.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instance_data));

            pass.prepare_draw(&self.pipeline, &self.camera);
            pass.draw_mesh(&self.render_engine, &self.mesh, &self.texture, &self.instance_buffer, self.instances.len() as u32);
        }

        // end render
        end_render!(self.render_engine, resources);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }
    fn events_cleared(&mut self) { self.render_engine.next_frame(); }

    fn exit(&mut self) {}
}

fn main() { run_app::<MainApp>(); }
