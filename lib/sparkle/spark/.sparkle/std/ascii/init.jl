module ASCII

export to_ascii, strip_non_ascii, is_ascii, ascii_only
export to_ascii_upper, to_ascii_lower
export encode_hex, decode_hex

"""
Convert string to ASCII, replacing non-ASCII characters
"""
function to_ascii(s::AbstractString)
    join(isascii(c) ? c : '?' for c in s)
end

"""
Remove all non-ASCII characters from string
"""
function strip_non_ascii(s::AbstractString)
    join(c for c in s if isascii(c))
end

"""
Check if string contains only ASCII characters
"""
function is_ascii(s::AbstractString)
    all(isascii, s)
end

"""
Ensure string contains only ASCII characters or throw error
"""
function ascii_only(s::AbstractString)
    is_ascii(s) || throw(ArgumentError("String contains non-ASCII characters"))
    s
end

"""
Convert string to uppercase ASCII
"""
function to_ascii_upper(s::AbstractString)
    ascii_only(uppercase(s))
end

"""
Convert string to lowercase ASCII
"""
function to_ascii_lower(s::AbstractString)
    ascii_only(lowercase(s))
end

"""
Encode string as ASCII hex
"""
function encode_hex(s::AbstractString)
    bytes = Vector{UInt8}(s)
    join(string(b, base=16, pad=2) for b in bytes)
end

"""
Decode ASCII hex string
"""
function decode_hex(s::AbstractString)
    length(s) % 2 == 0 || throw(ArgumentError("Hex string length must be even"))
    bytes = [parse(UInt8, s[i:i+1], base=16) for i in 1:2:length(s)]
    String(bytes)
end

end
