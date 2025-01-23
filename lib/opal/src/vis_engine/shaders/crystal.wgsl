struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) quantum_state: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Enhanced quantum wave pattern
    let time = in.quantum_state;
    let wave = sin(time * 3.14159);
    let rotation = mat2x2<f32>(
        cos(time), -sin(time),
        sin(time), cos(time)
    );

    // Apply rotation and scaling
    let pos2d = rotation * vec2<f32>(in.position.x, in.position.y);
    let scale = 1.0 + 0.3 * wave;
    let final_pos = vec3<f32>(pos2d.x * scale, pos2d.y * scale, in.position.z);

    out.clip_position = vec4<f32>(final_pos, 1.0);

    // Color pulsing effect
    let color_pulse = vec3<f32>(
        in.color.r * (0.5 + 0.5 * sin(time * 2.0)),
        in.color.g * (0.5 + 0.5 * cos(time * 1.5)),
        in.color.b * (0.5 + 0.5 * sin(time * 1.7))
    );
    out.color = color_pulse;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
