use moon_config::BinEntry;
use moon_pdk_api::{UnresolvedVersionSpec, config_struct};
use schematic::Config;

config_struct!(
    /// Configures and enables the Nix toolchain.
    /// Docs: https://moonrepo.dev/docs/config/toolchain#nix
    #[derive(Config)]
    pub struct NixToolchainConfig {
        /// List of packages to automatically install using `nix-env`.
        pub packages: Vec<String>,

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

        /// List of binaries to install into the environment.
        #[setting(nested)]
        pub bins: Vec<BinEntry>,

        /// Configured version of Nix to use. Only applicable when using proto.
        pub version: Option<UnresolvedVersionSpec>,
    }
);