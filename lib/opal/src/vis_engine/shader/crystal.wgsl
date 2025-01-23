struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Uniforms {
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    let base_angle = uniforms.time * 3.0; // Speed up rotation
    let crystal_index = vertex_index / 3u;
    let vertex_in_crystal = vertex_index % 3u;

    // Different rotation speeds for each crystal
    let angle = base_angle + f32(crystal_index) * 0.5;
    let scale = 0.3; // Make crystals smaller to fit more

    let rotation = mat2x2<f32>(
        cos(angle), -sin(angle),
        sin(angle), cos(angle)
    );

    // Base positions for an equilateral triangle
    var pos = vec2<f32>(0.0, 0.0);
    var color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    switch(vertex_in_crystal) {
        case 0u: {
            pos = vec2<f32>(-0.866 * scale, -0.5 * scale);
            color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
        case 1u: {
            pos = vec2<f32>(0.866 * scale, -0.5 * scale);
            color = vec4<f32>(0.0, 1.0, 0.0, 1.0);
        }
        default: {
            pos = vec2<f32>(0.0, 1.0 * scale);
            color = vec4<f32>(0.0, 0.0, 1.0, 1.0);
        }
    }

    // Offset each crystal
    let crystal_offset = vec2<f32>(
        cos(f32(crystal_index) * 2.094) * 0.6, // 2.094 radians = 120 degrees
        sin(f32(crystal_index) * 2.094) * 0.6
    );

    let rotated = rotation * pos;
    output.position = vec4<f32>(rotated + crystal_offset, 0.0, 1.0);
    output.color = color;
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
