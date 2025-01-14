use unstable_matter::mesh_clock::{MeshClock, CellState, QuantumState};
use unstable_matter::vector_space::FloatVector3D;

fn main() {
    println!("MeshClock Quantum Pattern Transfer Demo");
    println!("Time: 2025-01-13 06:48:01 UTC");
    println!("----------------------------------------\n");

    // Create a mesh clock with 1 light-microsecond spacing
    let origin = Vector3D::new(0.0, 0.0, 0.0);
    let mut clock = MeshClock::new(origin, 299.792458); // 1 microsecond light distance

    // Test 1: Basic entanglement
    println!("Test 1: Creating quantum entanglement...");
    match clock.entangle_cells() {
        Ok(_) => println!("✓ Cells successfully entangled"),
        Err(e) => println!("✗ Entanglement failed: {}", e),
    }
    println!("Entanglement strength: {:.6}", clock.get_entanglement_strength());

    // Test 2: Quantum pattern transfer
    println!("\nTest 2: Transferring quantum pattern...");
    match clock.transfer_quantum_pattern() {
        Ok(_) => println!("✓ Pattern successfully transferred"),
        Err(e) => println!("✗ Pattern transfer failed: {}", e),
    }

    // Test 3: Pattern replication
    println!("\nTest 3: Replicating pattern to new cell...");
    match clock.replicate_pattern() {
        Ok(new_cell) => {
            println!("✓ New cell created with pattern");
            println!("  - Cell state: {:?}", new_cell.state);
            println!("  - Position: {:?}", new_cell.position);
        },
        Err(e) => println!("✗ Pattern replication failed: {}", e),
    }

    // Test 4: Check pattern coherence
    println!("\nTest 4: Checking pattern coherence...");
    match clock.get_pattern_coherence() {
        Ok(coherence) => println!("✓ Pattern coherence: {:.6}", coherence),
        Err(e) => println!("✗ Coherence check failed: {}", e),
    }

    // Test 5: Quantum ping demonstration
    println!("\nTest 5: Performing quantum ping...");
    match clock.ping() {
        Ok(time) => println!("✓ Ping completed in {} ns", time),
        Err(e) => println!("✗ Ping failed: {}", e),
    }

    // Test 6: Time dilation calculation
    println!("\nTest 6: Calculating time dilation...");
    let dilation = clock.calculate_time_dilation();
    println!("✓ Time dilation factor: {:.12}", dilation);
}
