require "scribble/sparkle"

# Define computation pattern
Sparkle.pattern do |p|
  p.weave = 500
  p.bio = false
end

# Use Sparkle modules
math = Sparkle::Module.new("std**math")

# Perform computation
result = math.compute do |c|
  c.add(1.0, 2.0)
end

puts "Result: #{result}"
