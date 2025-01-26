module Murmur

export murmur32, murmur64, murmur128
export hash_file, hash_stream

"""
32-bit Murmur3 hash
"""
function murmur32(data::Vector{UInt8}, seed::UInt32=0x00000000)
    len = length(data)
    nblocks = div(len, 4)
    h1 = seed

    # Constants
    c1 = 0xcc9e2d51
    c2 = 0x1b873593

    # Body
    for block in 1:nblocks
        k1 = UInt32(data[block*4-3]) |
             UInt32(data[block*4-2]) << 8 |
             UInt32(data[block*4-1]) << 16 |
             UInt32(data[block*4]) << 24

        k1 *= c1
        k1 = (k1 << 15) | (k1 >> 17)
        k1 *= c2

        h1 ⊻= k1
        h1 = (h1 << 13) | (h1 >> 19)
        h1 = h1 * 5 + 0xe6546b64
    end

    # Tail
    tail_start = nblocks * 4 + 1
    k1 = 0x00000000
    if tail_start <= len
        for i in 0:min(len-tail_start, 3)
            k1 ⊻= UInt32(data[tail_start+i]) << (i * 8)
        end
        k1 *= c1
        k1 = (k1 << 15) | (k1 >> 17)
        k1 *= c2
        h1 ⊻= k1
    end

    # Finalization
    h1 ⊻= UInt32(len)
    h1 ⊻= h1 >> 16
    h1 *= 0x85ebca6b
    h1 ⊻= h1 >> 13
    h1 *= 0xc2b2ae35
    h1 ⊻= h1 >> 16

    h1
end

"""
64-bit Murmur2 hash
"""
function murmur64(data::Vector{UInt8}, seed::UInt64=0x0000000000000000)
    len = length(data)
    nblocks = div(len, 8)
    h = seed ⊻ (len * 0xc6a4a7935bd1e995)

    # Constants
    m = 0xc6a4a7935bd1e995
    r = 47

    # Body
    for block in 1:nblocks
        k = UInt64(data[block*8-7]) |
            UInt64(data[block*8-6]) << 8 |
            UInt64(data[block*8-5]) << 16 |
            UInt64(data[block*8-4]) << 24 |
            UInt64(data[block*8-3]) << 32 |
            UInt64(data[block*8-2]) << 40 |
            UInt64(data[block*8-1]) << 48 |
            UInt64(data[block*8]) << 56

        k *= m
        k ⊻= k >> r
        k *= m

        h ⊻= k
        h *= m
    end

    # Tail
    tail_start = nblocks * 8 + 1
    if tail_start <= len
        for i in 0:min(len-tail_start, 7)
            h ⊻= UInt64(data[tail_start+i]) << (i * 8)
        end
        h *= m
    end

    # Finalization
    h ⊻= h >> r
    h *= m
    h ⊻= h >> r

    h
end

