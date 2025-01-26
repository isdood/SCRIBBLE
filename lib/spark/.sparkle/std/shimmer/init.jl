module Shimmer

export animate, sparkle, fade, pulse
export ShimmerEffect, apply_effect, chain_effects

using UnicodePlots
using ColorSchemes

"""
Animation effect types
"""
@enum ShimmerEffect begin
    SPARKLE
    FADE
    PULSE
    RAINBOW
    GLITTER
    WAVE
end

"""
Animate text with effect
"""
function animate(text::String, effect::ShimmerEffect; duration::Float64=1.0)
    frames = Int(duration * 30)  # 30 fps
    for i in 1:frames
        # Clear previous frame
        print("\033[2K\033[1G")

        # Apply effect
        if effect == SPARKLE
            print(_sparkle_frame(text, i))
        elseif effect == FADE
            print(_fade_frame(text, i, frames))
        elseif effect == PULSE
            print(_pulse_frame(text, i))
        elseif effect == RAINBOW
            print(_rainbow_frame(text, i))
        elseif effect == GLITTER
            print(_glitter_frame(text, i))
        elseif effect == WAVE
            print(_wave_frame(text, i))
        end

        flush(stdout)
        sleep(1/30)
    end
    println()
end

"""
Create sparkle effect
"""
function sparkle(text::String; duration::Float64=1.0)
    animate(text, SPARKLE, duration=duration)
end

"""
Create fade effect
"""
function fade(text::String; duration::Float64=1.0)
    animate(text, FADE, duration=duration)
end

"""
Create pulse effect
"""
function pulse(text::String; duration::Float64=1.0)
    animate(text, PULSE, duration=duration)
end

# Internal frame generation functions
function _sparkle_frame(text::String, frame::Int)
    chars = collect(text)
    spark_pos = rand(1:length(chars))
    chars[spark_pos] = '✨'
    join(chars)
end

function _fade_frame(text::String, frame::Int, total_frames::Int)
    opacity = abs(sin(π * frame / total_frames))
    "\033[38;5;$(Int(round(255 * opacity)))m$text\033[0m"
end

function _pulse_frame(text::String, frame::Int)
    scale = 1.0 + 0.2 * sin(2π * frame / 30)
    "\033[${scale}m$text\033[0m"
end

function _rainbow_frame(text::String, frame::Int)
    colors = ColorSchemes.rainbow
    join(["\033[38;5;$(colors[(i+frame)%length(colors)])m$(c)" for (i,c) in enumerate(text)])
end

function _glitter_frame(text::String, frame::Int)
    chars = collect(text)
    for i in 1:length(chars)
        if rand() < 0.1
            chars[i] = '.'
        end
    end
    join(chars)
end

function _wave_frame(text::String, frame::Int)
    chars = collect(text)
    for (i, c) in enumerate(chars)
        offset = sin(2π * (i + frame) / 20)
        chars[i] = ' ' ^ Int(round(2 + 2 * offset)) * c
    end
    join(chars)
end

end
