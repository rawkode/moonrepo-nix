# Nix Flakes Example

This example demonstrates using moon with a Nix flake environment.

## Setup

1. Ensure you have Nix with flakes enabled
2. Run `nix flake lock` to generate the lock file
3. Run moon tasks:

```bash
# Enter the flake environment and run a task
moon run hello

# Check Python version from the flake
moon run python-version

# Check Rust version from the flake
moon run rust-version
```

## How it works

The moon Nix plugin automatically detects the `flake.nix` file and wraps all moon task commands with `nix develop --command <your-command>`. This ensures all tasks run within the reproducible Nix environment defined by the flake.