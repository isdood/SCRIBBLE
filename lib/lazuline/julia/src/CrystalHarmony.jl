module CrystalHarmony

using LinearAlgebra
using Distributed

const crystal_lib = joinpath(@__DIR__, "../../zig-out/lib/libcrystal_julia")

struct HarmonyState
    ptr::Ptr{Cvoid}
end

function HarmonyState()
    ptr = ccall((:julia_harmony_init, crystal_lib), Ptr{Cvoid}, ())
    HarmonyState(ptr)
end

function process_harmony_state(state::HarmonyState)
    ccall((:julia_harmony_process, crystal_lib), Cvoid, (Ptr{Cvoid},), state.ptr)
end

export HarmonyState, process_harmony_state

end
