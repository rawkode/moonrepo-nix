#!/usr/bin/env nix-shell
#!nix-shell -i bash -p nodejs_20 python3 rustc cargo

# This script uses nix-shell with inline package specification
# Useful when you don't have a flake.nix or shell.nix file

echo "Running script with nix-shell"
echo "============================="

echo "Node.js version:"
node --version

echo -e "\nPython version:"
python3 --version

echo -e "\nRust version:"
rustc --version

echo -e "\nThis script is self-contained and doesn't need a flake.nix!"