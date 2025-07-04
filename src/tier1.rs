use crate::config::NixToolchainConfig;
use extism_pdk::*;
use moon_pdk_api::*;
use schematic::SchemaBuilder;

#[plugin_fn]
pub fn register_toolchain(
    Json(_): Json<RegisterToolchainInput>,
) -> FnResult<Json<RegisterToolchainOutput>> {
    Ok(Json(RegisterToolchainOutput {
        name: "Nix".into(),
        plugin_version: env!("CARGO_PKG_VERSION").into(),
        config_file_globs: vec![
            "flake.nix".into(),
            "flake.lock".into(),
            "shell.nix".into(),
            "default.nix".into(),
            ".envrc".into(),
            "devenv.nix".into(),
            "devenv.lock".into(),
            "devenv.yaml".into(),
            ".flox/env.json".into(),
            ".flox/env.toml".into(),
        ],
        exe_names: vec![
            "nix".into(),
            "nix-shell".into(),
            "devenv".into(),
            "flox".into(),
        ],
        lock_file_names: vec!["flake.lock".into(), "devenv.lock".into()],
        manifest_file_names: vec![
            "flake.nix".into(),
            "shell.nix".into(),
            "devenv.nix".into(),
            "devenv.yaml".into(),
        ],
        vendor_dir_name: Some(".direnv".into()),
        ..Default::default()
    }))
}

#[plugin_fn]
pub fn initialize_toolchain(
    Json(_): Json<InitializeToolchainInput>,
) -> FnResult<Json<InitializeToolchainOutput>> {
    Ok(Json(InitializeToolchainOutput {
        prompts: vec![
            SettingPrompt::new(
                "useFlake",
                "Enable automatic detection and usage of <file>flake.nix</file>?",
                PromptType::Confirm { default: true },
            ),
            SettingPrompt::new(
                "useShellNix",
                "Enable automatic detection and usage of <file>shell.nix</file>?",
                PromptType::Confirm { default: false },
            ),
            SettingPrompt::new(
                "useFlox",
                "Enable automatic detection and usage of Flox environments?",
                PromptType::Confirm { default: false },
            ),
            SettingPrompt::new(
                "useDevenv",
                "Enable automatic detection and usage of devenv?",
                PromptType::Confirm { default: false },
            ),
        ],
        ..Default::default()
    }))
}

#[plugin_fn]
pub fn define_toolchain_config() -> FnResult<Json<DefineToolchainConfigOutput>> {
    Ok(Json(DefineToolchainConfigOutput {
        schema: SchemaBuilder::build_root::<NixToolchainConfig>(),
    }))
}

#[plugin_fn]
pub fn parse_manifest(
    Json(_input): Json<ParseManifestInput>,
) -> FnResult<Json<ParseManifestOutput>> {
    let output = ParseManifestOutput::default();
    Ok(Json(output))
}
