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
        config_file_globs: vec!["devenv.yaml".into()],
        exe_names: vec!["nix".into(), "devenv".into()],
        lock_file_names: vec!["flake.lock".into(), "devenv.lock".into()],
        manifest_file_names: vec!["devenv.nix".into()],

        vendor_dir_name: Some(".devenv/profile/bin".into()),
        ..Default::default()
    }))
}

#[plugin_fn]
pub fn initialize_toolchain(
    Json(_): Json<InitializeToolchainInput>,
) -> FnResult<Json<InitializeToolchainOutput>> {
    Ok(Json(InitializeToolchainOutput {
        prompts: vec![SettingPrompt::new(
            "useDevenv",
            "Enable automatic detection and usage of devenv?",
            PromptType::Confirm { default: false },
        )],
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
