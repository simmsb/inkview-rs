{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [
    pkgs.git 
    pkgs.git-cliff
    pkgs.just
    pkgs.secretspec
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  env.GITHUB_TOKEN = config.secretspec.secrets.GITHUB_TOKEN or "";
}
