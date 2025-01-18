// lib/unstable_matter/examples/mesh_clock_demo.rs
use unstable_matter::{
    mesh_clock::MeshClock,
    Vector3D,
};

fn main() {
    println!("MeshClock Quantum State Demo");
    println!("Timestamp: 2025-01-18 12:26:28 UTC");  // Updated timestamp
    println!("Current User: isdood\n");

    // Initialize mesh with origin point and quantum distance
    let origin = Vector3D::new(0.0, 0.0, 0.0);
    let quantum_distance = 1.0;
    let mut mesh = MeshClock::new(origin, quantum_distance);

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
    println!("  - Entanglement strength: {:.2}\n", mesh.get_entanglement_strength());

    // Test entanglement
    println!("Testing quantum entanglement...");
    if let Ok(()) = mesh.entangle_cells() {
        println!("Cells entangled successfully");
        println!("  - Quantum state: {:?}", mesh.get_quantum_state());
        println!("  - Entanglement strength: {:.2}", mesh.get_entanglement_strength());
    }

    // Test superposition
    println!("\nTesting quantum superposition...");
    if let Ok(()) = mesh.create_superposition() {
        println!("Superposition created successfully");
        println!("  - Quantum state: {:?}", mesh.get_quantum_state());
        println!("  - Pattern coherence: {:.2}",
                 mesh.get_pattern_coherence().unwrap_or(0.0));
    }

    // Test quantum pattern transfer
    println!("\nTesting quantum pattern transfer...");
    if let Ok(()) = mesh.transfer_quantum_pattern() {
        println!("Pattern transferred successfully");
        println!("  - Quantum state: {:?}", mesh.get_quantum_state());
        match mesh.get_pattern_coherence() {
            Ok(coherence) => println!("  - Pattern coherence: {:.2}", coherence),
            Err(e) => println!("  - Pattern coherence: {}", e),
        }
    }

    // Test pattern replication
    println!("\nTesting pattern replication...");
    match mesh.replicate_pattern() {
        Ok(_) => println!("Pattern replicated successfully"),
        Err(e) => println!("Pattern replication failed: {}", e),
    }

    // Perform multiple pings to generate frequency data
    println!("\nPerforming quantum measurements...");
    for _ in 0..10 {
        if let Ok(time) = mesh.ping() {
            println!("  - Ping successful - propagation time: {} ns", time);
        }
    }

    println!("\nFinal quantum state:");
    match mesh.get_pattern_coherence() {
        Ok(coherence) => println!("  - Pattern coherence: {:.2}", coherence),
        Err(e) => println!("  - Pattern coherence: {}", e),
    }
    println!("  - Quantum state: {:?}", mesh.get_quantum_state());
    println!("  - Entanglement strength: {:.2}", mesh.get_entanglement_strength());

    // Get frequency measurements
    match mesh.get_frequency() {
        Ok(freq) => println!("  - Frequency: {:.2} Hz", freq),
        Err(e) => println!("  - Frequency: {}", e),
    }
}
