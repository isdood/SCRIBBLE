// lib/unstable_matter/examples/mesh_clock_demo.rs
/// Last Updated: 2025-01-14 05:23:44 UTC
/// Author: isdood
/// Current User: isdood

use unstable_matter::{
    mesh_clock::MeshClock,
    Vector3D,
};

fn main() {
    println!("MeshClock Quantum State Demo");
    println!("Timestamp: 2025-01-14 05:23:44 UTC");
    println!("Current User: isdood\n");

    // Initialize mesh with origin point and quantum distance
    let origin = Vector3D::new(0.0, 0.0, 0.0);
    let quantum_distance = 1.0;
    let mesh = MeshClock::new(origin, quantum_distance);

    println!("Initializing quantum mesh:");
    println!("  - Origin: {:?}", origin);
    println!("  - Quantum distance: {:.2}\n", quantum_distance);

    // Test initial state
    println!("Initial quantum state:");
    match mesh.get_pattern_coherence() {
        Ok(coherence) => println!("  - Pattern coherence: {:.2}", coherence),
        Err(e) => println!("  - Pattern coherence: {}", e),
    }
    println!("  - Quantum state: {:?}", mesh.get_quantum_state());

    // Test with some patterns
    println!("\nSetting quantum patterns...");
    let test_positions = [
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (1.0, 1.0, 0.0),
    ];

    for (x, y, z) in test_positions.iter() {
        let position = Vector3D::new(*x, *y, *z);
        if let Ok(()) = mesh.set_pattern(position) {
            println!("\nPattern at {:?}:", position);
            match mesh.get_pattern_coherence() {
                Ok(coherence) => println!("  - Pattern coherence: {:.2}", coherence),
                Err(e) => println!("  - Pattern coherence: {}", e),
            }
            println!("  - Quantum state: {:?}", mesh.get_quantum_state());
        }
    }

    println!("\nFinal quantum state:");
    match mesh.get_pattern_coherence() {
        Ok(coherence) => println!("  - Pattern coherence: {:.2}", coherence),
        Err(e) => println!("  - Pattern coherence: {}", e),
    }
    println!("  - Quantum state: {:?}", mesh.get_quantum_state());
}
