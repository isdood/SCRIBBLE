using Resonance

# Create a test crystal structure
lattice = rand(8, 8, 8)
harmony_field = complex.(rand(8, 8, 8))
crystal = CrystalStructure(lattice, harmony_field, 0.618)

# Compute harmony
result = compute_harmony(crystal)

println("Harmony level: ", result.harmony_level)
println("Whimsy factor: ", result.whimsy_factor)
