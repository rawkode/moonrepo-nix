use crate::config::NixToolchainConfig;
use extism_pdk::*;
use moon_pdk::{get_host_environment, get_toolchain_config, parse_toolchain_config};
use moon_pdk_api::*;
use std::path::PathBuf;

#[plugin_fn]
pub fn extend_task_command(
    Json(input): Json<ExtendTaskCommandInput>,
) -> FnResult<Json<ExtendTaskCommandOutput>> {
    let mut output = ExtendTaskCommandOutput::default();
    // Get the toolchain config from the workspace
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
            let mut args = vec!["develop".into(), "--command".into(), input.command.clone()];
            args.extend(input.args.clone());
            
            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::ProjectDevenv => {
            output.command = Some("devenv".into());
            let mut args = vec!["shell".into(), "--".into(), input.command.clone()];
            args.extend(input.args.clone());

            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::ProjectFlox => {
            output.command = Some("flox".into());
            let mut args = vec!["activate".into(), "--".into(), input.command.clone()];
            args.extend(input.args.clone());
            
            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::ProjectShellNix => {
            output.command = Some("nix-shell".into());
            output.args = Some(Extend::Replace(vec![
                "--command".into(),
                format!("{} {}", input.command, input.args.join(" ")),
            ]));
        }
        NixEnv::WorkspaceFlake => {
            output.command = Some("nix".into());
            let mut args = vec!["develop".into(), "--command".into(), input.command.clone()];
            args.extend(input.args.clone());
            
            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::WorkspaceDevenv => {
            output.command = Some("devenv".into());
            let mut args = vec!["shell".into(), "--".into(), input.command.clone()];
            args.extend(input.args.clone());
            
            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::WorkspaceFlox => {
            output.command = Some("flox".into());
            let mut args = vec!["activate".into(), "--".into(), input.command.clone()];
            args.extend(input.args.clone());
            
            output.args = Some(Extend::Prepend(args));
        }
        NixEnv::WorkspaceShellNix => {
            output.command = Some("nix-shell".into());
            output.args = Some(Extend::Replace(vec![
                "--command".into(),
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
pub fn extend_task_script(
    Json(input): Json<ExtendTaskScriptInput>,
) -> FnResult<Json<ExtendTaskScriptOutput>> {
    let mut output = ExtendTaskScriptOutput::default();
    let env = get_host_environment()?;

    // Get the project ID from the task target
    let target_str = input.task.target.to_string();
    let project_id = target_str.split(':').next().unwrap_or("workspace");

    // Try to get the stored PATH for this project
    let path_key = format!("nix_path_{}", project_id);
    let type_key = format!("nix_type_{}", project_id);

    // Check if we have a stored PATH for this project
    if let Ok(Some(nix_path)) = var::get::<String>(&path_key) {
        // Use the captured PATH from the Nix environment
        output.env.insert("PATH".to_string(), nix_path);

        // Optionally, add environment type info
        if let Ok(Some(env_type)) = var::get::<String>(&type_key) {
            output.env.insert("NIX_ENV_TYPE".to_string(), env_type);
        }
    } else if project_id != "workspace" {
        // If project-specific PATH not found, try workspace PATH
        if let Ok(Some(nix_path)) = var::get::<String>("nix_path_workspace") {
            output.env.insert("PATH".to_string(), nix_path);
        }
    } else {
        // Fallback to adding basic Nix paths
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
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn locate_dependencies_root(
    Json(input): Json<LocateDependenciesRootInput>,
) -> FnResult<Json<LocateDependenciesRootOutput>> {
    let mut output = LocateDependenciesRootOutput::default();
    let config = parse_toolchain_config::<NixToolchainConfig>(input.toolchain_config)?;

    // Priority order for locating dependencies
    let files_to_check: Vec<(&str, bool)> = vec![
        ("flake.lock", config.use_flake),
        ("flake.nix", config.use_flake),
        ("devenv.lock", config.use_devenv),
        ("devenv.nix", config.use_devenv),
        ("devenv.yaml", config.use_devenv),
        (".flox", config.use_flox),
        ("shell.nix", config.use_shell_nix),
        ("default.nix", true), // Always check as fallback
    ];

    // Find the first matching file
    for (file, enabled) in files_to_check {
        match (enabled, moon_pdk::locate_root(&input.starting_dir, file)) {
            (true, Some(root)) => {
                output.root = root.virtual_path();
                return Ok(Json(output));
            }
            _ => continue,
        }
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn install_dependencies(
    Json(input): Json<InstallDependenciesInput>,
) -> FnResult<Json<InstallDependenciesOutput>> {
    let mut output = InstallDependenciesOutput::default();
    let config = parse_toolchain_config::<NixToolchainConfig>(input.toolchain_config)?;

    // Determine which install command to use based on environment
    enum InstallType {
        FlakeLock,
        DevenvInfo,
        NixEnvPackages(Vec<String>),
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
        _ if !config.packages.is_empty() => InstallType::NixEnvPackages(config.packages.clone()),
        _ => InstallType::None,
    };

    output.install_command = match install_type {
        InstallType::FlakeLock => Some(
            ExecCommandInput::new("nix", ["flake", "lock"])
                .cwd(input.root.clone())
                .into(),
        ),
        InstallType::DevenvInfo => Some(
            ExecCommandInput::new("devenv", ["shell", "echo", "installing dependencies"])
                .cwd(input.root.clone())
                .into(),
        ),
        InstallType::NixEnvPackages(packages) => {
            let mut args = vec!["--install".into()];
            args.extend(packages);
            Some(
                ExecCommandInput::new("nix-env", args)
                    .cwd(input.root.clone())
                    .into(),
            )
        }
        InstallType::None => None,
    };

    Ok(Json(output))
}

#[plugin_fn]
pub fn setup_environment(
    Json(input): Json<SetupEnvironmentInput>,
) -> FnResult<Json<SetupEnvironmentOutput>> {
    let mut output = SetupEnvironmentOutput::default();
    let config = parse_toolchain_config::<NixToolchainConfig>(input.toolchain_config)?;

    // Check for Nix environments and prepare them
    let workspace_root = &input.context.workspace_root;

    // Determine which Nix environment to set up
    enum NixEnv {
        Flake,
        Devenv,
        Flox,
        ShellNix,
        None,
    }

    let nix_env = match () {
        _ if config.use_flake && workspace_root.join("flake.nix").exists() => NixEnv::Flake,
        _ if config.use_devenv
            && (workspace_root.join("devenv.nix").exists()
                || workspace_root.join("devenv.yaml").exists()) =>
        {
            NixEnv::Devenv
        }
        _ if config.use_flox && workspace_root.join(".flox").exists() => NixEnv::Flox,
        _ if config.use_shell_nix && workspace_root.join("shell.nix").exists() => NixEnv::ShellNix,
        _ => NixEnv::None,
    };

    // Add setup operations based on the environment type
    match nix_env {
        NixEnv::Flake => {
            // For flakes, we might want to ensure flake.lock exists
            if !workspace_root.join("flake.lock").exists() {
                output.commands.push(ExecCommand {
                    command: ExecCommandInput::new("nix", ["flake", "lock"])
                        .cwd(workspace_root.clone()),
                    label: Some("Lock Nix flake".into()),
                    ..Default::default()
                });
            }
        }
        NixEnv::Devenv => {
            // For devenv, run devenv info to initialize
            output.commands.push(ExecCommand {
                command: ExecCommandInput::new("devenv", ["info"]).cwd(workspace_root.clone()),
                label: Some("Initialize devenv".into()),
                ..Default::default()
            });
        }
        NixEnv::Flox => {
            // For flox, we might want to initialize the environment
            output.commands.push(ExecCommand {
                command: ExecCommandInput::new("flox", ["install"]).cwd(workspace_root.clone()),
                label: Some("Initialize Flox environment".into()),
                ..Default::default()
            });
        }
        NixEnv::ShellNix => {
            // For shell.nix, no specific setup needed
        }
        NixEnv::None => {
            // No Nix environment found
        }
    }

    Ok(Json(output))
}
