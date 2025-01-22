module QuantumVector

export QuantumState, create_quantum_vector, quantum_dot

struct QuantumState
    amplitudes::Vector{Complex{Float64}}
    coherence::Float64

    function QuantumState(amplitudes::Vector{Complex{Float64}}, coherence::Float64)
        coherence = clamp(coherence, 0.0, 1.0)
        new(normalize!(amplitudes), coherence)
    end
end

struct QuantumVector3D
    x::Float64
    y::Float64
    z::Float64
    state::QuantumState
end

function normalize!(v::Vector{Complex{Float64}})
    norm = sqrt(sum(abs2.(v)))
    if norm > 0
        v ./= norm
    end
    return v
end

function create_quantum_vector(x::Float64, y::Float64, z::Float64,
                             state::QuantumState=QuantumState([Complex{Float64}(1.0, 0.0)], 1.0))
    QuantumVector3D(x, y, z, state)
end

function quantum_dot(a::QuantumVector3D, b::QuantumVector3D)
    classical_dot = a.x * b.x + a.y * b.y + a.z * b.z
    quantum_factor = real(sum(conj.(a.state.amplitudes) .* b.state.amplitudes))
    coherence = min(a.state.coherence, b.state.coherence)
    return classical_dot * (coherence * quantum_factor + (1 - coherence))
end

end # module
