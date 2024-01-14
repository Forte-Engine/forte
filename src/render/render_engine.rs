use std::time::SystemTime;

use winit::{window::Window, event::WindowEvent};

use crate::render::{RenderEngineApp, primitives::{mesh::Mesh, cameras::Camera, vertices::Vertex}, textures::{textures::Texture, depth_textures::DepthTexture}, pipelines::Pipeline, resources::{ResourceCache, Handle}, files::Files};

use super::{render_utils, input::EngineInput};

/// A struct with all required information to render to a given window.
/// 
/// DO NOT try to create this object by yourself, this object will be provided to your RenderEngineApp.
/// DO NOT try to modify any values in this struct, this will only cause errors unless you know what you are doing.
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
        // convert the winit event to a `EngineInput`
        let input = EngineInput::from_winit_input(event);

        // if the input is some, call the apps input function and return true, otherwise return false
        if input.is_some() {
            app.input(self, input.unwrap());
            true
        } else { false }
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
        // start the render and get some render resources
        let mut render_resources = render_utils::prepare_render(&self)?;

        // call app render
        app.render(self, &render_resources.view, &mut render_resources.encoder);

        // finalize the render
        render_utils::finalize_render(self, render_resources);

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

/// A set of functions for RenderPass to setup cameras and draw mesh.
pub trait DrawMesh<'a, 'b> where 'b: 'a {
    /// Prepares to draw mesh by setting up a pipeline and a camera to render with.
    /// 
    /// Arguments:
    /// * pipeline: &Pipeline - The pipeline that will be used to render.
    /// * camera: &Camera - The camera to be used to render.
    fn prepare_draw(
        &mut self,
        pipeline: &'b Pipeline,
        camera: &'b Camera,
    );

    /// Draws a mesh to this render pass.
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine that will be used to draw this.
    /// * mesh: &Handle<Mesh> - A handle to the mesh to be drawn.
    /// * texture: &Handle<Texture> - A handle to the texture the mesh will be drawn with.
    /// * instance_buf: &wgpu::Buffer - The instances buffer to draw the mesh with.
    /// * instance_count: u32 - The number of instances in the above buffer.
    fn draw_mesh(
        &mut self,
        engine: &'b RenderEngine,
        mesh: &'b Handle<Mesh>,
        texture: &'b Handle<Texture>,
        instance_buf: &'b wgpu::Buffer,
        instance_count: u32
    );

    /// Draws a mesh to this render pass only using its vertices buffer.
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine that will be used to draw this.
    /// * mesh: &Handle<Mesh> - A handle to the mesh to be drawn.  Only vertices will be used, the indices buffer will be disregarded.
    /// * texture: &Handle<Texture> - A handle to the texture to draw the mesh with.
    /// * instance_buf: &wgpu::Buffer - The instances buffer to draw the mesh with.
    /// * instance_count: u32 - The number of instances in the above buffer.
    fn draw_list_mesh(
        &mut self,
        engine: &'b RenderEngine,
        mesh: &'b Handle<Mesh>,
        texture: &'b Handle<Texture>,
        instance_buf: &'b wgpu::Buffer,
        instance_count: u32
    );
}

/// An implementation of DrawMesh for wgpu::RenderPass.  See documentation for more information.
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
