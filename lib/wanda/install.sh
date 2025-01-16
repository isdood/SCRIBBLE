#!/bin/bash
set -e

# Store the script's directory and move there
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Function to display the clock
display_clock() {
  while true; do
    clear

    # Display current time in AM/PM format with orange square brackets
    echo ""
    echo -e "              \033[0;33m[\033[0m$(date +"%I:%M:%S %p")\033[0;33m]\033[0m"
    echo ""

    # ASCII Art for Wanda in purple with yellow stars forming a ">" shape to the right
    echo -e "    \033[1;35m _       __                 __   \033[1;33m *  *\033[0m"
    echo -e "    \033[1;35m| |     / /___ _____  ____/ /___    \033[1;33m*  *\033[0m"
    echo -e "    \033[1;35m| | /| / / __ \`/ __ \/ __  / __ \\     \033[1;33m*  *\033[0m"
    echo -e "    \033[1;35m| |/ |/ / /_/ / / / / /_/ / /_/ /   \033[1;33m   *  *\033[0m"
    echo -e "    \033[1;35m|__/|__/\__,_/_/ /_/\__,_/\____/      \033[1;33m*  *\033[0m"
    echo -e "                                        \033[1;33m *  *\033[0m"
    echo -e "                                       \033[1;33m*  *\033[0m"

    # Install message
    echo ""
    echo -e "\nThis will install \033[1;35mWanda AI Assistant\033[0m on your system."
    echo -e "\033[0;33mPress Enter\033[0m to continue or \033[0;33mCtrl+C\033[0m to cancel... \c"

    sleep 1
  done
}

# Run the clock in the background
display_clock &

# Wait for user to press Enter or Ctrl+C to cancel
read -r -p ""

# Kill the clock background process
kill $!

# Move to a new line after user input
echo ""

echo "Cleaning previous build..."
cargo clean

echo "Building Wanda..."
CARGO_TARGET_DIR="$SCRIPT_DIR/target" cargo build --release

echo "Installing Wanda binaries..."
CARGO_TARGET_DIR="$SCRIPT_DIR/target" cargo install --path .

echo "Creating directories..."
mkdir -p ~/.local/share/wanda
mkdir -p ~/.config/systemd/user
mkdir -p ~/.local/bin

echo "Stopping existing service..."
systemctl --user stop wanda.service || true

echo "Cleaning up old socket..."
rm -f ~/.local/share/wanda/wanda.sock

echo "Installing binaries..."
# Explicit path to target directory
cp "$SCRIPT_DIR/target/release/wanda" ~/.local/bin/
cp "$SCRIPT_DIR/target/release/wandad" ~/.local/bin/

echo "Creating systemd service..."
cat > ~/.config/systemd/user/wanda.service << EOF
[Unit]
Description=Wanda AI Assistant Service
After=network.target

[Service]
ExecStart=/home/$USER/.local/bin/wandad
Restart=always
Environment=RUST_LOG=info

[Install]
WantedBy=default.target
EOF

echo "Setting up service..."
systemctl --user daemon-reload
systemctl --user enable wanda.service

echo "Starting service..."
systemctl --user start wanda.service

echo "Waiting for service to be ready..."
for i in {1..10}; do
    echo "Waiting for socket (attempt $i/10)..."
    if [ -S ~/.local/share/wanda/wanda.sock ]; then
        ls -l ~/.local/share/wanda/wanda.sock
        break
    fi
    sleep 1
done

echo "Testing connection..."
~/.local/bin/wanda status

echo "Installation completed at $(date -u +"%I:%M:%S %p")"
echo "Service is running and responding to commands"
echo "You can check the service status with: systemctl --user status wanda"

# Check user's current shell and update PATH accordingly
SHELL_NAME=$(basename "$SHELL")

case "$SHELL_NAME" in
    bash)
        PROFILE="$HOME/.bashrc"
        ;;
    zsh)
        PROFILE="$HOME/.zshrc"
        ;;
    fish)
        PROFILE="$HOME/.config/fish/config.fish"
        ;;
    *)
        echo "Unsupported shell: $SHELL_NAME"
        exit 1
        ;;
esac

# Add PATH update if needed
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    if [ "$SHELL_NAME" = "fish" ]; then
        if ! grep -q "set -Ux PATH \$HOME/.local/bin \$PATH" "$PROFILE"; then
            echo "set -Ux PATH \$HOME/.local/bin \$PATH" >> "$PROFILE" || {
                echo ""
                echo "NOTE: Please add ~/.local/bin to your PATH by adding this line to your $PROFILE:"
                echo "set -Ux PATH \$HOME/.local/bin \$PATH"
            }
        fi
    else
        if ! grep -q "export PATH=\$HOME/.local/bin:\$PATH" "$PROFILE"; then
            echo "export PATH=\$HOME/.local/bin:\$PATH" >> "$PROFILE" || {
                echo ""
                echo "NOTE: Please add ~/.local/bin to your PATH by adding this line to your $PROFILE:"
                echo "export PATH=\$HOME/.local/bin \$PATH"
            }
        fi
    fi
fi
