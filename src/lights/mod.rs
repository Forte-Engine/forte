use std::collections::HashMap;

use lights::LightUniform;
use crate::render::render_engine::RenderEngine;
use wgpu::util::DeviceExt;

pub mod lights;

/// A engine used to drive all lights in the scene.
#[derive(Debug)]
pub struct LightEngine {
    // wgpu
    light_buffer: wgpu::Buffer,
    light_count_buffer: wgpu::Buffer,
    light_ambient_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,

    // internal
    lights: HashMap<u32, LightUniform>,
    ambient_color: [f32; 3],
    last_count: usize,
    dirty: bool
}

impl LightEngine {
    /// The bind group currently being used by this engine.
    pub fn bind_group(&self) -> &wgpu::BindGroup { &self.light_bind_group }

    /// Marks the light engine "dirty" so that its buffers will be updated next time this engine is updated.
    pub fn mark_dirty(&mut self) { self.dirty = true; }

    /// Adds a light to the engine.
    /// 
    /// Arguments:
    /// * id: 32 - The id of the light.
    /// * light: LightUniform - The lights information.
    pub fn add_light(&mut self, id: u32, light: LightUniform) { self.lights.insert(id, light); self.mark_dirty(); }

    /// Removes a light from the engine.
    /// 
    /// Arguments:
    /// * idx: u32 - The index of the light to be removed.
    pub fn remove_light(&mut self, idx: u32) { self.lights.remove(&idx); self.mark_dirty(); }

    /// Clears all lights from the engine.
    pub fn clear_lights(&mut self) { self.lights.clear(); self.mark_dirty(); }

    /// Sets the current ambient light color.
    /// 
    /// Arguments:
    /// * ambient: [f32; 3] - The new ambient color.
    pub fn set_ambient_color(&mut self, ambient: [f32; 3]) { self.ambient_color = ambient; }

    /// Gets teh current ambient light color.
    /// 
    /// Returns a [f32; 3] of the current ambient light color.
    pub fn get_ambient_color(&self) -> [f32; 3] { return self.ambient_color; }

    /// Creates a new light engine.
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine this light engine will be used with.
    /// * ambient_light: [f32; 3] - The ambient light color.
    pub fn new(engine: &RenderEngine, ambient_light: [f32; 3]) -> Self {
        // setup a default light
        let default_light = lights::LightUniform::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.0, 0.0, 1000.0);

        // create a light buffer
        let light_buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light Buffer"),
                contents: bytemuck::cast_slice(&[default_light]),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
            }
        );

        // create light count buffer
        let light_count_buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light Count Buffer"),
                contents: bytemuck::cast_slice(&[1]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        // create ambient light buffer
        let light_ambient_buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light Ambient Buffer"),
                contents: bytemuck::cast_slice(&ambient_light),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        // create light bind group
        let light_bind_group = engine.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &engine.device.create_bind_group_layout(&lights::LightUniform::BIND_LAYOUT),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: light_count_buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light_ambient_buffer.as_entire_binding()
                }
            ],
            label: Some("light_bind_group")
        });

        Self { 
            light_buffer, light_count_buffer, 
            light_bind_group, light_ambient_buffer,
            ambient_color: ambient_light,
            lights: HashMap::new(), last_count: 0, dirty: false 
        }
    }

    /// Updates the light engine.  This should be called once per frame while lights are in use.
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The render engine this light engine is being used with.
    pub fn update(&mut self, engine: &RenderEngine) {
        // if not marked dirty, stop here
        if !self.dirty { return }

        // update count buffer
        engine.queue.write_buffer(
            &self.light_count_buffer, 
            0, 
            bytemuck::cast_slice(&[self.lights.len() as u32])
        );

        // update ambient light buffer
        engine.queue.write_buffer(&self.light_ambient_buffer, 0, bytemuck::cast_slice(&self.ambient_color));

        // if the lights count is not the same size as the current count, create a new buffer and bind count
        if self.last_count != self.lights.len() {
            self.last_count = self.lights.len();

            // create array of lights
            let mut lights: Vec<LightUniform> = self.lights.values().cloned().collect();
            if lights.len() < 1 {
                lights.push(LightUniform::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.0, 0.0, 1000.0));
            }

            // update light buffer
            self.light_buffer = engine.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Light Buffer"),
                    contents: bytemuck::cast_slice(&lights),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
                }
            );
        
            // update bind group
            self.light_bind_group = engine.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &engine.device.create_bind_group_layout(&lights::LightUniform::BIND_LAYOUT),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.light_buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.light_count_buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.light_ambient_buffer.as_entire_binding()
                    }
                ],
                label: Some("light_bind_group")
            });
        }
        // otherwise, update light buffer
        else {
            // create array of lights
            let mut lights: Vec<LightUniform> = self.lights.values().cloned().collect();
            if lights.len() < 1 {
                lights.push(LightUniform::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.0, 0.0, 1000.0));
            }

            // update light buffer
            engine.queue.write_buffer(
                &self.light_buffer, 
                0, 
                bytemuck::cast_slice(&lights)
            );
        }
    }
}

/// A trait to add light related functions to a render pass.
pub trait SetupLights<'a, 'b> where 'b: 'a {
    /// Loads the lights from the given light engine into the current render pass for rendering.
    /// 
    /// Arguments:
    /// * engine: &LightEngine - The light engine that the lights will be loaded from.
    fn load_lights(&mut self, engine: &'b LightEngine);
}

/// An implementation of SetupLights for wgpu::RenderPass.
impl<'a, 'b> SetupLights<'a, 'b> for wgpu::RenderPass<'a> where 'b: 'a {
    fn load_lights(&mut self, engine: &'b LightEngine) {
        self.set_bind_group(2, engine.bind_group(), &[]);
    }
}
