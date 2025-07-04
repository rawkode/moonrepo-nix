use moon_pdk_api::{config_struct, UnresolvedVersionSpec};
use schematic::Config;

config_struct!(
    #[derive(Config)]
    pub struct NixToolchainConfig {
        /// When enabled, automatically detects and uses devenv configuration.
        pub use_devenv: bool,

        /// Configured version of Nix to use. Only applicable when using proto.
        pub version: Option<UnresolvedVersionSpec>,
    }
);
