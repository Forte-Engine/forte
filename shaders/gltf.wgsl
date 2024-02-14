#import ./light.wgsl as Lights

struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>
}

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) normal_matrix_0: vec3<f32>,
    @location(10) normal_matrix_1: vec3<f32>,
    @location(11) normal_matrix_2: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    // the models position, rotation, scale
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    // used to rotate the normals
    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.world_normal = normal_matrix * model.normal;
    var world_position = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;
    return out;
}

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(1) @binding(1)
var diffuse_sampler: sampler;
@group(1) @binding(2)
var roughness_texture: texture_2d<f32>;
@group(1) @binding(3)
var roughness_sampler: sampler;
@group(1) @binding(4)
var emissive_texture: texture_2d<f32>;
@group(1) @binding(5)
var emissive_sampler: sampler;
@group(1) @binding(6)
var normal_texture: texture_2d<f32>;
@group(1) @binding(7)
var normal_sampler: sampler;
@group(1) @binding(8)
var occlusion_texture: texture_2d<f32>;
@group(1) @binding(9)
var occlusion_sampler: sampler;
@group(1) @binding(10)
var<uniform> diffuse_color: vec4<f32>;
@group(1) @binding(11)
var<uniform> emissive_color: vec4<f32>;
@group(1) @binding(12)
var<uniform> metadata: vec4<f32>; // FORMAT: metallic_factor, roughness_factor, alpha_mode, alpha_cutoff

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // let diffuse = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // let color = diffuse.xyz * Lights::calculate_lights(camera.view_pos.xyz, in.world_position, in.world_normal);
    let lights = vec4<f32>(Lights::calculate_lights(camera.view_pos.xyz, in.world_position, in.world_normal), 1.0);
    return diffuse_color * textureSample(diffuse_texture, diffuse_sampler, in.tex_coords) * lights;
}
