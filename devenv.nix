{ pkgs, lib, config, inputs, ... }:
{
  packages = with pkgs; [ rustup ];
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "wasm32-wasip1" ];
  };
}
