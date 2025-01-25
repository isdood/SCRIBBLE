cat > ~/.config/fish/functions/git_push.fish << 'FISHFUNC'
function git_push
    # Get current branch
    set -l current_branch (git rev-parse --abbrev-ref HEAD)

    # Add all changes
    git add .

    # Try to commit (won't fail if nothing to commit)
    git commit -m "dbg: $(date -u '+%Y-%m-%d %H:%M:%S')" 2>/dev/null || true

    # Push to current branch explicitly
    git push origin $current_branch
end
FISHFUNC
