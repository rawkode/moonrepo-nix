{
  description = "Example Nix flake for moon project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            nodejs_20
            nodePackages.pnpm
            python3
            rustc
            cargo
          ];

          shellHook = ''
            echo "Welcome to the moon flake example!"
            echo "Available tools: node, pnpm, python3, rustc"
          '';
        };
      });
}