using Test
using Crystometer
using LinearAlgebra, Statistics

@testset "Crystometer.jl" begin
    @testset "CrystalMetrics" begin
        measurements = [1.0, 2.0, 3.0, 4.0, 5.0]
        lattice = [1.0 0.0; 0.0 1.0]
        pattern = Complex{Float64}[1.0 + 0.0im, 2.0 + 0.0im]

        metrics = CrystalMetrics(measurements, lattice, pattern)
        results = analyze_formation(metrics)

        @test results.precision > 0
        @test 0 ≤ results.alignment ≤ 1
        @test results.formation_strength > 0
    end
end
