using BioStructures

struct DNAEncoding
    sequence::Vector{UInt8}

    function DNAEncoding(seq::String)
        # 4-bit DNA operations
        encoding = map(base -> begin
            if base == 'A'; 0x0
            elseif base == 'T'; 0x1
            elseif base == 'C'; 0x2
            elseif base == 'G'; 0x3
            else; throw(ArgumentError("Invalid base: $base"))
            end
        end, seq)
        new(encoding)
    end
end

struct ProteinFolder
    chain::Chain
    energy::Float64
    angles::Vector{Float64}
end
