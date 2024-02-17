use cgmath::Quaternion;
use forte_engine::{component_app::EngineComponent, create_app, egui::EguiEngine, math::{quaternion::QuaternionExt, transforms::Transform}, primitives::{cameras::Camera, mesh::Mesh, textures::Texture, transforms::TransformRaw, vertices::Vertex}, render::pipelines::Pipeline, run_app, ui::{elements::UIElement, style::{Color, PositionSetting, Sizing, Style}, UIEngine}, utils::resources::Handle};

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
    mesh: Handle<Mesh>, 
    texture: Handle<Texture>, 
    camera: Camera,
    instances: Vec<Transform>,
    instance_buffer: wgpu::Buffer,
    test: String
}

impl EngineComponent<(&mut RenderEngine, &mut UIEngine, &mut EguiEngine)> for TestComponent {

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

        engine.verify_pipeline_exists("forte.test", |engine| {
            Pipeline::new(
                "std", &engine, include_str!("rotating_cube.wgsl"),
                &[Vertex::desc(), TransformRaw::desc()],
                &[
                    &engine.device.create_bind_group_layout(&Camera::BIND_LAYOUT),
                    &engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT),
                ],
                true
            )
        });

        Self {
            instance_buffer: TransformRaw::buffer_from_generic(engine, &instances),
            mesh: engine.create_mesh("test", VERTICES, INDICES),
            texture: engine.create_texture("test", include_bytes!("rotating_cube.png")),
            camera, instances,
            test: "".to_string()
        }
    }

    fn start(&mut self, (engine, ui, _): (&mut RenderEngine, &mut UIEngine, &mut EguiEngine)) {
        let mut a = UIElement::container(
            &engine, 
            Style { 
                width: Sizing::Px(200.0), 
                height: Sizing::Px(200.0), 
                color: Color { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 },
                top: Sizing::Px(10.0),
                left: Sizing::Px(10.0),
                border: Sizing::Px(5.0),
                round: Sizing::Px(15.0),
                ..Default::default() 
            }
        );
        a.children.push(UIElement::container(
            &engine, 
            Style {
                width: Sizing::Px(100.0), 
                height: Sizing::Px(100.0), 
                position_setting: PositionSetting::Parent,
                top: Sizing::Px(10.0),
                left: Sizing::Px(10.0),
                border: Sizing::Px(5.0),
                round: Sizing::Px(10.0),
                color: Color { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 },
                ..Default::default() 
            }
        ));
        ui.elements.push(a);
    }

    fn update(&mut self, (engine, _, egui): (&mut RenderEngine, &mut UIEngine, &mut EguiEngine)) {
        // update rotation
        TransformRaw::update_buffer_generic(
            engine, &self.instance_buffer, 
            &[Transform {
                rotation: Quaternion::euler_deg(0.0, engine.time_since_start * 45.0, engine.time_since_start * 45.0),
                ..Default::default()
            }]
        );
        
        // test window
        egui::Window::new("Test")
            .show(egui.context(), |ui| { 
                ui.label("Hi from test window!");
                ui.text_edit_singleline(&mut self.test); 
                if ui.button("Search").clicked() {
                    println!("Search for {}", self.test);
                }
            });
    }
    
    fn render<'rpass>(&'rpass mut self, engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>) {
        // draw
        self.camera.bind(pass, engine, 0);
        engine.draw_textured_mesh(pass, &self.mesh, &self.texture, &self.instance_buffer, self.instances.len() as u32);
    }

    fn exit(&mut self, _: (&mut RenderEngine, &mut UIEngine, &mut EguiEngine)) {}
}

create_app! {
    CLEAR_COLOR = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },

    APP {
        ui_engine: UIEngine[render_engine],
        test: TestComponent[render_engine, ui_engine, egui],
        egui: EguiEngine[render_engine, inputs]
    },

    PASSES {
        0: {
            PARTS: [
                {
                    PIPELINE: "forte.test",
                    PREPARE: [],
                    RENDER: test,
                }
            ],
            DEPTH: true
        },
        1: {
            PARTS: [
                {
                    PIPELINE: "forte.ui",
                    PREPARE: [],
                    RENDER: ui_engine,
                },
                {
                    PIPELINE: "forte.ui",
                    PREPARE: [],
                    RENDER: egui,
                }
            ],
            DEPTH: false
        }
    }
}

fn main() { run_app::<App>() }