use crate::config::NixToolchainConfig;
use extism_pdk::*;
use moon_pdk::{parse_toolchain_config, is_project_toolchain_enabled};
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
            "nix-env".into(),
            "nix-build".into(),
            "nix-store".into(),
            "nix-instantiate".into(),
            "flox".into(),
            "devenv".into(),
        ],
        lock_file_names: vec![
            "flake.lock".into(),
            "devenv.lock".into(),
        ],
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
        config_url: Some("https://moonrepo.dev/docs/guides/nix/handbook".into()),
        docs_url: Some("https://moonrepo.dev/docs/config/toolchain#nix".into()),
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
pub fn define_docker_metadata(
    Json(input): Json<DefineDockerMetadataInput>,
) -> FnResult<Json<DefineDockerMetadataOutput>> {
    let config = parse_toolchain_config::<NixToolchainConfig>(input.toolchain_config)?;

    Ok(Json(DefineDockerMetadataOutput {
        default_image: Some(format!(
            "nixos/nix:{}",
            config
                .version
                .as_ref()
                .map(|version| version.to_string())
                .unwrap_or_else(|| "latest".into())
        )),
        ..Default::default()
    }))
}

#[plugin_fn]
pub fn sync_project(Json(input): Json<SyncProjectInput>) -> FnResult<Json<SyncOutput>> {
    let mut output = SyncOutput::default();
    
    // Check if Nix toolchain is enabled for this project
    if !is_project_toolchain_enabled(&input.project) {
        output.skipped = true;
        return Ok(Json(output));
    }

    let config = parse_toolchain_config::<NixToolchainConfig>(input.toolchain_config)?;
    
    // In a monorepo, each project may have its own Nix configuration
    let project_root = input.context.workspace_root.join(&input.project.source);
    
    // Check for project-specific Nix configurations
    struct NixConfigs {
        flake: bool,
        shell_nix: bool,
        devenv: bool,
        flox: bool,
    }
    
    let configs = NixConfigs {
        flake: config.use_flake && project_root.join("flake.nix").exists(),
        shell_nix: config.use_shell_nix && project_root.join("shell.nix").exists(),
        devenv: config.use_devenv && (project_root.join("devenv.nix").exists() || project_root.join("devenv.yaml").exists()),
        flox: config.use_flox && (project_root.join(".flox/env.json").exists() || project_root.join(".flox/env.toml").exists()),
    };
    
    // For now, we just track which Nix environments are available
    // In the future, we could create operations to sync lock files or update configurations
    let has_project_config = configs.flake || configs.shell_nix || configs.devenv || configs.flox;
    
    if has_project_config {
        // Project has its own Nix configuration
        // Could add operations here in the future to sync lock files, etc.
    }
    
    Ok(Json(output))
}