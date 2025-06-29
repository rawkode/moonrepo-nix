#!/usr/bin/env -S nix develop --command bash

# This script runs inside the Nix flake environment
# All tools defined in flake.nix are available

echo "Running script with Nix flake environment"
echo "==============================================="

# These commands will work because they're provided by the flake
echo "Node version:"
node --version

echo -e "\nPython version:"
python3 --version

echo -e "\nRust version:"
rustc --version

echo -e "\nAll tools from flake.nix are available!"