using Test

# Use absolute paths from project root
const PROJECT_ROOT = dirname(dirname(dirname(@__FILE__)))
include(joinpath(PROJECT_ROOT, "src", "runtime", "SparkRuntime.jl"))
include(joinpath(PROJECT_ROOT, "src", "runtime", "TestHelper.jl"))

using .SparkRuntime
using .TestHelper

@spark_test begin
    @testset "Path handling" begin
        @test validate_path("std**math")
        @test validate_path("std**math**add")
        @test !validate_path("std/math")
        @test !validate_path("std\\math")

        path = SparkPath("std**math**add")
        @test length(path.components) == 3
        @test path.components == ["std", "math", "add"]

        normalized = normalize_path(path)
        @static if Sys.iswindows()
            @test normalized == "std\\math\\add"
        else
            @test normalized == "std/math/add"
        end
    end

    @testset "Module registry" begin
        path = SparkPath("std**math")
        mod = SparkModule(name="math", path=path)
        @test mod.name == "math"
        @test mod.loaded == false
        @test isempty(mod.exports)
    end
end

println("✨ All tests completed successfully!")
