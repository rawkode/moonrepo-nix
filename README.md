# moon Nix Toolchain Plugin

A [moon](https://moonrepo.dev) plugin for the Nix toolchain, supporting Nix Flakes, Flox, and devenv.

## Features

- 🚀 **Nix Flakes** - Automatic detection and usage of `flake.nix` files
- 📦 **Flox** - Support for Flox environments (`.flox` directory)
- 🛠️ **devenv** - Integration with devenv (`devenv.nix` and `devenv.yaml`)
- 🐚 **shell.nix** - Traditional Nix shell environment support
- 🔒 **Reproducible builds** - Lockfile tracking and hashing
- ⚡ **Automatic wrapping** - Commands are automatically wrapped in the appropriate Nix environment

## Installation

Add the plugin to your `.moon/toolchain.yml`:

```yaml
nix:
  plugin: "https://github.com/yourusername/moon-toolchain-nix/releases/download/v0.1.0/moon_toolchain_nix.wasm"
```

## Configuration

Configure the Nix toolchain in your `.moon/toolchain.yml`:

```yaml
nix:
  # Enable automatic detection and usage of flake.nix
  useFlake: true
  
  # Enable automatic detection and usage of shell.nix
  useShellNix: false
  
  # Enable automatic detection and usage of Flox environments
  useFlox: false
  
  # Enable automatic detection and usage of devenv
  useDevenv: false
  
  # Flox environment name (defaults to "default")
  floxEnvironment: "default"
  
  # List of Nix packages to install
  packages:
    - nodejs
    - python3
    - rustc
```

## Usage

Once configured, moon will automatically:

1. **Detect your Nix environment** - Based on the presence of `flake.nix`, `shell.nix`, `.flox`, or `devenv.nix`
2. **Wrap commands** - All task commands will be wrapped in the appropriate Nix environment
3. **Track lockfiles** - Changes to `flake.lock`, `devenv.lock`, or Flox environments are tracked
4. **Generate lockfiles** - Missing lockfiles are automatically generated when running `moon install-deps`

### Example with Nix Flakes

Create a `flake.nix` in your workspace root:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
    devShells.default = nixpkgs.legacyPackages.x86_64-linux.mkShell {
      packages = with nixpkgs.legacyPackages.x86_64-linux; [
        nodejs
        yarn
      ];
    };
  };
}
```

Enable flakes in your toolchain:

```yaml
nix:
  useFlake: true
```

Now all moon tasks will run inside the Nix flake environment!

### Example with devenv

Create a `devenv.nix`:

```nix
{ pkgs, ... }:

{
  packages = [ pkgs.git ];

  languages.javascript.enable = true;
  languages.rust.enable = true;
}
```

Enable devenv in your toolchain:

```yaml
nix:
  useDevenv: true
```

### Example with Flox

Initialize a Flox environment:

```bash
flox init
flox install nodejs yarn
```

Enable Flox in your toolchain:

```yaml
nix:
  useFlox: true
  floxEnvironment: "default"
```

## Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/moon-toolchain-nix
cd moon-toolchain-nix

# Build the plugin
cargo build --release --target wasm32-wasi

# The plugin will be at target/wasm32-wasi/release/moon_toolchain_nix.wasm
```

## License

MIT