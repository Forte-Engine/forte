# Forte Engine

A basic WGPU render engine designed to have a very small amount of overhead.
In the future, this will serve as a base for other Forte Engine related modules.

## Modules
### Math
Extensions for `cgmath` to make our lives easier.  For example, `Quaternion::euler_deg_x(angle: f32)` allows us to quickly create quaternions with a angle around the x axis.

### Render
The basic render engine.  This handles everything from creating the app and window, to handling inputs, to rendering mesh.  See below for examples.

### Lights
This module handles the basic information needed for lights.  See examples below.

## Shader Information
### Basic
#### Group 0: Camera Information
This contains the information of the camera.  

Struct arguments:
 - view_pos: The position in 3d space of the camera.  The fourth W value can be set to anything needed for the shader.
 - view_proj: The combined view and projection matrix for rendering.

```wgsl
struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;
```

#### Group 1: Texture
The texture to render with.

Binding 0: The texture itself.
Binding 1: The sampler that will be used to "sample" the texture according to WGPU specs.

```wgsl
@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;
```

### Options: Lights
#### Group 2: Lights
This is the basic information required for lights in a scene.  You may wish to tweak what lights are passed to the shader based on their distances to the player.

Struct arguments:
- position: the position the light in 3d space.
- range: the range of the light in units.
- color: the color of the light.
- exponent: the exponent used to "soften" the edges of the lights.
- direction: the direction the light is pointing, this only needs to be set if cutoff is set.
- cutoff: the dot product where the light will "cutoff", this is useful for spotlights where the light will not emit in 360 degrees.

```wgsl
struct Light {
    position: vec3<f32>,
    range: f32,
    color: vec3<f32>,
    exponent: f32,
    direction: vec3<f32>, 
    cutoff: f32
}
@group(2) @binding(0)
var<storage, read_write> lights: array<Light>;
@group(2) @binding(1)
var<uniform> num_lights: u32;
@group(2) @binding(2)
var<uniform> ambient_light: vec3<f32>;
```

## Examples
