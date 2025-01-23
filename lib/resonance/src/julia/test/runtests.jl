using Resonance
using Test

@testset "Resonance.jl" begin
    # Create a small test lattice
    lattice = rand(4, 4, 4)
    harmony_field = complex.(rand(4, 4, 4))
    crystal = CrystalStructure(lattice, harmony_field, 0.618)
    
    result = compute_harmony(crystal)
    
    @test size(result.crystal_pattern) == (4, 4, 4)
    @test 0.0 ≤ result.harmony_level ≤ 1.0
    @test 0.0 ≤ result.whimsy_factor ≤ π
end
