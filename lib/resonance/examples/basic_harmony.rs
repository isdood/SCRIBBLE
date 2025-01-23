use resonance::{ResonanceCore, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let core = ResonanceCore::new();

    // Create a simple pattern
    let pattern = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    // Weave harmony
    let harmony = core.weave_harmony(&pattern).await?;

    println!("Harmony level achieved: {:.4}", harmony);

    Ok(())
}
