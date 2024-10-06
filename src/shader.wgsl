struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) texture_position: vec2<f32>,
    @location(2) id: u32,
};

struct Object {
    color: vec4<f32>,
    texture: u32,
};

@group(0) @binding(0) var<storage, read> storage_data: array<Object>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let object = storage_data[model.id];

    var out: VertexOutput;

    out.color = object.color;
    out.clip_position = vec4<f32>(model.position, 1.0, 1.0);

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
 
