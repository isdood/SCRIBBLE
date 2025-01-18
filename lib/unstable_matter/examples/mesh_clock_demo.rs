// lib/unstable_matter/examples/mesh_clock_demo.rs

use unstable_matter::{
    mesh_clock::MeshClock,
    Vector3D,
};

fn main() {
    // Updated timestamp to match current system time
    println!("MeshClock Quantum State Demo");
    println!("Timestamp: 2025-01-18 12:37:41 UTC");
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
        println!("  - Entanglement strength: {:.2}\n", mesh.get_entanglement_strength());
    }

    // Test superposition
    println!("Testing quantum superposition...");
    if let Ok(()) = mesh.create_superposition() {
        println!("Superposition created successfully");
        println!("  - Quantum state: {:?}", mesh.get_quantum_state());
        println!("  - Pattern coherence: {:.2}\n",
                 mesh.get_pattern_coherence().unwrap_or(0.0));
    }

    // Test quantum pattern transfer
    println!("Testing quantum pattern transfer...");
    if let Ok(()) = mesh.transfer_quantum_pattern() {
        println!("Pattern transferred successfully");
        println!("  - Quantum state: {:?}", mesh.get_quantum_state());
        match mesh.get_pattern_coherence() {
            Ok(coherence) => println!("  - Pattern coherence: {:.2}\n", coherence),
            Err(e) => println!("  - Pattern coherence: {}\n", e),
        }
    }

    // Test pattern replication
    println!("Testing pattern replication...");
    match mesh.replicate_pattern() {
        Ok(_) => println!("Pattern replicated successfully\n"),
        Err(e) => println!("Pattern replication failed: {}\n", e),
    }

    // Perform multiple pings to generate frequency data
    println!("Performing quantum measurements...");
    let mut total_time = 0;
    let mut measurement_blocks = 0;

    for i in 1..=10 {
        match mesh.ping() {
            Ok(time) => {
                total_time += time;
                println!("  - Ping {} successful - propagation time: {} ns", i, time);

                if i % 5 == 0 {
                    measurement_blocks += 1;
                    match mesh.get_frequency() {
                        Ok(freq) => {
                            println!("    Block {} measurements:", measurement_blocks);
                            println!("    - Current frequency: {:.2} kHz", freq / 1000.0);
                            println!("    - Total measurement time: {:.3} µs", total_time as f64 / 1000.0);
                            println!("    - Average propagation time: {:.2} ns\n",
                                     total_time as f64 / i as f64);
                        },
                        Err(e) => println!("    Frequency measurement error: {}\n", e),
                    }
                }
            },
            Err(e) => println!("  - Ping {} failed: {}", i, e),
        }
    }

    // Display final quantum state with quantum decay effects
    println!("Final quantum state:");
    match mesh.get_pattern_coherence() {
        Ok(coherence) => println!("  - Pattern coherence: {:.3}", coherence),
        Err(e) => println!("  - Pattern coherence: {}", e),
    }
    println!("  - Quantum state: {:?}", mesh.get_quantum_state());
    println!("  - Entanglement strength: {:.3}", mesh.get_entanglement_strength());

    // Display final frequency measurements with enhanced precision
    match mesh.get_frequency() {
        Ok(freq) => {
            println!("\nFinal frequency measurements:");
            println!("  - Frequency: {:.3} kHz", freq / 1000.0);
            println!("  - Total measurement time: {:.3} µs", total_time as f64 / 1000.0);
            println!("  - Total oscillations: {}", 10);
            println!("  - Average propagation time: {:.3} ns", total_time as f64 / 10.0);
        },
        Err(e) => println!("\nFinal frequency measurement error: {}", e),
    }
}
