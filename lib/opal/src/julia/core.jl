module OpalCore

export ResonanceField, CrystalLattice, optimize!

using Base: @kwdef

@kwdef mutable struct ResonanceField
    intensity::Float64 = 1.0
    frequency::Float64 = 1.0
    phase::Float64 = 0.0
    counter::Int64 = 0
    max_intensity::Float64 = 10.0
    max_frequency::Float64 = 10.0
end

@kwdef mutable struct CrystalLattice
    spacing::Float64 = 1.0
    alignment::Float64 = 1.0
    orientation::Float64 = 0.0
    counter::Int64 = 0
    max_spacing::Float64 = 10.0
    max_alignment::Float64 = 10.0
end

function safe_update(value::Float64, factor::Float64, max_value::Float64)
    new_value = value * factor
    return min(max_value, new_value)
end

function optimize!(field::ResonanceField)
    field.counter += 1
    base_angle = (field.counter % 100) * π / 100.0

    # Use bounded factors for numerical stability
    i_factor = abs(sin(base_angle)) * 0.01 + 1.0
    f_factor = abs(cos(base_angle)) * 0.01 + 1.0

    field.intensity = safe_update(field.intensity, i_factor, field.max_intensity)
    field.frequency = safe_update(field.frequency, f_factor, field.max_frequency)
    field.phase = mod(field.phase + 0.01, 2π)
    nothing
end

function optimize!(lattice::CrystalLattice)
    lattice.counter += 1
    base_angle = (lattice.counter % 100) * π / 100.0

    # Use bounded factors for numerical stability
    s_factor = abs(sin(base_angle)) * 0.01 + 1.0
    a_factor = abs(cos(base_angle)) * 0.01 + 1.0

    lattice.spacing = safe_update(lattice.spacing, s_factor, lattice.max_spacing)
    lattice.alignment = safe_update(lattice.alignment, a_factor, lattice.max_alignment)
    lattice.orientation = mod(lattice.orientation + 0.01, 2π)
    nothing
end

end # module
