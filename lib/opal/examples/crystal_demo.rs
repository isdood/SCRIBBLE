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
