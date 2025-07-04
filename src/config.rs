use moon_pdk_api::{config_struct, UnresolvedVersionSpec};
use schematic::Config;

config_struct!(
    #[derive(Config)]
    pub struct NixToolchainConfig {
        /// When enabled, automatically detects and uses flake.nix for environment setup.
        pub use_flake: bool,

        /// When enabled, automatically detects and uses shell.nix for environment setup.
        pub use_shell_nix: bool,

        /// When enabled, automatically detects and uses Flox environment.
        pub use_flox: bool,

        /// When enabled, automatically detects and uses devenv configuration.
        pub use_devenv: bool,

        /// Flox environment name to activate (defaults to "default").
        pub flox_environment: Option<String>,

        /// Configured version of Nix to use. Only applicable when using proto.
        pub version: Option<UnresolvedVersionSpec>,
    }
);
