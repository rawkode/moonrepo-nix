use crate::config::NixToolchainConfig;
use extism_pdk::*;
use moon_pdk::parse_toolchain_config;
use moon_pdk_api::*;

#[plugin_fn]
pub fn setup_environment(
    Json(input): Json<SetupEnvironmentInput>,
) -> FnResult<Json<SetupEnvironmentOutput>> {
    let mut output = SetupEnvironmentOutput::default();
    let config = parse_toolchain_config::<NixToolchainConfig>(input.toolchain_config)?;

    let workspace_root = &input.context.workspace_root;

    // Default to the workspace root if no project is specified
    let project_root = match &input.project {
        Some(project) => workspace_root.join(&project.source),
        None => workspace_root.clone(),
    };

    enum NixEnv {
        Devenv,
        None,
    }

    let nix_env = match () {
        _ if config.use_devenv
            && (project_root.join("devenv.nix").exists()
                || project_root.join("devenv.yaml").exists()) =>
        {
            NixEnv::Devenv
        }
        _ => NixEnv::None,
    };

    match nix_env {
        NixEnv::Devenv => {
            output.commands.push(ExecCommand {
                command: ExecCommandInput::new(
                    "devenv",
                    ["shell", "echo", "Devenv environment prepared"],
                )
                .cwd(project_root.clone()),
                label: Some("Initialize devenv".into()),
                ..Default::default()
            });
        }
        NixEnv::None => {}
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
        DevenvInfo,
        None,
    }

    let install_type = match () {
        _ if config.use_devenv
            && (input.root.join("devenv.nix").exists()
                || input.root.join("devenv.yaml").exists()) =>
        {
            InstallType::DevenvInfo
        }
        _ => InstallType::None,
    };

    output.install_command = match install_type {
        InstallType::DevenvInfo => Some(
            ExecCommandInput::new("devenv", ["info"])
                .cwd(input.root.clone())
                .into(),
        ),
        InstallType::None => None,
    };

    Ok(Json(output))
}
