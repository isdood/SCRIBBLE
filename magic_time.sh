# First, let's remove the accidentally added repository
cd /home/guavabot1/scribble/scribble/lib
git rm --cached sparkle

# Then properly add it as a submodule
git submodule add git@github.com:isdood/scribble.git sparkle

# Now let's update the sparkle configuration
cd sparkle

cat > "config.sparkle" << SPARKLECONFIG
# Sparkle Garden Configuration
garden:
  version: "1.0.0"
  created: "2025-01-25 01:48:30"
  tender: "isdood"

packages:
  - name: "std/math"
    version: "1.0.0"
    config: "std/math/config.spark"
    whispers:
      - "The mathematical moonlight dances"
      - "Numbers weave through starlit paths"
SPARKLECONFIG

# Update the math module configuration
mkdir -p std/math
cat > "std/math/config.spark" << SPARKCONFIG
# Math Module Configuration
pattern:
  weave: 500  # Thread weaving intensity
  bio: false  # Bio-computational mode

garden:
  planted: "2025-01-25 01:48:30"
  tender: "isdood"
  sparkles: ["add", "sub", "mul", "div"]

whispers:
  - "Numbers dance in moonlit arrays"
  - "Calculations flow like starlight"
SPARKCONFIG

# Commit the changes in sparkle
git add .
git commit -m "feat(sparkle): initialize standard library with math module"
git push origin sparkle

# Go back to main repo and commit the submodule
cd ..
git add sparkle
git commit -m "feat: add sparkle framework as submodule"
git push origin main
