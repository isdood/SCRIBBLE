// lib/unstable_matter/examples/mesh_clock_demo.rs
/// Last Updated: 2025-01-14 05:16:42 UTC
/// Author: isdood
/// Current User: isdood

use unstable_matter::{
    mesh_clock::{MeshClock, MeshCell},
    Vector3D,
};

fn main() {
    println!("MeshClock Quantum State Demo");
    println!("Timestamp: 2025-01-14 05:16:42 UTC");
    println!("Current User: isdood\n");

    // Initialize mesh with origin point
    let origin = Vector3D::new(0, 0, 0);
    let mut mesh = MeshClock::new();

    println!("Initializing quantum mesh at origin: {:?}", origin);

    // Create some test cells
    let test_positions = [
        (0, 0, 0),
        (1, 0, 0),
        (0, 1, 0),
        (1, 1, 0),
    ];

    println!("\nCreating test cells...");
    for (x, y, z) in test_positions.iter() {
        let position = Vector3D::new(*x, *y, *z);
        let new_cell = mesh.create_cell(position);

        println!("\nCell at position {:?}:", position);
        println!("  - State: {:?}", new_cell.get_state());
        println!("  - Cell ID: {}", new_cell.get_id());

        // Update cell state
        mesh.update_cell(&new_cell);
    }

    println!("\nMesh statistics:");
    println!("  - Active cells: {}", mesh.get_active_cells());
    println!("  - Total updates: {}", mesh.get_update_count());
    println!("  - Quantum coherence: {:.2}", mesh.get_coherence());
}
