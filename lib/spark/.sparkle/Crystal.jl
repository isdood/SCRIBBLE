# Crystal manipulation functions
function crystal(dims=(32,32,32), spacing=1.0)
    data = zeros(dims...)
    center = dims .÷ 2
    for i in 1:dims[1], j in 1:dims[2], k in 1:dims[3]
        r = sqrt(((i-center[1])/dims[1])^2 + ((j-center[2])/dims[2])^2 + ((k-center[3])/dims[3])^2)
        data[i,j,k] = exp(-r^2 * 5)
    end

    GLOBAL_STATE.current_crystal = Crystal(dims, spacing, data)
    println("Created crystal structure with dimensions $(dims) and spacing $(spacing)")
    visualize()
    return GLOBAL_STATE.current_crystal
end

function wave(n=100)
    x = range(0, 4π, length=n)
    data = sin.(x) .+ 0.5 .* cos.(2x) .+ 0.2 .* randn(n)
    GLOBAL_STATE.current_wave = Wave(data, 1.0)
    println("Created wave pattern with $(n) points")
    visualize()
    return GLOBAL_STATE.current_wave
end

function weave(pattern="default")
    if isnothing(GLOBAL_STATE.current_wave)
        println("Error: No wave pattern to weave. Create one first with 'wave'")
        return nothing
    end
    if !haskey(patterns, pattern)
        println("Error: Pattern '$(pattern)' not found")
        return nothing
    end
    println("Applied $(pattern) weave pattern to wave")
    result = patterns[pattern].transform(GLOBAL_STATE.current_wave)
    GLOBAL_STATE.current_wave = result
    println("Pattern applied successfully")
    visualize()
    return result
end

function visualize()
    if !isnothing(GLOBAL_STATE.current_crystal)
        crystal = GLOBAL_STATE.current_crystal
        middle_slice = crystal.data[:,:,crystal.dimensions[3]÷2]
        println("\nCrystal Visualization (middle slice):")
        display(heatmap(middle_slice, colormap=:viridis))
    end

    if !isnothing(GLOBAL_STATE.current_wave)
        wave = GLOBAL_STATE.current_wave
        n = length(wave.data)
        println("\nWave Visualization:")
        display(lineplot(1:n, wave.data, title="Wave Pattern", name="amplitude"))
    end
end

function optimize()
    if isnothing(GLOBAL_STATE.current_crystal) && isnothing(GLOBAL_STATE.current_wave)
        println("Error: Nothing to optimize. Create a crystal or wave first")
        return nothing
    end
    println("\nOptimizing current structure...")

    if !isnothing(GLOBAL_STATE.current_crystal)
        println("• Crystal optimization:")
        println("  - Dimensions: $(GLOBAL_STATE.current_crystal.dimensions)")
        println("  - Spacing: $(GLOBAL_STATE.current_crystal.spacing)")
        println("  - Mean density: $(mean(GLOBAL_STATE.current_crystal.data))")
        println("  ✓ Crystal optimization complete")
    end

    if !isnothing(GLOBAL_STATE.current_wave)
        println("• Wave optimization:")
        println("  - Points: $(length(GLOBAL_STATE.current_wave.data))")
        println("  - Frequency: $(GLOBAL_STATE.current_wave.frequency)")
        println("  - Amplitude range: [$(minimum(GLOBAL_STATE.current_wave.data)), $(maximum(GLOBAL_STATE.current_wave.data))]")
        println("  ✓ Wave optimization complete")
    end

    visualize()
    return (crystal=GLOBAL_STATE.current_crystal, wave=GLOBAL_STATE.current_wave)
end
