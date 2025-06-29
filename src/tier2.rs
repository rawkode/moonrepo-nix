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
    
    // Determine which Nix environment to use
    enum NixEnv {
        Flake,
        Devenv,
        Flox,
        ShellNix,
        None,
    }
    
    let nix_env = match () {
        _ if config.use_flake && workspace_root.join("flake.nix").exists() => NixEnv::Flake,
        _ if config.use_devenv && (workspace_root.join("devenv.nix").exists() || workspace_root.join("devenv.yaml").exists()) => NixEnv::Devenv,
        _ if config.use_flox && workspace_root.join(".flox").exists() => NixEnv::Flox,
        _ if config.use_shell_nix && workspace_root.join("shell.nix").exists() => NixEnv::ShellNix,
        _ => NixEnv::None,
    };
    
    // Wrap command in appropriate Nix environment
    match nix_env {
        NixEnv::Flake => {
            output.command = Some("nix".into());
            output.args = Some(Extend::Prepend(vec![
                "develop".into(),
                "--command".into(),
                input.command.clone(),
            ]));
        }
        NixEnv::Devenv => {
            output.command = Some("devenv".into());
            output.args = Some(Extend::Prepend(vec![
                "shell".into(),
                "--".into(),
                input.command.clone(),
            ]));
        }
        NixEnv::Flox => {
            output.command = Some("flox".into());
            output.args = Some(Extend::Prepend(vec![
                "activate".into(),
                "--".into(),
                input.command.clone(),
            ]));
        }
        NixEnv::ShellNix => {
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
pub fn extend_task_script(
    Json(_input): Json<ExtendTaskScriptInput>,
) -> FnResult<Json<ExtendTaskScriptOutput>> {
    let mut output = ExtendTaskScriptOutput::default();
    let env = get_host_environment()?;
    
    // Add Nix paths for scripts
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
        _ if config.use_flake && input.root.join("flake.nix").exists() && !input.root.join("flake.lock").exists() => {
            InstallType::FlakeLock
        }
        _ if config.use_devenv && (input.root.join("devenv.nix").exists() || input.root.join("devenv.yaml").exists()) => {
            InstallType::DevenvInfo
        }
        _ if !config.packages.is_empty() => {
            InstallType::NixEnvPackages(config.packages.clone())
        }
        _ => InstallType::None,
    };
    
    output.install_command = match install_type {
        InstallType::FlakeLock => {
            Some(
                ExecCommandInput::new("nix", ["flake", "lock"])
                    .cwd(input.root.clone())
                    .into(),
            )
        }
        InstallType::DevenvInfo => {
            Some(
                ExecCommandInput::new("devenv", ["info"])
                    .cwd(input.root.clone())
                    .into(),
            )
        }
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