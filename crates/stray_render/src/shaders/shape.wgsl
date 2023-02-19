struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct Transform {
    @location(3) matrix_0: vec4<f32>,
    @location(4) matrix_1: vec4<f32>,
    @location(5) matrix_2: vec4<f32>,
    @location(6) matrix_3: vec4<f32>,
}
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput, transform: Transform) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        transform.matrix_0,
        transform.matrix_1,
        transform.matrix_2,
        transform.matrix_3
    );
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = model_matrix * vec4<f32>(model.position, 1.0) ;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}