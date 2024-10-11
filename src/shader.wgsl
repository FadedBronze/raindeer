struct VertexInput {
    @location(0) color: u32,
    @location(1) position: vec2<f32>,
    @location(2) texture_position: vec2<f32>,
    @location(3) id: u32,
};

struct Object {
    transform: mat4x4<f32>,
    texture: u32,
};

@group(0) @binding(0) var<storage, read> storage_data: array<Object>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

fn unpack_u8_to_f32(packed: u32) -> vec4<f32> {
    let r: u32 = (packed >> 0) & 0xFFu;
    let g: u32 = (packed >> 8) & 0xFFu;
    let b: u32 = (packed >> 16) & 0xFFu;
    let a: u32 = (packed >> 24) & 0xFFu;

    return vec4<f32>(f32(r) / 255.0, f32(g) / 255.0, f32(b) / 255.0, f32(a) / 255.0);
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let object = storage_data[model.id];

    var out: VertexOutput;

    out.color = unpack_u8_to_f32(model.color);
    out.clip_position = object.transform * vec4<f32>(model.position, 1.0, 1.0);

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
 
