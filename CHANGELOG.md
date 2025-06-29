# Changelog

## Unreleased

#### 🎉 Release

- Initial release of the Nix toolchain plugin!

#### 🚀 Updates

- Added support for Nix Flakes with automatic `flake.nix` detection
- Added support for Flox environments with `.flox` directory detection
- Added support for devenv with `devenv.nix` and `devenv.yaml` detection
- Added support for traditional `shell.nix` environments
- Added automatic command wrapping for different Nix environment types
- Added lockfile generation and hashing for reproducible builds