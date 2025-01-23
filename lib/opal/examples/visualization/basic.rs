use opal::vis_engine::VisEngine;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("OPAL Visualizer")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut vis_engine = VisEngine::new(window).await;

    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == vis_engine.window().id() => {
                match event {
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::Resized(physical_size) => {
                        vis_engine.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        match vis_engine.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => vis_engine.resize(vis_engine.window().inner_size()),
                            Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                            Err(e) => eprintln!("Render error: {}", e),
                        }
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                vis_engine.window().request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}

fn main() {
    pollster::block_on(run());
}
