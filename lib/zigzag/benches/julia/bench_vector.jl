using BenchmarkTools
include("../../src/julia/quantum/quantum_vector.jl")
using .QuantumVector

function run_benchmarks()
    # Setup vectors
    v1 = create_quantum_vector(1.0, 2.0, 3.0)
    v2 = create_quantum_vector(4.0, 5.0, 6.0)

    println("ZigZag Julia Benchmarks")
    println("======================")

    # Benchmark dot product
    dot_bench = @benchmark quantum_dot($v1, $v2)
    println("\nDot Product:")
    display(dot_bench)

    # Benchmark with different coherence values
    println("\nQuantum Dot Product with varying coherence:")
    coherence_results = Dict{Float64, BenchmarkTools.Trial}()

    for coh in [0.0, 0.25, 0.5, 0.75, 1.0]
        println("\nCoherence = $coh:")
        state = QuantumState([Complex{Float64}(1.0, 0.0)], coh)
        v_quantum = create_quantum_vector(1.0, 2.0, 3.0, state)
        bench = @benchmark quantum_dot($v_quantum, $v2)
        display(bench)
        coherence_results[coh] = bench
    end
end

run_benchmarks()
