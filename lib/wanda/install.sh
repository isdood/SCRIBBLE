#!/bin/bash
set -e

# Clear the screen
clear

echo "Installation started at 2025-01-15 22:48:34 by isdood"

echo "Cleaning previous build..."
cargo clean

echo "Building Wanda..."
cargo build --release

echo "Installing Wanda binaries..."
cargo install --path .

echo "Creating directories..."
mkdir -p ~/.local/share/wanda
mkdir -p ~/.config/systemd/user
mkdir -p ~/.local/bin

echo "Stopping existing service..."
systemctl --user stop wanda.service || true

echo "Cleaning up old socket..."
rm -f ~/.local/share/wanda/wanda.sock

echo "Installing binaries..."
cp target/release/wanda ~/.local/bin/
cp target/release/wandad ~/.local/bin/

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

echo "Installation completed at 2025-01-15 22:48:34"
echo "Service is running and responding to commands"
echo "You can check the service status with: systemctl --user status wanda"

# Add PATH update hint if needed
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "NOTE: Please add ~/.local/bin to your PATH by adding this line to your ~/.bashrc:"
    echo "export PATH=\$HOME/.local/bin:\$PATH"
fi