"""
128-bit Murmur3 hash
"""
function murmur128(data::Vector{UInt8}, seed::UInt64=0x0000000000000000)
    len = length(data)
    nblocks = div(len, 16)

    # Constants
    c1 = 0x87c37b91114253d5
    c2 = 0x4cf5ad432745937f

    # Initialize hash values
    h1 = seed
    h2 = seed

    # Body
    for block in 1:nblocks
        # Get 128 bits
        k1 = UInt64(data[block*16-15]) |
            UInt64(data[block*16-14]) << 8 |
            UInt64(data[block*16-13]) << 16 |
            UInt64(data[block*16-12]) << 24 |
            UInt64(data[block*16-11]) << 32 |
            UInt64(data[block*16-10]) << 40 |
            UInt64(data[block*16-9]) << 48 |
            UInt64(data[block*16-8]) << 56

        k2 = UInt64(data[block*16-7]) |
            UInt64(data[block*16-6]) << 8 |
            UInt64(data[block*16-5]) << 16 |
            UInt64(data[block*16-4]) << 24 |
            UInt64(data[block*16-3]) << 32 |
            UInt64(data[block*16-2]) << 40 |
            UInt64(data[block*16-1]) << 48 |
            UInt64(data[block*16]) << 56

        # Mix k1
        k1 *= c1
        k1 = (k1 << 31) | (k1 >> 33)
        k1 *= c2
        h1 ⊻= k1

        h1 = (h1 << 27) | (h1 >> 37)
        h1 += h2
        h1 = h1 * 5 + 0x52dce729

        # Mix k2
        k2 *= c2
        k2 = (k2 << 33) | (k2 >> 31)
        k2 *= c1
        h2 ⊻= k2

        h2 = (h2 << 31) | (h2 >> 33)
        h2 += h1
        h2 = h2 * 5 + 0x38495ab5
    end

    # Tail
    tail_start = nblocks * 16 + 1
    if tail_start <= len
        k1 = 0x0000000000000000
        k2 = 0x0000000000000000

        if len - tail_start >= 15
            k2 ⊻= UInt64(data[tail_start+14]) << 48
        end
        if len - tail_start >= 14
            k2 ⊻= UInt64(data[tail_start+13]) << 40
        end
        if len - tail_start >= 13
            k2 ⊻= UInt64(data[tail_start+12]) << 32
        end
        if len - tail_start >= 12
            k2 ⊻= UInt64(data[tail_start+11]) << 24
        end
        if len - tail_start >= 11
            k2 ⊻= UInt64(data[tail_start+10]) << 16
        end
        if len - tail_start >= 10
            k2 ⊻= UInt64(data[tail_start+9]) << 8
        end
        if len - tail_start >= 9
            k2 ⊻= UInt64(data[tail_start+8])
            k2 *= c2
            k2 = (k2 << 33) | (k2 >> 31)
            k2 *= c1
            h2 ⊻= k2
        end

        if len - tail_start >= 8
            k1 ⊻= UInt64(data[tail_start+7]) << 56
        end
        if len - tail_start >= 7
            k1 ⊻= UInt64(data[tail_start+6]) << 48
        end
        if len - tail_start >= 6
            k1 ⊻= UInt64(data[tail_start+5]) << 40
        end
        if len - tail_start >= 5
            k1 ⊻= UInt64(data[tail_start+4]) << 32
        end
        if len - tail_start >= 4
            k1 ⊻= UInt64(data[tail_start+3]) << 24
        end
        if len - tail_start >= 3
            k1 ⊻= UInt64(data[tail_start+2]) << 16
        end
        if len - tail_start >= 2
            k1 ⊻= UInt64(data[tail_start+1]) << 8
        end
        if len - tail_start >= 1
            k1 ⊻= UInt64(data[tail_start])
            k1 *= c1
            k1 = (k1 << 31) | (k1 >> 33)
            k1 *= c2
            h1 ⊻= k1
        end
    end

    # Finalization
    h1 ⊻= UInt64(len)
    h2 ⊻= UInt64(len)

    h1 += h2
    h2 += h1

    # Final mix functions
    h1 ⊻= h1 >> 33
    h1 *= 0xff51afd7ed558ccd
    h1 ⊻= h1 >> 33
    h1 *= 0xc4ceb9fe1a85ec53
    h1 ⊻= h1 >> 33

    h2 ⊻= h2 >> 33
    h2 *= 0xff51afd7ed558ccd
    h2 ⊻= h2 >> 33
    h2 *= 0xc4ceb9fe1a85ec53
    h2 ⊻= h2 >> 33

    return (h1, h2)
end

"""
Hash a file using Murmur3
"""
function hash_file(filename::String; bits::Integer=32)
    open(filename, "r") do file
        data = read(file)
        if bits == 32
            murmur32(data)
        elseif bits == 64
            murmur64(data)
        elseif bits == 128
            murmur128(data)
        else
            throw(ArgumentError("bits must be 32, 64, or 128"))
        end
    end
end

"""
Hash a stream using Murmur3
"""
function hash_stream(io::IO; bits::Integer=32)
    data = read(io)
    if bits == 32
        murmur32(data)
    elseif bits == 64
        murmur64(data)
    elseif bits == 128
        murmur128(data)
    else
        throw(ArgumentError("bits must be 32, 64, or 128"))
    end
end

end
