cat > "setup_sparkle_repo.sh" << 'SETUPSCRIPT'
#!/bin/bash

PURPLE='\033[0;35m'
NC='\033[0m'

echo -e "${PURPLE}Setting up Sparkle Framework...${NC}"

# Set up sparkle repository
cd /home/guavabot1/scribble/scribble/lib/sparkle
git init
git remote add origin git@github.com:isdood/scribble.git
git checkout -b sparkle

# Update timestamp in spark launcher
sed -i "s/Created: .*/Created: 2025-01-25 00:52:42/" bin/spark

# Add all files
git add -A
git commit -m "feat(sparkle): add SpiderWeb Resonance Framework with bio/weave patterns"

# Navigate to the root of the main repo
cd /home/guavabot1/scribble/scribble

# Add sparkle as a submodule
git submodule init
git submodule add -f ./lib/sparkle
git add .gitmodules lib/sparkle
git commit -m "feat: add sparkle framework as submodule"

# Push changes
cd lib/sparkle
git push -u origin sparkle

cd ../..
git push origin main

echo -e "${PURPLE}✨ Sparkle repository and submodule setup complete!${NC}"
SETUPSCRIPT

chmod +x setup_sparkle_repo.sh
