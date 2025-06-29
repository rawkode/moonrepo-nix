use crate::config::NixToolchainConfig;
use extism_pdk::*;
use moon_pdk::parse_toolchain_config;
use moon_pdk_api::*;
// use rustc_hash::FxHashMap; // Removed as generate_lockfile_hash is no longer part of the API

#[plugin_fn]
pub fn sync_workspace(Json(input): Json<SyncWorkspaceInput>) -> FnResult<Json<SyncOutput>> {
    let mut output = SyncOutput::default();
    let config = parse_toolchain_config::<NixToolchainConfig>(input.toolchain_config)?;

    // Check if we need to create any initial Nix configuration
    let workspace_root = &input.context.workspace_root;

    // If no Nix files exist and config has packages, create a basic shell.nix
    if !workspace_root.join("flake.nix").exists()
        && !workspace_root.join("shell.nix").exists()
        && !workspace_root.join("devenv.nix").exists()
        && !workspace_root.join(".flox").exists()
        && !config.packages.is_empty()
    {
        let shell_nix = format!(
            r#"{{ pkgs ? import <nixpkgs> {{}} }}:
pkgs.mkShell {{
  buildInputs = with pkgs; [
    {}
  ];
}}"#,
            config.packages.join("\n    ")
        );

        // Write shell.nix file
        let shell_nix_path = workspace_root.join("shell.nix");
        if let Some(real_path) = shell_nix_path.real_path() {
            std::fs::write(&real_path, shell_nix)?;
            output.changed_files.push(real_path);
        }
    }

    Ok(Json(output))
}

// The generate_lockfile_hash function is no longer part of the moon_pdk_api
// It has been removed in newer versions of the API
