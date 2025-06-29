# Flox Example

This example demonstrates using moon with [Flox](https://flox.dev/) environments.

## Setup

1. Install Flox: https://flox.dev/docs/install
2. The `.flox/env.json` file defines the packages for this environment
3. Run moon tasks:

```bash
# Simple hello
moon run hello

# Check Node version from Flox
moon run node-version

# Check Deno version from Flox
moon run deno-version

# Check Rust version from Flox
moon run rust-version

# Use ripgrep from Flox
moon run search-example
```

## How it works

The moon Nix plugin automatically detects the `.flox` directory and wraps all moon task commands with `flox activate -e default -- <your-command>`. This ensures all tasks run within the Flox environment with access to all packages defined in the environment.

## Customizing the Environment

You can modify `.flox/env.json` to add or remove packages. The `floxEnvironment` setting in `.moon/toolchain.yml` can be changed to use different Flox environments (default is "default").