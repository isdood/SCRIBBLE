module Shout

export shout, whisper, toggle_case

shout(text::String) = uppercase(text)
whisper(text::String) = lowercase(text)
toggle_case(text::String) = join(islowercase(c) ? uppercase(c) : lowercase(c) for c in text)

end
