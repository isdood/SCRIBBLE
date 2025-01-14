// lib/unstable_matter/examples/mesh_clock_demo.rs
/// Last Updated: 2025-01-14 05:19:56 UTC
/// Author: isdood
/// Current User: isdood

use unstable_matter::{
    mesh_clock::MeshClock,
    Vector3D,
};

fn main() {
    println!("MeshClock Quantum State Demo");
    println!("Timestamp: 2025-01-14 05:19:56 UTC");
    println!("Current User: isdood\n");

    // Initialize mesh with origin point and quantum distance
    let origin = Vector3D::new(0.0, 0.0, 0.0);
    let quantum_distance = 1.0;
    let mut mesh = MeshClock::new(origin, quantum_distance);

    println!("Initializing quantum mesh at origin: {:?}", origin);

    // Create some test positions
    let test_positions = [
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (1.0, 1.0, 0.0),
    ];

    println!("\nTesting quantum positions...");
    for (x, y, z) in test_positions.iter() {
        let position = Vector3D::new(*x, *y, *z);

        println!("\nQuantum state at position {:?}:", position);
        match mesh.get_pattern_coherence() {
            Ok(coherence) => println!("  - Pattern coherence: {:.2}", coherence),
            Err(e) => println!("  - Coherence error: {}", e),
        }

        println!("  - Quantum state: {:?}", mesh.get_quantum_state());
    }

    println!("\nMesh statistics:");
    match mesh.get_pattern_coherence() {
        Ok(coherence) => println!("  - Pattern coherence: {:.2}", coherence),
        Err(e) => println!("  - Coherence error: {}", e),
    }
    println!("  - Quantum state: {:?}", mesh.get_quantum_state());
}
