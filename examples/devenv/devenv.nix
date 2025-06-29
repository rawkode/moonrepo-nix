{ pkgs, ... }:

{
  # Package dependencies
  packages = with pkgs; [
    git
    nodejs_20
    nodePackages.yarn
    go_1_21
  ];

  # Environment variables
  env = {
    DEVENV_EXAMPLE = "true";
    GOPATH = "$DEVENV_ROOT/.go";
  };

  # Scripts
  scripts.hello.exec = ''
    echo "Hello from devenv!"
    echo "Node version: $(node --version)"
    echo "Go version: $(go version)"
  '';

  # Pre-commit hooks
  pre-commit.hooks = {
    prettier.enable = true;
    gofmt.enable = true;
  };

  # Process management
  processes = {
    docs.exec = "echo 'Documentation server would run here'";
  };

  enterShell = ''
    echo "Welcome to the devenv example!"
    echo "Run 'hello' to see available tools"
  '';
}