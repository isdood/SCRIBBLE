using Test
using SparkJL

@testset "SparkJL Tests" begin
    @testset "WeavePattern" begin
        @test_throws ArgumentError WeavePattern(0)
        @test_throws ArgumentError WeavePattern(1001)
        pattern = WeavePattern(500)
        @test pattern.factor == 500
        @test 0.0 ≤ pattern.zx_ratio ≤ 1.0
        @test 0.0 ≤ pattern.qw_ratio ≤ 1.0
        @test pattern.zx_ratio + pattern.qw_ratio ≈ 1.0
    end

    @testset "Wave" begin
        data = randn(100)
        wave = Wave(data)
        @test length(wave.data) == 100
        @test !wave.is_optimized
        @test !wave.simd_enabled
    end

    @testset "Crystal" begin
        # Test tuple constructor
        crystal1 = Crystal((32, 32, 32), 1.0)
        @test crystal1.dimensions == (32, 32, 32)
        @test crystal1.spacing == 1.0

        # Test individual dimensions constructor
        crystal2 = Crystal(16, 16, 16, 0.5)
        @test crystal2.dimensions == (16, 16, 16)
        @test crystal2.spacing == 0.5
    end
end
