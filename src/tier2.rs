use crate::config::NixToolchainConfig;
use extism_pdk::*;
use moon_pdk::{get_host_environment, get_toolchain_config};
use moon_pdk_api::*;
use std::path::PathBuf;

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

    // Determine which Nix environment to set up and run initial setup commands
    enum NixEnv {
        ProjectFlake,
        ProjectDevenv,
        ProjectFlox,
        ProjectShellNix,
        WorkspaceFlake,
        WorkspaceDevenv,
        WorkspaceFlox,
        WorkspaceShellNix,
        None,
    }

    let nix_env = match () {
        // Check project-specific configurations first
        _ if config.use_flake && project_root.join("flake.nix").exists() => NixEnv::ProjectFlake,
        _ if config.use_devenv
            && (project_root.join("devenv.nix").exists()
                || project_root.join("devenv.yaml").exists()) =>
        {
            NixEnv::ProjectDevenv
        }
        _ if config.use_flox && project_root.join(".flox").exists() => NixEnv::ProjectFlox,
        _ if config.use_shell_nix && project_root.join("shell.nix").exists() => {
            NixEnv::ProjectShellNix
        }
        // Fall back to workspace-level configurations
        _ if config.use_flake && workspace_root.join("flake.nix").exists() => {
            NixEnv::WorkspaceFlake
        }
        _ if config.use_devenv
            && (workspace_root.join("devenv.nix").exists()
                || workspace_root.join("devenv.yaml").exists()) =>
        {
            NixEnv::WorkspaceDevenv
        }
        _ if config.use_flox && workspace_root.join(".flox").exists() => NixEnv::WorkspaceFlox,
        _ if config.use_shell_nix && workspace_root.join("shell.nix").exists() => {
            NixEnv::WorkspaceShellNix
        }
        _ => NixEnv::None,
    };

    // Add setup operations based on the environment type
    match nix_env {
        NixEnv::ProjectFlake | NixEnv::WorkspaceFlake => {
            let root = if matches!(nix_env, NixEnv::ProjectFlake) {
                &project_root
            } else {
                workspace_root
            };
            // For flakes, ensure flake.lock exists
            if !root.join("flake.lock").exists() {
                output.commands.push(ExecCommand {
                    command: ExecCommandInput::new("nix", ["flake", "lock"])
                        .cwd(root.clone()),
                    label: Some("Lock Nix flake".into()),
                    ..Default::default()
                });
            }
        }
        NixEnv::ProjectDevenv | NixEnv::WorkspaceDevenv => {
            let root = if matches!(nix_env, NixEnv::ProjectDevenv) {
                &project_root
            } else {
                workspace_root
            };
            // For devenv, run devenv info to initialize
            output.commands.push(ExecCommand {
                command: ExecCommandInput::new("devenv", ["info"]).cwd(root.clone()),
                label: Some("Initialize devenv".into()),
                ..Default::default()
            });
        }
        NixEnv::ProjectFlox | NixEnv::WorkspaceFlox => {
            let root = if matches!(nix_env, NixEnv::ProjectFlox) {
                &project_root
            } else {
                workspace_root
            };
            // For flox, initialize the environment
            output.commands.push(ExecCommand {
                command: ExecCommandInput::new("flox", ["install"]).cwd(root.clone()),
                label: Some("Initialize Flox environment".into()),
                ..Default::default()
            });
        }
        NixEnv::ProjectShellNix | NixEnv::WorkspaceShellNix => {
            // For shell.nix, no specific setup needed
        }
        NixEnv::None => {
            // No Nix environment found
        }
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn extend_task_command(
    Json(input): Json<ExtendTaskCommandInput>,
) -> FnResult<Json<ExtendTaskCommandOutput>> {
    let mut output = ExtendTaskCommandOutput::default();
    let config = get_toolchain_config::<NixToolchainConfig>()?;
    let env = get_host_environment()?;

    // Check for various Nix environment setups
    let workspace_root = &input.context.workspace_root;

    // Get the project directory from the task target
    let target_str = input.task.target.to_string();
    let project_id = target_str.split(':').next().unwrap_or("");
    let project_root = workspace_root.join(project_id);

    // Determine which Nix environment to use
    enum NixEnv {
        ProjectFlake,
        ProjectDevenv,
        ProjectFlox,
        ProjectShellNix,
        WorkspaceFlake,
        WorkspaceDevenv,
        WorkspaceFlox,
        WorkspaceShellNix,
        None,
    }

    let nix_env = match () {
        // Check project-specific configurations first
        _ if config.use_flake && project_root.join("flake.nix").exists() => NixEnv::ProjectFlake,
        _ if config.use_devenv
            && (project_root.join("devenv.nix").exists()
                || project_root.join("devenv.yaml").exists()) =>
        {
            NixEnv::ProjectDevenv
        }
        _ if config.use_flox && project_root.join(".flox").exists() => NixEnv::ProjectFlox,
        _ if config.use_shell_nix && project_root.join("shell.nix").exists() => {
            NixEnv::ProjectShellNix
        }
        // Fall back to workspace-level configurations
        _ if config.use_flake && workspace_root.join("flake.nix").exists() => {
            NixEnv::WorkspaceFlake
        }
        _ if config.use_devenv
            && (workspace_root.join("devenv.nix").exists()
                || workspace_root.join("devenv.yaml").exists()) =>
        {
            NixEnv::WorkspaceDevenv
        }
        _ if config.use_flox && workspace_root.join(".flox").exists() => NixEnv::WorkspaceFlox,
        _ if config.use_shell_nix && workspace_root.join("shell.nix").exists() => {
            NixEnv::WorkspaceShellNix
        }
        _ => NixEnv::None,
    };

    // Wrap command in appropriate Nix environment
    match nix_env {
        NixEnv::ProjectFlake => {
            output.command = Some("nix".into());
            output.args = Some(Extend::Prepend(vec![
                "develop".into(),
                project_root
                    .real_path()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| project_root.to_string()),
                "--command".into(),
                input.command.clone(),
            ]));
        }
        NixEnv::ProjectDevenv => {
            output.command = Some("devenv".into());
            output.args = Some(Extend::Prepend(vec![
                "shell".into(),
                "-C".into(),
                project_root
                    .real_path()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| project_root.to_string()),
                "--".into(),
                input.command.clone(),
            ]));
        }
        NixEnv::ProjectFlox => {
            output.command = Some("flox".into());
            output.args = Some(Extend::Prepend(vec![
                "activate".into(),
                "-d".into(),
                project_root
                    .real_path()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| project_root.to_string()),
                "--".into(),
                input.command.clone(),
            ]));
        }
        NixEnv::ProjectShellNix => {
            output.command = Some("nix-shell".into());
            output.args = Some(Extend::Replace(vec![
                project_root
                    .join("shell.nix")
                    .real_path()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| project_root.join("shell.nix").to_string()),
                "--run".into(),
                format!("{} {}", input.command, input.args.join(" ")),
            ]));
        }
        NixEnv::WorkspaceFlake => {
            output.command = Some("nix".into());
            output.args = Some(Extend::Prepend(vec![
                "develop".into(),
                "--command".into(),
                input.command.clone(),
            ]));
        }
        NixEnv::WorkspaceDevenv => {
            output.command = Some("devenv".into());
            output.args = Some(Extend::Prepend(vec![
                "shell".into(),
                "--".into(),
                input.command.clone(),
            ]));
        }
        NixEnv::WorkspaceFlox => {
            output.command = Some("flox".into());
            output.args = Some(Extend::Prepend(vec![
                "activate".into(),
                "--".into(),
                input.command.clone(),
            ]));
        }
        NixEnv::WorkspaceShellNix => {
            output.command = Some("nix-shell".into());
            output.args = Some(Extend::Replace(vec![
                "--run".into(),
                format!("{} {}", input.command, input.args.join(" ")),
            ]));
        }
        NixEnv::None => {}
    }

    // Add Nix paths to PATH
    let mut paths = vec![];
    if let Some(nix_profiles) = env.home_dir.join(".nix-profile/bin").real_path() {
        paths.push(nix_profiles);
    }
    if let Ok(path) = PathBuf::from("/nix/var/nix/profiles/default/bin").canonicalize() {
        paths.push(path);
    }

    if !paths.is_empty() {
        output.paths = paths;
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn install_dependencies(
    Json(input): Json<InstallDependenciesInput>,
) -> FnResult<Json<InstallDependenciesOutput>> {
    let mut output = InstallDependenciesOutput::default();
    let config = get_toolchain_config::<NixToolchainConfig>()?;

    // Determine which install command to use based on environment
    enum InstallType {
        FlakeLock,
        DevenvInfo,
        FloxInstall,
        None,
    }

    let install_type = match () {
        _ if config.use_flake
            && input.root.join("flake.nix").exists()
            && !input.root.join("flake.lock").exists() =>
        {
            InstallType::FlakeLock
        }
        _ if config.use_devenv
            && (input.root.join("devenv.nix").exists()
                || input.root.join("devenv.yaml").exists()) =>
        {
            InstallType::DevenvInfo
        }
        _ if config.use_flox && input.root.join(".flox").exists() => InstallType::FloxInstall,
        _ => InstallType::None,
    };

    output.install_command = match install_type {
        InstallType::FlakeLock => Some(
            ExecCommandInput::new("nix", ["flake", "lock"])
                .cwd(input.root.clone())
                .into(),
        ),
        InstallType::DevenvInfo => Some(
            ExecCommandInput::new("devenv", ["info"])
                .cwd(input.root.clone())
                .into(),
        ),
        InstallType::FloxInstall => Some(
            ExecCommandInput::new("flox", ["install"])
                .cwd(input.root.clone())
                .into(),
        ),
        InstallType::None => None,
    };

    Ok(Json(output))
}
