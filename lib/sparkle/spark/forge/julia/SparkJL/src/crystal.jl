# Constructor for creating a Crystal from 3 dimensions
function Crystal(x::Integer, y::Integer, z::Integer, spacing::Float64)
    Crystal((x, y, z), spacing)
end
