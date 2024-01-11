use std::time::SystemTime;

use cgmath::Point2;
use winit::{window::Window, event::WindowEvent};

use crate::render::{RenderEngineApp, primitives::{mesh::Mesh, cameras::Camera, vertices::Vertex}, textures::{textures::Texture, depth_textures::DepthTexture}, pipelines::Pipeline, resources::{ResourceCache, Handle}, files::Files};

#[derive(Debug)]
pub struct RenderEngine {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    mesh_cache: ResourceCache<Mesh>,
    texture_cache: ResourceCache<Texture>,
    pub depth_texture: DepthTexture,
    pub(crate) start_time: u128,
    pub time_since_start: f32,
    pub delta_time: f32,

    pub window: Window // must be declared after surface due to unsafe code in windows resources
}

impl RenderEngine {
    /// Get a reference to the WGPU window used by the render engine.
    pub fn window(&self) -> &Window { &self.window }

    /// Get a texture from the texture cache using a handle.
    /// 
    /// Arguments
    /// * handle - The resource handle that will be used to get the texture from the cache.
    pub fn texture(&self, handle: &Handle<Texture>) -> &Texture { self.texture_cache.get(handle).unwrap() }

    /// Get a mesh from the mesh cache using a handle.
    /// 
    /// Arguments
    /// * handle - The resource handle that will be used to get the mesh from the cache.
    pub fn mesh(&self, handle: &Handle<Mesh>) -> &Mesh { self.mesh_cache.get(handle).unwrap() }
    
    /// Create a new render engine using the given WGPU window.
    /// 
    /// Arguments
    /// * window - The WGPU window that will be used to create this render engine.
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // create surface
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // create adapter
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();

