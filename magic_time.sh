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

# Function to ensure git configuration is correct
setup_git_config() {
    echo -e "\n${BLUE}🔧 Setting up Git configuration...${NC}"

    # Get the git root directory
    local git_root=$(get_git_root)

    # Update git config for the repository
    git config push.default current
    git config pull.rebase true

    # Create git_push script in the repository root
    mkdir -p "${git_root}/bin"
    cat > "${git_root}/bin/git_push" << 'GITPUSH'
#!/bin/bash
set -euo pipefail

# Get current branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

# Ensure we have latest changes
git fetch origin "${CURRENT_BRANCH}" || true

# Add all changes
git add .

# Commit with timestamp
git commit -m "dbg: $(date -u '+%Y-%m-%d %H:%M:%S')" || true

# Push to current branch explicitly
git push origin "${CURRENT_BRANCH}:${CURRENT_BRANCH}"
GITPUSH

    chmod +x "${git_root}/bin/git_push"
}

# Function to ensure we're on the sparkle branch
setup_git_branch() {
    echo -e "\n${BLUE}🔄 Setting up Git branch...${NC}"

    # Ensure we have the latest changes
    git fetch origin

    # Switch to or create sparkle branch
    if git rev-parse --verify sparkle >/dev/null 2>&1; then
        echo "Switching to existing sparkle branch..."
        git checkout sparkle
        git pull origin sparkle || true
    else
        echo "Creating new sparkle branch..."
        git checkout -b sparkle
        git push -u origin sparkle || true
    fi
}

# Function to push changes safely
safe_push() {
    echo -e "\n${BLUE}🚀 Pushing changes...${NC}"
    local branch=$(git rev-parse --abbrev-ref HEAD)

    # Ensure branch exists on remote
    git push origin "${branch}:${branch}" || {
        echo -e "${RED}Failed to push directly. Attempting to fix...${NC}"
        git fetch origin
        git push -u origin "${branch}"
    }
}

# Function to create necessary files
create_module_files() {
    echo -e "\n${BLUE}📝 Creating module files...${NC}"
    local git_root=$(get_git_root)

    # Create Crystal module definition
    cat > "mod.cr" << 'MOD'
module Scribble
  module Sparkle
    VERSION = "1.0.0"
    CREATED = "2025-01-25 02:55:14"
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
}

# Function to commit changes
commit_changes() {
    echo -e "\n${BLUE}📦 Committing changes...${NC}"
    local git_root=$(get_git_root)

    # Stage changes
    git add mod.cr || true
    git add "${git_root}/config/modules.yml" || true
    git add "${git_root}/bin/git_push" || true

    # Commit
    git commit -m "feat: integrate sparkle module with scribble framework

- Add Crystal module definition
- Configure module in Scribble framework
- Set up proper module structure
- Fix git push handling
- Update module configuration

Created: 2025-01-25 02:55:14
Author: isdood" || true
}

# Main execution
main() {
    echo -e "${BLUE}🔍 Starting Sparkle module integration...${NC}"

    # Verify we're in the right directory
    check_sparkle_dir

    # Setup git configuration
    setup_git_config

    # Setup git branch
    setup_git_branch

    # Create module files
    create_module_files

    # Commit changes
    commit_changes

    # Push changes safely
    safe_push

    echo -e "\n${GREEN}✨ Integration completed successfully!${NC}"
    echo -e "${BLUE}Next steps:${NC}"
    echo "1. Visit: https://github.com/isdood/scribble/pull/new/sparkle"
    echo "2. Create a pull request to merge the sparkle branch into main"
    echo "3. Add reviewers and wait for approval"
}

# Run the script
main

