# Moon Nix Plugin Examples

This directory contains examples demonstrating how to use the moon Nix toolchain plugin with different Nix-based development environments.

## Available Examples

### 1. [Nix Flakes](./flakes/)
Modern, reproducible Nix environments using flakes. Best for projects that need strict reproducibility and are already using Nix flakes.

### 2. [Devenv](./devenv/)
High-level development environment management with devenv. Great for teams that want powerful dev environments without deep Nix knowledge.

### 3. [Flox](./flox/)
Simplified package management with Flox. Ideal for teams that want easy package management with a simpler interface than raw Nix.

## Running the Examples

Each example includes:
- Configuration files for the respective tool
- Moon configuration (`.moon/toolchain.yml` and `moon.yml`)
- Example tasks demonstrating the integration
- README with specific instructions

To run any example:

1. Navigate to the example directory
2. Follow the setup instructions in the example's README
3. Run moon tasks to see the integration in action

## How the Plugin Works

The moon Nix plugin automatically detects which Nix environment type you're using and wraps all moon task commands appropriately:

- **Flakes**: Commands wrapped with `nix develop --command`
- **Devenv**: Commands wrapped with `devenv shell --`
- **Flox**: Commands wrapped with `flox activate -e <env> --`

This ensures all tasks run within the configured Nix environment with access to all specified packages and environment variables.