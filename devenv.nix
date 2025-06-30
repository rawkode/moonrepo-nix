{ pkgs, lib, config, inputs, ... }:
{
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "wasm32-wasip1" ];
  };
}
