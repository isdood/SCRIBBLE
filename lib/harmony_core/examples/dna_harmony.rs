use harmony_core::{HarmonyCore, HarmonyError};

fn main() -> Result<(), HarmonyError> {
    // Initialize harmony system
    let mut core = HarmonyCore::new();
    
    // Create DNA-like pattern (ATCG)
    let pattern = vec![65, 84, 67, 71];
    
    // Process through crystal lattice
    let result = core.weave_pattern(&pattern)?;
    
    println!("Harmony achieved!");
    println!("Resonance: {:.2} Hz", result.frequency);
    println!("Coherence: {:.2}%", result.coherence * 100.0);
    
    Ok(())
}
