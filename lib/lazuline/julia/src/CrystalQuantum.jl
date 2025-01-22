module CrystalQuantum

using LinearAlgebra
using Distributed
using CUDA

# FFI exports
const crystal_lib = joinpath(@__DIR__, "../../zig-out/lib/libcrystal_julia")

struct QuantumState
    ptr::Ptr{Cvoid}
end

function QuantumState()
    ptr = ccall((:julia_quantum_init, crystal_lib), Ptr{Cvoid}, ())
    QuantumState(ptr)
end

function process_quantum_state(state::QuantumState)
    ccall((:julia_quantum_process, crystal_lib), Cvoid, (Ptr{Cvoid},), state.ptr)
end

export QuantumState, process_quantum_state

end
