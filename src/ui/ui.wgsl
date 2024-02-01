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
    @location(9) color: vec4<f32>,
    @location(10) border_color: vec4<f32>,
    @location(11) extra: vec4<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) border_color: vec4<f32>,
    @location(3) round: f32,
    @location(4) border: f32,
    @location(5) draw_texture: f32
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    // create model matrix
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    // create final output
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = model_matrix * vec4<f32>(model.position, 1.0);
    out.color = instance.color;
    out.border_color = instance.border_color;
    out.round = instance.extra.x;
    out.border = instance.extra.y;
    out.draw_texture = instance.extra.z;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

fn dist_to_edge(coords: vec2<f32>, dimensions: vec2<f32>, max_dist: f32) -> f32 {
    let dist_from_max = min(
        min(
            min(coords.y / max_dist, 1.0), 
            min(coords.x / max_dist, 1.0)
        ), 
        min(
            min((1.0 - coords.y) / max_dist, 1.0), 
            min((1.0 - coords.x) / max_dist, 1.0)
        )
    );
    
    var circle_center = vec2<f32>(max_dist);
    if (coords.x < circle_center.x && coords.y < circle_center.y) { return -(distance(coords, circle_center) / max_dist) + 1.0; }
    
    circle_center = vec2<f32>(dimensions.x - max_dist, max_dist);
    if (coords.x > circle_center.x && coords.y < circle_center.y) { return -(distance(coords, circle_center) / max_dist) + 1.0; }
    
    circle_center = dimensions - vec2<f32>(max_dist);
    if (coords.x > circle_center.x && coords.y > circle_center.y) { return -(distance(coords, circle_center) / max_dist) + 1.0; }
    
    circle_center = vec2<f32>(max_dist, dimensions.y - max_dist);
    if (coords.x < circle_center.x && coords.y > circle_center.y) { return -(distance(coords, circle_center) / max_dist) + 1.0; }
    
    return dist_from_max;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = in.color;
    color *= textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let max_dist = max(in.border, in.round);
    let border_ratio = min(in.border / max_dist, 0.99);
    let dist = dist_to_edge(in.tex_coords, vec2<f32>(1.0, 1.0), max_dist);
    if (dist <= 0.1) { return in.border_color * smoothstep(0.0, 0.1, dist); }
    else if (dist <= border_ratio - 0.1) { return in.border_color; }
    else if (dist <= border_ratio) { 
        let step = smoothstep(border_ratio - 0.1, border_ratio, dist);
        return (color * step) + (in.border_color * (1.0 - step));
    } else { return vec4<f32>(color); }
}
