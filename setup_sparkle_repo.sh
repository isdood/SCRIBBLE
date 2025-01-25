#!/bin/bash

# Set up sparkle repository
cd /home/guavabot1/scribble/scribble/lib/sparkle
git init
git remote add origin git@github.com:isdood/scribble.git
git checkout -b sparkle
git add -A
git commit -m "feat(sparkle): add SpiderWeb Resonance Framework with bio/weave patterns"

# Set up submodule in main repo
cd ..
git submodule add ./sparkle lib/sparkle
git add .gitmodules lib/sparkle
git commit -m "feat: add sparkle framework as submodule"

# Push changes
cd sparkle
git push -u origin sparkle
cd ..
git push origin master

echo "✨ Sparkle repository and submodule setup complete!"
