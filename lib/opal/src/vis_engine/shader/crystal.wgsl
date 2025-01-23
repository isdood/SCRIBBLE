struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Uniforms {
    time: f32,
    resolution: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

fn crystal_pattern(pos: vec2<f32>, time: f32) -> vec4<f32> {
    let phi = 1.618033988749895;
    let center = vec2<f32>(0.0, 0.0);
    let dist = length(pos - center);
    let angle = atan2(pos.y - center.y, pos.x - center.x);

    let spiral = sin(dist * 10.0 - time * 2.0 + angle * phi) * 0.5 + 0.5;
    let wave = sin(dist * 5.0 - time + angle * 3.0) * 0.5 + 0.5;
    let ripple = cos(dist * 15.0 - time * 3.0) * 0.5 + 0.5;

    return vec4<f32>(
        spiral * wave,
        wave * ripple,
        ripple * spiral,
        1.0
    );
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    let base_angle = uniforms.time * 3.0;
    let crystal_index = vertex_index / 3u;
    let vertex_in_crystal = vertex_index % 3u;

    let angle = base_angle + f32(crystal_index) * 0.5;
    let scale = 0.3;

    let rotation = mat2x2<f32>(
        cos(angle), -sin(angle),
        sin(angle), cos(angle)
    );

    var pos = vec2<f32>(0.0, 0.0);

    switch(vertex_in_crystal) {
        case 0u: {
            pos = vec2<f32>(-0.866 * scale, -0.5 * scale);
        }
        case 1u: {
            pos = vec2<f32>(0.866 * scale, -0.5 * scale);
        }
        default: {
            pos = vec2<f32>(0.0, 1.0 * scale);
        }
    }

    // Adjust crystal positions to form a regular pattern
    let angle_offset = 6.283185307179586 * f32(crystal_index) / 3.0; // 2π/3 spacing
    let crystal_offset = vec2<f32>(
        cos(angle_offset) * 0.6,
        sin(angle_offset) * 0.6
    );

    let rotated = rotation * pos;
    let final_pos = rotated + crystal_offset;
    output.position = vec4<f32>(final_pos, 0.0, 1.0);
    output.color = crystal_pattern(final_pos, uniforms.time);
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
