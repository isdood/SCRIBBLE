module Align

export align_left, align_right, align_center

function align_left(text::String, width::Integer)
    rpad(text, width)
end

function align_right(text::String, width::Integer)
    lpad(text, width)
end

function align_center(text::String, width::Integer)
    padding = width - length(text)
    left_pad = div(padding, 2)
    right_pad = padding - left_pad
    " "^left_pad * text * " "^right_pad
end

end
