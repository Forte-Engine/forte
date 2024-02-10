use std::time::SystemTime;

use winit::window::Window;

use crate::{primitives::{mesh::Mesh, textures::{Texture, depth_textures::DepthTexture}, vertices::Vertex}, utils::{files::Files, resources::{ResourceCache, Handle}}};

use super::pipelines::Pipeline;

/// A struct with all required information to render to a given window.
/// 
/// DO NOT try to create this object by yourself, this object will be provided to your RenderEngineApp.
/// DO NOT try to modify any values in this struct, this will only cause errors unless you know what you are doing.
#[derive(Debug)]
pub struct RenderEngine {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    mesh_cache: ResourceCache<Mesh>,
    texture_cache: ResourceCache<Texture>,
    pipeline_cache: ResourceCache<Pipeline>,
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

    pub fn texture_from_path(&self, path: impl Into<String>) -> &Texture { 
        self.texture_cache.get(
            &Handle { 
                hash: ResourceCache::<Texture>::hash_path(path.into()), 
                data: Default::default() 
            }
        ).unwrap() 
    }

    /// Get a mesh from the mesh cache using a handle.
    /// 
    /// Arguments
    /// * handle - The resource handle that will be used to get the mesh from the cache.
    pub fn mesh(&self, handle: &Handle<Mesh>) -> &Mesh { self.mesh_cache.get(handle).unwrap() }
    
    /// Create a new render engine using the given WGPU window.
    /// 
    /// Arguments
    /// * window - The WGPU window that will be used to create this render engine.
    pub fn new(window: Window) -> Self {
        let size = window.inner_size();

        // create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // create surface
        let surface = unsafe { instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap()) }.unwrap();

        // create adapter
        let adapter = pollster::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        )).unwrap();

        // create device and queue
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::VERTEX_WRITABLE_STORAGE,
                required_limits: wgpu::Limits::default(),
                label: None
            },
            None
        )).unwrap();

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
            view_formats: vec![],
            desired_maximum_frame_latency: 2
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
            texture_cache: ResourceCache::new(),
            pipeline_cache: ResourceCache::new()
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

    /// Requests the next frame from the window.
    pub fn next_frame(&self) { self.window().request_redraw(); }

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
    /// Arguments:
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

    /// Draws the given mesh and texture handles with the given instance buffer and count.
    /// 
    /// Arguments:
    /// &self - The render engine to draw with.
    /// mesh: &Handle<Mesh> - A handle to the mesh to draw.
    /// texture: &Handle<Texture> - A handle to the texture to draw.
    /// instance_buffer: &wgpu::Buffer - The buffer to draw.
    pub fn draw_textured_mesh<'rpass>(
        &'rpass self,
        pass: &mut wgpu::RenderPass<'rpass>,
        mesh: &'rpass Handle<Mesh>,
        texture: &'rpass Handle<Texture>,
        instance_buffer: &'rpass wgpu::Buffer,
        instance_count: u32
    ) {
        self.texture(texture).bind(pass, 1);
        self.mesh(mesh).draw(pass, instance_buffer, instance_count);
    }

    /// Draws the given mesh and texture handles with the given instance buffer and count, however, the mesh will be drawn as a list mesh.
    /// 
    /// Arguments:
    /// &self - The render engine to draw with.
    /// mesh: &Handle<Mesh> - A handle to the mesh to draw.
    /// texture: &Handle<Texture> - A handle to the texture to draw.
    /// instance_buffer: &wgpu::Buffer - The buffer to draw.
    pub fn draw_textured_list_mesh<'rpass>(
        &'rpass self,
        pass: &mut wgpu::RenderPass<'rpass>,
        mesh: &'rpass Handle<Mesh>,
        texture: &'rpass Handle<Texture>,
        instance_buffer: &'rpass wgpu::Buffer,
        instance_count: u32
    ) {
        self.texture(texture).bind(pass, 1);
        self.mesh(mesh).draw_list(pass, instance_buffer, instance_count);
    }

    pub fn register_pipeline(&mut self, path: impl Into<String>, pipeline: Pipeline) {
        self.pipeline_cache.insert(ResourceCache::<Pipeline>::hash_path(path.into()), pipeline);
    }

    pub fn verify_pipeline_exists(&mut self, path: &str, create: fn() -> Pipeline) {
        self.pipeline_cache.load(path, create);
    }
}