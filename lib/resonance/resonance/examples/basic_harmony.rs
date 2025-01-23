use resonance::ResonanceCore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut core = ResonanceCore::new();

    // Create a harmony pattern
    let pattern = HarmonyPattern::new()
        .with_crystal_structure([3, 4, 5])
        .with_whimsy_factor(0.618)
        .build()?;

    // Weave harmony through the crystal lattice
    let resonance = core.weave_harmony(pattern).await?;

    println!("Harmony Level: {}", resonance.harmony_level());
    println!("Whimsy Factor: {}", resonance.whimsy_factor());
    println!("Crystal Coherence: {}", resonance.crystal_coherence());

    Ok(())
}
