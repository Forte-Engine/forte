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
var diffuse_color: vec4<f32>;
@group(1) @binding(11)
var emissive_color: vec4<f32>;
@group(1) @binding(12)
var metadata: vec4<f32>; // FORMAT: metallic_factor, roughness_factor, alpha_mode, alpha_cutoff

fn cur_diffuse_color(tex_coords: vec2<f32>) -> vec4<f32> {
    return textureSample(diffuse_texture, diffuse_sampler, tex_coords) * diffuse_color;
}
