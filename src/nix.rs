use crate::config::NixToolchainConfig;
use extism_pdk::*;
use moon_pdk::get_toolchain_config;
use moon_pdk_api::*;
use schematic::SchemaBuilder;

enum NixEnv {
    Devenv,
    Flox,
    NixFlake,
    NixShell,
    None,
}

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
        vendor_dir_name: Some(".devenv/profile/bin".into()),
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

#[plugin_fn]
pub fn locate_dependencies_root(
    Json(input): Json<LocateDependenciesRootInput>,
) -> FnResult<Json<LocateDependenciesRootOutput>> {
    let config = get_toolchain_config::<NixToolchainConfig>()?;
    let starting_dir = &input.starting_dir;

    // Check for Nix environment files in the starting directory
    let nix_env = match () {
        _ if config.use_devenv
            && (starting_dir.join("devenv.nix").exists()
                || starting_dir.join("devenv.yaml").exists()) =>
        {
            NixEnv::Devenv
        }
        _ if config.use_flake && starting_dir.join("flake.nix").exists() => NixEnv::NixFlake,
        _ if config.use_flox && starting_dir.join(".flox").exists() => NixEnv::Flox,
        _ if config.use_shell_nix && starting_dir.join("shell.nix").exists() => NixEnv::NixShell,
        _ => NixEnv::None,
    };

    match nix_env {
        NixEnv::None => Ok(Json(LocateDependenciesRootOutput::default())),
        _ => Ok(Json(LocateDependenciesRootOutput {
            root: Some(starting_dir.to_path_buf()),
            members: None,
        })),
    }
}

#[plugin_fn]
pub fn install_dependencies(
    Json(_input): Json<InstallDependenciesInput>,
) -> FnResult<Json<InstallDependenciesOutput>> {
    // Nix manages dependencies through the shell environment,
    // so we don't need to run any explicit install commands here.
    // The actual environment setup happens in setup_environment.
    Ok(Json(InstallDependenciesOutput::default()))
}

#[plugin_fn]
pub fn setup_environment(
    Json(input): Json<SetupEnvironmentInput>,
) -> FnResult<Json<SetupEnvironmentOutput>> {
    let mut output = SetupEnvironmentOutput::default();
    let config = get_toolchain_config::<NixToolchainConfig>()?;

    let workspace_root = &input.context.workspace_root;

    // Default to the workspace root if no project is specified
    let project_root = match &input.project {
        Some(project) => workspace_root.join(&project.source),
        None => workspace_root.clone(),
    };

    let nix_env = match () {
        _ if config.use_devenv
            && (project_root.join("devenv.nix").exists()
                || project_root.join("devenv.yaml").exists()) =>
        {
            NixEnv::Devenv
        }
        _ if config.use_flake && project_root.join("flake.nix").exists() => NixEnv::NixFlake,
        _ if config.use_flox && project_root.join(".flox").exists() => NixEnv::Flox,
        _ if config.use_shell_nix && project_root.join("shell.nix").exists() => NixEnv::NixShell,
        _ => NixEnv::None,
    };

    match nix_env {
        NixEnv::NixFlake => {
            if !project_root.join("flake.lock").exists() {
                output.commands.push(ExecCommand {
                    command: ExecCommandInput::new("nix", ["develop", "--command", "pwd"])
                        .cwd(project_root.clone()),
                    label: Some("Lock Nix flake".into()),
                    ..Default::default()
                });
            }
        }
        NixEnv::Devenv => {
            output.commands.push(ExecCommand {
                command: ExecCommandInput::new("devenv", ["shell", "pwd"])
                    .cwd(project_root.clone()),
                label: Some("Install devenv dependencies".into()),
                ..Default::default()
            });
        }
        NixEnv::Flox => {
            output.commands.push(ExecCommand {
                command: ExecCommandInput::new("flox", ["activate", "--", "pwd"])
                    .cwd(project_root.clone()),
                label: Some("Initialize Flox environment".into()),
                ..Default::default()
            });
        }
        NixEnv::NixShell => {
            output.commands.push(ExecCommand {
                command: ExecCommandInput::new("nix-shell", ["--run", "pwd"])
                    .cwd(project_root.clone()),
                label: Some("Run Nix shell".into()),
                ..Default::default()
            });
        }
        NixEnv::None => {}
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn extend_task_command(
    Json(input): Json<ExtendTaskCommandInput>,
) -> FnResult<Json<ExtendTaskCommandOutput>> {
    let mut output = ExtendTaskCommandOutput::default();
    let config = get_toolchain_config::<NixToolchainConfig>()?;

    // Check for various Nix environment setups
    let workspace_root = &input.context.workspace_root;

    // Get the project directory from the task target
    let target_str = input.task.target.to_string();
    let project_id = target_str.split(':').next().unwrap_or("");
    let project_root = workspace_root.join(project_id);

    let nix_env = match () {
        _ if config.use_flake && project_root.join("flake.nix").exists() => NixEnv::NixFlake,
        _ if config.use_devenv
            && (project_root.join("devenv.nix").exists()
                || project_root.join("devenv.yaml").exists()) =>
        {
            NixEnv::Devenv
        }
        _ if config.use_flox && project_root.join(".flox").exists() => NixEnv::Flox,
        _ if config.use_shell_nix && project_root.join("shell.nix").exists() => NixEnv::NixShell,
        _ => NixEnv::None,
    };

    match nix_env {
        NixEnv::Devenv => {
            output.command = Some("devenv".into());

            let mut args = vec!["shell".into(), "--".into(), input.command.clone()];
            args.extend_from_slice(&input.args);

            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::NixFlake => {
            output.command = Some("nix".into());
            let mut args = vec!["develop".into(), "--command".into(), input.command.clone()];
            args.extend_from_slice(&input.args);
            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::Flox => {
            output.command = Some("flox".into());
            let mut args = vec!["activate".into(), "--".into(), input.command.clone()];
            args.extend_from_slice(&input.args);
            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::NixShell => {
            output.command = Some("nix-shell".into());
            let mut args = vec!["nix-shell".into(), "--run".into(), input.command.clone()];
            args.extend_from_slice(&input.args);
            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::None => {}
    }

    Ok(Json(output))
}
