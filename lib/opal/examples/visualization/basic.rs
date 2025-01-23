use opal::vis_engine::VisEngine;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("OPAL Visualizer")
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
                            Err(e) => eprintln!("Render error: {}", e),
                        }
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                vis_engine.window().request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}

fn main() {
    pollster::block_on(run());
}
