#!/bin/bash

# Set strict error handling
set -euo pipefail
IFS=$'\n\t'

PURPLE='\033[0;35m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${PURPLE}🌟 Sparkle Git Integration Setup${NC}"

# Function to check if we're in the sparkle directory
check_sparkle_dir() {
    if [[ ! "$PWD" =~ /sparkle$ ]]; then
        echo -e "${RED}Error: Must be run from the sparkle directory${NC}"
        exit 1
    fi
}

# Function to get git root directory
get_git_root() {
    git rev-parse --show-toplevel
}

# Function to fix submodule issue
fix_submodule() {
    echo -e "\n${BLUE}🔧 Fixing submodule configuration...${NC}"
    local git_root=$(get_git_root)

    # Remove submodule entries if they exist
    if [ -f "${git_root}/.gitmodules" ]; then
        echo "Removing submodule configuration..."
        git config --file "${git_root}/.gitmodules" --remove-section "submodule.lib/sparkle" 2>/dev/null || true
        git add "${git_root}/.gitmodules"
    fi

    # Remove from git cache and modules
    cd "${git_root}"
    git rm --cached lib/sparkle 2>/dev/null || true
    rm -rf .git/modules/lib/sparkle 2>/dev/null || true

    # Recreate directory structure
    mkdir -p lib/sparkle
    cd lib/sparkle
}

# Function to ensure we're on the sparkle branch
setup_git_branch() {
    echo -e "\n${BLUE}🔄 Setting up Git branch...${NC}"

    # Create or switch to sparkle branch
    if git rev-parse --verify sparkle >/dev/null 2>&1; then
        echo "Switching to existing sparkle branch..."
        git checkout sparkle
    else
        echo "Creating new sparkle branch..."
        git checkout -b sparkle
    fi
}

# Function to create necessary module files
create_module_files() {
    echo -e "\n${BLUE}📝 Creating module files...${NC}"
    local git_root=$(get_git_root)

    # Create Crystal module definition
    cat > "mod.cr" << 'MOD'
module Scribble
  module Sparkle
    VERSION = "1.0.0"
    CREATED = "2025-01-25 03:05:28"
    TENDER  = "isdood"

    def self.root_path
      File.dirname(__FILE__)
    end

    def self.pattern_path
      File.join(root_path, "patterns")
    end

    def self.config_path
      File.join(root_path, "config.sparkle")
    end
  end
end
MOD

    # Create module configuration
    mkdir -p "${git_root}/config"
    cat > "${git_root}/config/modules.yml" << 'MODULES'
modules:
  sparkle:
    path: lib/sparkle
    version: 1.0.0
    dependencies:
      - core
      - std
    features:
      - package_management
      - build_system
      - pattern_matching
MODULES

    # Create patterns directory
    mkdir -p patterns
    touch patterns/.keep
}

# Function to stage and commit changes
commit_changes() {
    echo -e "\n${BLUE}📦 Committing changes...${NC}"
    local git_root=$(get_git_root)

    # Stage all changes
    git add .

    git commit -m "feat: initialize sparkle module structure

- Remove submodule configuration
- Add Crystal module definition
- Set up proper directory structure
- Configure module paths and dependencies

Created: 2025-01-25 03:05:28
Author: isdood" || true
}

# Function to push changes
push_changes() {
    echo -e "\n${BLUE}🚀 Pushing changes...${NC}"
    local BRANCH=$(git rev-parse --abbrev-ref HEAD)
    git push origin "${BRANCH}"
}

# Main execution
main() {
    echo -e "${BLUE}🔍 Starting Sparkle module integration...${NC}"

    # Verify we're in the right directory
    check_sparkle_dir

    # Fix submodule issue
    fix_submodule

    # Setup git branch
    setup_git_branch

    # Create module files
    create_module_files

    # Commit changes
    commit_changes

    # Push changes
    push_changes

    echo -e "\n${GREEN}✨ Integration completed successfully!${NC}"
    echo -e "${BLUE}Next steps:${NC}"
    echo "1. Visit: https://github.com/isdood/scribble/pull/new/sparkle"
    echo "2. Create a pull request to merge the sparkle branch into main"
    echo "3. Add reviewers and wait for approval"
}

# Run the script
main

