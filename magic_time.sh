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

# Function to fix git push command
fix_git_push() {
    echo -e "\n${BLUE}🔧 Setting up Git push command...${NC}"
    local git_root=$(get_git_root)

    # Create or update git_push script
    cat > "${git_root}/bin/git_push" << 'GITPUSH'
#!/bin/bash

# Get current branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

# Add all changes
git add .

# Commit with timestamp
git commit -m "dbg: $(date -u '+%Y-%m-%d %H:%M:%S')"

# Push to current branch
git push origin "${CURRENT_BRANCH}"
GITPUSH

    chmod +x "${git_root}/bin/git_push"
}

# Function to ensure we're on the sparkle branch
setup_git_branch() {
    echo -e "\n${BLUE}🔄 Setting up Git branch...${NC}"

    # Fetch all branches
    git fetch origin

    # Create or switch to sparkle branch
    if git rev-parse --verify sparkle >/dev/null 2>&1; then
        echo "Switching to existing sparkle branch..."
        git checkout sparkle
        git pull origin sparkle || true
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
    CREATED = "2025-01-25 02:53:12"
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

    # Ensure bin directory exists
    mkdir -p "${git_root}/bin"
}

# Function to stage and commit changes
commit_changes() {
    echo -e "\n${BLUE}📦 Committing changes...${NC}"
    local git_root=$(get_git_root)

    git add mod.cr
    git add "${git_root}/config/modules.yml"
    git add "${git_root}/bin/git_push"

    git commit -m "feat: integrate sparkle module with scribble framework

- Add Crystal module definition
- Configure module in Scribble framework
- Set up proper module structure
- Update module configuration
- Fix git push command

Created: 2025-01-25 02:53:12
Author: isdood"
}

# Function to attempt pushing changes
push_changes() {
    echo -e "\n${BLUE}🚀 Pushing changes...${NC}"

    # First try to push
    if git push origin sparkle 2>/dev/null; then
        echo -e "${GREEN}✨ Changes pushed successfully!${NC}"
    else
        echo -e "${BLUE}⚠️  Need to integrate with main first...${NC}"

        # Fetch latest main
        git fetch origin main

        # Try to rebase on main
        if git rebase origin/main; then
            echo "Rebased on main, pushing changes..."
            git push origin sparkle --force-with-lease
        else
            echo -e "${RED}⚠️  Rebase conflicts detected. Please resolve manually:${NC}"
            echo "1. Resolve the conflicts in the files"
            echo "2. git add . "
            echo "3. git rebase --continue"
            echo "4. git push origin sparkle --force-with-lease"
            exit 1
        fi
    fi
}

# Main execution
main() {
    echo -e "${BLUE}🔍 Starting Sparkle module integration...${NC}"

    # Verify we're in the right directory
    check_sparkle_dir

    # Setup git branch
    setup_git_branch

    # Create module files
    create_module_files

    # Fix git push command
    fix_git_push

    # Commit changes
    commit_changes

    # Push changes
    push_changes

    echo -e "\n${GREEN}✨ Integration completed successfully!${NC}"
    echo -e "${BLUE}Next steps:${NC}"
    echo "1. Visit: https://github.com/isdood/scribble/pull/new/sparkle"
    echo "2. Create a pull request to merge the sparkle branch into main"
    echo "3. Add reviewers and wait for approval"
    echo -e "\n${PURPLE}Note: The git_push command has been updated to work with the current branch${NC}"
}

# Run the script
main

