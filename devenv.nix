{ pkgs, lib, config, inputs, ... }:

let
  # Target package set for the armv7 PocketBook runtime environment
  pkgsCross = pkgs.pkgsCross.armv7l-hf-multiplatform;
in
{
  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.git-cliff
    pkgs.just
    pkgs.secretspec

    # Native cross-compilation toolchain
    pkgs.zig
    pkgs.cargo-zigbuild

    # Use the target-specific cross-pkg-config tool
    pkgsCross.buildPackages.pkg-config
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "armv7-unknown-linux-gnueabi" ];
  };

  env = {
    PKG_CONFIG_ALLOW_CROSS = "1";

    # Provide the combined search paths for fontconfig and its system dependencies
    PKG_CONFIG_PATH_armv7_unknown_linux_gnueabi = lib.makeSearchPath "lib/pkgconfig" [
      pkgsCross.fontconfig.dev
      pkgsCross.freetype.dev
      pkgsCross.expat.dev
      pkgsCross.libpng.dev
      pkgsCross.zlib.dev
    ];
  };
}
