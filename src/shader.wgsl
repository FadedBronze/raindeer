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

fn extract_u8_from_u32(value: u32) -> vec4<f32> {
    let r: f32 = f32(value & 0xFFu) / 255.0;           // Extract red channel
    let g: f32 = f32((value >> 8) & 0xFFu) / 255.0;    // Extract green channel
    let b: f32 = f32((value >> 16) & 0xFFu) / 255.0;   // Extract blue channel
    let a: f32 = f32((value >> 24) & 0xFFu) / 255.0;   // Extract alpha channel
    return vec4<f32>(r, g, b, a);                      // Return color as vec4<f32>
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let object = storage_data[model.id];

    var out: VertexOutput;

    out.color = extract_u8_from_u32(model.color);
    out.clip_position = object.transform * vec4<f32>(model.position, 1.0, 1.0);

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
 
