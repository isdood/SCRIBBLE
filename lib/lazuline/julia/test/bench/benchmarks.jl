using BenchmarkTools
using Lazuline

# Define benchmark suite
const SUITE = BenchmarkGroup()

# Quantum operations benchmark
SUITE["quantum"] = BenchmarkGroup(["quantum", "parallel"])
SUITE["quantum"]["state_evolution"] = @benchmarkable begin
    state = QuantumState(1000)
    evolve!(state)
end

# Parallel processing benchmark
SUITE["parallel"] = BenchmarkGroup(["compute", "parallel"])
SUITE["parallel"]["matrix_mul"] = @benchmarkable begin
    A = rand(1000, 1000)
    B = rand(1000, 1000)
    parallel_matmul(A, B)
end

# Run benchmarks
results = run(SUITE, verbose = true)
