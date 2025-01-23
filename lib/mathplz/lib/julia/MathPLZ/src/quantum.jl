struct QuantumState{T<:Complex}
    amplitude::T
    phase::Float64
    stability::Float64

    function QuantumState(amplitude::T) where T<:Complex
        new{T}(amplitude, angle(amplitude), 0.87)  # 0.87 stability threshold
    end
end

function quantum_stability(state::QuantumState)
    state.stability
end
