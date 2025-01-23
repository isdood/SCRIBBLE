#[cfg(test)]
mod tests {
    use winit::window::WindowBuilder;
    use winit::event_loop::EventLoop;

    #[tokio::test]
    async fn test_engine_creation() {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let engine = super::VisEngine::new(window).await;
        assert!(engine.window().inner_size().width > 0);
    }
}
