module Lend

export borrow, return_borrowed, with_borrowed
export track_usage, show_borrowed, clear_borrowed

using Base.Threads

mutable struct BorrowedResource{T}
    value::T
    borrowed::Bool
    owner::String
    timestamp::Float64
end

const BORROWED_RESOURCES = Dict{String, BorrowedResource}()
const BORROW_LOCK = ReentrantLock()

"""
Borrow a resource
"""
function borrow(resource::T, name::String) where T
    lock(BORROW_LOCK) do
        if haskey(BORROWED_RESOURCES, name)
            throw(ErrorException("Resource $name is already borrowed"))
        end
        BORROWED_RESOURCES[name] = BorrowedResource(resource, true, string(current_task()), time())
        resource
    end
end

"""
Return a borrowed resource
"""
function return_borrowed(name::String)
    lock(BORROW_LOCK) do
        if !haskey(BORROWED_RESOURCES, name)
            throw(ErrorException("Resource $name was not borrowed"))
        end
        resource = BORROWED_RESOURCES[name]
        delete!(BORROWED_RESOURCES, name)
        resource.value
    end
end

"""
Use resource within a scope
"""
function with_borrowed(f::Function, resource::T, name::String) where T
    borrowed = borrow(resource, name)
    try
        f(borrowed)
    finally
        return_borrowed(name)
    end
end

"""
Track resource usage
"""
function track_usage(name::String)
    lock(BORROW_LOCK) do
        if haskey(BORROWED_RESOURCES, name)
            resource = BORROWED_RESOURCES[name]
            (
                borrowed=resource.borrowed,
                owner=resource.owner,
                duration=time() - resource.timestamp
            )
        else
            nothing
        end
    end
end

"""
Show all borrowed resources
"""
function show_borrowed()
    lock(BORROW_LOCK) do
        Dict(name => (
            owner=resource.owner,
            duration=time() - resource.timestamp
        ) for (name, resource) in BORROWED_RESOURCES)
    end
end

"""
Clear all borrowed resources
"""
function clear_borrowed()
    lock(BORROW_LOCK) do
        empty!(BORROWED_RESOURCES)
    end
end

end
