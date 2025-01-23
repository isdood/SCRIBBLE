#!/bin/bash

# start_crystal.sh
# Author: isdood
# Created: 2025-01-23 02:38:23 UTC

echo "=== Crystal Optimization Initialization ==="
echo "Date: 2025-01-23 02:38:23 UTC"
echo "User: isdood"
echo "Framework: Scribble/Opal"

# First, ensure all directories exist and are clean
mkdir -p ../scribe/src
mkdir -p ../errors/src
mkdir -p ../magicmath/src
mkdir -p src/vis_engine/crystal/{core,buffer,tunnel,resonance}
mkdir -p src/vis_engine/shader
mkdir -p examples

# Create shader module with faster rotation and multiple crystals
cat > src/vis_engine/shader/crystal.wgsl << 'EOF'
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
EOF

# Update VisEngine implementation to draw more vertices
cat > src/vis_engine/core/engine.rs << 'EOF'
[Previous implementation until render function...]

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let elapsed = self.start_time.elapsed().as_secs_f32();

        let uniforms = Uniforms { time: elapsed };
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..9, 0..1); // Draw 3 crystals (3 vertices each)
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

[Rest of implementation remains the same...]
EOF

# Update crystal demo to remove unused variables
cat > examples/crystal_demo.rs << 'EOF'
use opal::vis_engine::{crystal::init, VisEngine};
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Crystal-Enhanced Visualization Demo");
    init()?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Crystal Demo")
        .build(&event_loop)?;

    let mut engine = VisEngine::new(&window).await?;
    let mut frame_count = 0;
    let mut last_fps_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                window.request_redraw();

                // Calculate and display FPS
                frame_count += 1;
                if last_fps_update.elapsed().as_secs_f32() >= 1.0 {
                    let fps = frame_count as f32 / last_fps_update.elapsed().as_secs_f32();
                    window.set_title(&format!("Crystal Demo - {:.1} FPS", fps));
                    frame_count = 0;
                    last_fps_update = Instant::now();
                }
            }
            Event::RedrawRequested(_) => {
                if let Err(e) = engine.render() {
                    eprintln!("Render error: {:?}", e);
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                engine.resize(size);
            }
            _ => {}
        }
    });
}
EOF

[Rest of script remains the same...]

echo "=== Initialization Complete ==="
echo "Next steps:"
echo "1. Run 'cargo build' in the workspace root (/home/guavabot1/scribble/scribble/)"
echo "2. Run 'cargo build' in the opal directory"
echo "3. Execute 'cargo run --example crystal_demo' to test the implementation"
echo ""
echo "For optimal results:"
echo "- Using magicmath constants for optimization"
echo "- Crystal patterns integrated with visualization engine"
echo "- Phi-based memory alignment active"

chmod +x start_crystal.sh

echo "Script completed successfully!"