        // create device and queue
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::VERTEX_WRITABLE_STORAGE,
                limits: wgpu::Limits::default(),
                label: None
            },
            None
        ).await.unwrap();

        // configure surface
        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities.formats.iter().copied()
            .filter(|f| f.is_srgb()).next()
            .unwrap_or(capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: format, width: size.width, height: size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![]
        };
        surface.configure(&device, &config);

        // setup depth texture
        let depth_texture = DepthTexture::new(&config, &device, "depth_texture");

        // setup timing
        let now = SystemTime::now();
        let start_time = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();

        // create light buffer
        // let default_light = LightUniform::new([2.0, 2.0, 2.0], [1.0, 1.0, 1.0]);
        // let default_lights = [
        //     LightUniform::new([2.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        //     LightUniform::new([0.0, 2.0, 0.0], [0.0, 1.0, 0.0]),
        //     LightUniform::new([0.0, 0.0, 2.0], [0.0, 0.0, 1.0]),
        // ];

        Self {
            window, surface, device,
            queue, config, size, depth_texture,
            start_time,
            time_since_start: 0.0,
            delta_time: 0.0,
            mesh_cache: ResourceCache::new(),
            texture_cache: ResourceCache::new()
        }
    }
    
    /// Handle inputs to the given window, returning true if the event is handled.
    /// This automatically processes and passes appropriate inputs to the given app.
    /// 
    /// Arguments
    /// * app - The app to which inputs should be passed too.
    pub fn input(&mut self, app: &mut Box<impl RenderEngineApp + 'static>, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                app.input(
                    self, 
                    RenderEngineInput::MouseMove(
                        Point2 { 
                            x: position.x as f32, 
                            y: position.y as f32 
                        }
                    )
                );
                true
            }

            WindowEvent::MouseInput { state, button, .. } => {
                app.input(
                    self,
                    RenderEngineInput::MouseButton(*button, *state)
                );
                true
            }

            WindowEvent::MouseWheel { delta, .. } => {
                app.input(self, RenderEngineInput::MouseWheel(*delta));
                true
            },

            WindowEvent::KeyboardInput { input, .. } => {
                if input.virtual_keycode.is_some() {
                    app.input(
                        self,
                        RenderEngineInput::KeyInput(input.virtual_keycode.unwrap(), input.state)
                    );
                }
                true
            },

            _ => false
        }
    }

    /// Resizes all resources used for rendering to the new size given.
    /// 
    /// Arguments
    /// * new_size - The new size that this render engine should resize its resources too.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // if size is valid, reconfigure surface
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.depth_texture = DepthTexture::new(&self.config, &self.device, "depth_texture");
        }
    }

    /// Starts a render cycle.  During this cycle, the given apps render function will be called.
    /// 
    /// Arguments
    /// * app - The app whose render functino should be called
    /// 
    /// Returns a result with an error from wgpu if it occurs, this will return nothing if no errors occur.
    pub fn render(&mut self, app: &mut Box<impl RenderEngineApp + 'static>) -> Result<(), wgpu::SurfaceError> {
        // create output, view, and encoder
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // call app render
        app.render(self, &view, &mut encoder);

        // render the queue and wrap up
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // update time since start and delta time
        let now = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let old_time = self.time_since_start;
        let ms_since_start = if now > self.start_time { now - self.start_time } else { 0 };
        self.time_since_start = ms_since_start as f32 / 1000.0;
        self.delta_time = self.time_since_start - old_time;

        Ok(())
    }

    /// Creates a texture from the given bytes and the given path ID
    /// 
    /// Arguments
    /// * path - The path ID so that this texture can be identified in the cache
    /// * bytes - The bytes of the png/jpg file to be loaded into the texture
    /// 
    /// Returns a resource handle for the texture
    pub fn create_texture(&mut self, path: impl Into<String>, bytes: &[u8]) -> Handle<Texture> { 
        self.texture_cache.load(path, || { 
            Texture::from_bytes(
                &self.device, 
                &self.queue, 
                bytes, 
                "diffuse"
            ).unwrap() 
        })
    }

    /// Creates a texture from the given local path
    /// 
    /// Arguments
    /// * path - The local path to the file to be loaded for the texture
    /// 
    /// Returns a resource handle for the texture
    pub fn load_texture(&mut self, path: &str) -> Handle<Texture> {
        self.texture_cache.load(path, || {
            Texture::from_bytes(
                &self.device, 
                &self.queue, 
                &Files::load_bytes(path).expect("Failed to load file"), 
                "diffuse"
            ).unwrap()
        }) 
    }
    
    /// Creates a mesh from the given vertices and indices
    /// 
    /// Arguments
    /// * path - The path ID so that this mesh can be identified in the cache
    /// * vertices - An array of vertices for the mesh
    /// * indices - An array of indices for the mesh
    /// 
    /// Returns a resource handle for the mesh
    pub fn create_mesh(&mut self, path: &str, vertices: &[Vertex], indices: &[u16]) -> Handle<Mesh> { 
        self.mesh_cache.load(path, || { 
            Mesh::from_raw(&self.device, vertices, indices) 
        }) 
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RenderEngineInput {
    MouseMove(Point2<f32>),
    MouseButton(winit::event::MouseButton, winit::event::ElementState),
    MouseWheel(winit::event::MouseScrollDelta),
    KeyInput(winit::event::VirtualKeyCode, winit::event::ElementState)
}

pub trait DrawMesh<'a, 'b> where 'b: 'a {
    fn prepare_draw(
        &mut self,
        pipeline: &'b Pipeline,
        camera: &'b Camera,
    );

    fn draw_mesh(
        &mut self,
        engine: &'b RenderEngine,
        mesh: &'b Handle<Mesh>,
        texture: &'b Handle<Texture>,
        instance_buf: &'b wgpu::Buffer,
        instance_count: u32
    );

    fn draw_list_mesh(
        &mut self,
        engine: &'b RenderEngine,
        mesh: &'b Handle<Mesh>,
        texture: &'b Handle<Texture>,
        instance_buf: &'b wgpu::Buffer,
        instance_count: u32
    );
}

impl<'a, 'b> DrawMesh<'a, 'b> for wgpu::RenderPass<'a> where 'b: 'a {
    fn draw_mesh(
        &mut self,
        engine: &'b RenderEngine,
        mesh: &'b Handle<Mesh>,
        texture: &'b Handle<Texture>,
        instance_buf: &'b wgpu::Buffer,
        instance_count: u32
    ) {
        let texture = engine.texture_cache.get(texture).unwrap();
        let mesh = engine.mesh_cache.get(mesh).unwrap();
        self.set_bind_group(1, &texture.bind_group, &[]);
        self.set_vertex_buffer(0, mesh.vertex_buf.slice(..));
        self.set_vertex_buffer(1, instance_buf.slice(..));
        self.set_index_buffer(mesh.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        self.draw_indexed(0..mesh.num_indices, 0, 0..instance_count);
    }

    fn prepare_draw(
        &mut self,
        pipeline: &'b Pipeline,
        camera: &'b Camera,
    ) {
        self.set_pipeline(&pipeline.render_pipeline);
        self.set_bind_group(0, &camera.bind_group, &[]);
    }

    fn draw_list_mesh(
        &mut self,
        engine: &'b RenderEngine,
        mesh: &'b Handle<Mesh>,
        texture: &'b Handle<Texture>,
        instance_buf: &'b wgpu::Buffer,
        instance_count: u32
    ) {
        let texture = engine.texture_cache.get(texture).unwrap();
        let mesh = engine.mesh_cache.get(mesh).unwrap();
        self.set_bind_group(1, &texture.bind_group, &[]);
        self.set_vertex_buffer(0, mesh.vertex_buf.slice(..));
        self.set_vertex_buffer(1, instance_buf.slice(..));
        self.set_index_buffer(mesh.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        self.draw(0 .. mesh.num_vertices, 0..instance_count);
    }
}
