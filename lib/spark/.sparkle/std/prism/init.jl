"""
Prism filesystem integration for Sparkle
"""
module PrismIntegration

export init_prism, mount_prism, unmount_prism

using ..Prism

# Global filesystem instance
const PRISM_FS = Ref{Union{PrismFS,Nothing}}(nothing)

"""
Initialize Prism filesystem
"""
function init_prism(resolution=(32,32,32))
    PRISM_FS[] = PrismFS(resolution)
    println("✨ Initialized Prism filesystem with resolution $resolution")
end

"""
Mount Prism filesystem
"""
function mount_prism(mount_point::String)
    if isnothing(PRISM_FS[])
        init_prism()
    end
    PRISM_FS[] = mount(PRISM_FS[], mount_point)
    println("📦 Mounted Prism filesystem at $mount_point")
end

"""
Unmount Prism filesystem
"""
function unmount_prism()
    if !isnothing(PRISM_FS[])
        PRISM_FS[] = unmount(PRISM_FS[])
        println("🔽 Unmounted Prism filesystem")
    end
end

end # module PrismIntegration
