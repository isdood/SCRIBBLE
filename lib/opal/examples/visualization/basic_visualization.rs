use opal::vis_engine::VisEngine;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = VisEngine::new()?;

    // Main visualization loop
    while engine.get_fps() < 60.0 {
        engine.update(1.0 / 60.0)?;
        std::thread::sleep(Duration::from_millis(16)); // Cap at ~60 FPS
    }

    Ok(())
}
