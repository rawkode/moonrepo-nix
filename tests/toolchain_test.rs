#[cfg(test)]
mod tests {
    use moon_pdk_test_utils::*;
    use starbase_sandbox::assert_snapshot;

    #[test]
    fn registers_toolchain() {
        let sandbox = create_empty_plugin_sandbox("nix");
        let plugin = sandbox.create_plugin_with_config("moon_toolchain_nix", |config| {
            config.host(HostOS::Linux, HostArch::X64);
        });

        let output = plugin.register_toolchain(RegisterToolchainInput::default());

        assert_eq!(output.name, "Nix");
        assert!(output.config_file_globs.contains(&"flake.nix".to_string()));
        assert!(output.config_file_globs.contains(&"shell.nix".to_string()));
        assert!(output.config_file_globs.contains(&"devenv.nix".to_string()));
        assert!(output.exe_names.contains(&"nix".to_string()));
        assert!(output.exe_names.contains(&"flox".to_string()));
        assert!(output.exe_names.contains(&"devenv".to_string()));
    }

    #[test]
    fn defines_config_schema() {
        let sandbox = create_empty_plugin_sandbox("nix");
        let plugin = sandbox.create_plugin_with_config("moon_toolchain_nix", |config| {
            config.host(HostOS::Linux, HostArch::X64);
        });

        let output = plugin.define_toolchain_config();

        assert!(!output.schema.is_empty());
    }

    #[test]
    fn initializes_toolchain() {
        let sandbox = create_empty_plugin_sandbox("nix");
        let plugin = sandbox.create_plugin_with_config("moon_toolchain_nix", |config| {
            config.host(HostOS::Linux, HostArch::X64);
        });

        let output = plugin.initialize_toolchain(InitializeToolchainInput::default());

        assert_eq!(output.prompts.len(), 4);
        assert_eq!(output.prompts[0].id, "useFlake");
        assert_eq!(output.prompts[1].id, "useShellNix");
        assert_eq!(output.prompts[2].id, "useFlox");
        assert_eq!(output.prompts[3].id, "useDevenv");
    }
}