# Devenv Example

This example demonstrates using moon with [devenv](https://devenv.sh/) for development environments.

## Setup

1. Install devenv: https://devenv.sh/getting-started/
2. Run `devenv shell` once to initialize the environment
3. Run moon tasks:

```bash
# Run the devenv hello script
moon run hello

# Check Node version from devenv
moon run node-version

# Check Go version from devenv
moon run go-version

# Verify environment variables
moon run env-check
```

## How it works

The moon Nix plugin automatically detects the `devenv.nix` or `devenv.yaml` file and wraps all moon task commands with `devenv shell -- <your-command>`. This ensures all tasks run within the devenv environment with access to:

- All packages defined in devenv.nix
- Environment variables
- Custom scripts
- Pre-commit hooks
- And more devenv features