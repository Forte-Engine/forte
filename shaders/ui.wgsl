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
    @location(10) cornerRounds: vec4<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) cornerRounds: vec4<f32>
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
    out.cornerRounds = instance.cornerRounds;
    return out;
}

fn checkRoundDiscard(coords: vec2<f32>, dimensions: vec2<f32>, radi: vec4<f32>) -> bool
{
    var radius = radi.y;
    var circle_center = vec2<f32>(radius);

    if(length(coords - circle_center) > radius && coords.x < circle_center.x && coords.y < circle_center.y) { return true; } //first circle
    
    radius = radi.x;
    circle_center = vec2<f32>(dimensions.x - radius, radius);
    
    if(length(coords - circle_center) > radius && coords.x > circle_center.x && coords.y < circle_center.y) { return true; } //second circle
    
    radius = radi.z;
    circle_center = dimensions - vec2<f32>(radius);

    if(length(coords - circle_center) > radius && coords.x > circle_center.x && coords.y > circle_center.y) { return true; } //third circle
    
    radius = radi.w;
    circle_center = vec2<f32>(radius, dimensions.y - radius);
    
    if(length(coords - circle_center) > radius && coords.x < circle_center.x && coords.y > circle_center.y) { return true; } //fourth circle
    
    return false;
        
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (checkRoundDiscard(in.tex_coords, vec2<f32>(1.0, 1.0), in.cornerRounds)) { return vec4<f32>(0.0, 0.0, 0.0, 0.0); }
    return vec4<f32>(in.color);
}
