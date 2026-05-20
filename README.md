<h1 align="center">Inkview-rs</h1>

<h2 align="center">Rust bindings for pocketbook E-Reader devices (inkview)</h2>

<p align="center">
  <a href="https://crates.io/crates/inkview"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/inkview"></a>
  <a href="https://docs.rs/inkview/"><img alt="docs.rs" src="https://img.shields.io/docsrs/inkview"></a>
</p>


This repo contains bindings for `libinkview`, which is used by pocketbook devices.

## Overview

- `inkview` is the core of the project.
It dynamically loads `libinkview` rather than linking it statically, so that users of this
crate woudln't have to setup the pocketbook SDK during the build.
Instead, one needs to cross-compile to `armv7-unknown-linux-gnueabi.2.23`
(`cargo zigbuild` works well for this) and the binary will run on a pocketbook E-Reader.

- `inkview-eg` is an [embedded-graphics-core](https://crates.io/crates/embedded-graphics-core) driver for `inkview-rs`.

- `inkview-slint` provides a backend for the [slint](https://github.com/slint-ui/slint)
crate that works with `inkview`, with a respective demo project under `examples/inkview-slint-demo`.

The subprojects may contain examples that lay under `examples/` subfolders (e.g. `inkview/examples`)

## Prerequisites (Set-up)

1. [Zig](https://ziglang.org/learn/getting-started/#installing-zig) must be installed.
1. [just](https://github.com/casey/just) command runner must be installed.
    The build and deployment helper commands are defined in the [justfile](./justfile).
    Available recipes can be listed with:
    ```bash
    just --list
    ```
1. To set-up a reproducible sandbox build environment for the target `armv7-unknown-linux-gnueabi.2.23`
one must install [NIX](https://nixos.org/download/) +
[direnv](https://direnv.net/docs/installation.html) +
[devenv](https://devenv.sh/getting-started/).
    This way Nix installs the exact cross-compilation tools required for the e-reader, 
    while direnv automatically injects them into your shell the moment you enter the project directory.
1. Then execute the following to install the `armv7-unknown-linux-gnueabi` rustc target and `cargo-zigbuild`:
    ```bash
    just prerequisites
    ```

## Build

To build a binary crate located in this repo, run:

```bash
just pb_sdk_version=<sdk-version> build-app <name>
```

For example:
```bash
just pb_sdk_version=6.8 build-app inkview-slint
```

**NOTE:** while the demo projects lay in the [./examples/](./examples/) folder they are still apps,
and therefore should be built with the `build-app` command

To build an example:

```bash
just pb_sdk_version=<sdk-version> build-example <crate> <name>
```

For example:
```bash
just pb_sdk_version=6.8 build-example inkview hello_world
```

By default, the any build is going to be done with the `debug` profile.
Changing the profile to `release`, one should add the `cargo_profile=release` argument.

For example:
```bash
just pb_sdk_version=6.8 cargo_profile=release build-app inkview-slint-demo
```

## Deployment

To deploy a built binary to the device over USB, run the following,  
assuming the device is connected and appears in path `/run/media/$USER/<pb-device>`:

The path argument is the relative path from `target/armv7-unknown-linux-gnueabi/<cargo-profile>`,  
so for example: `examples/hello_world`

```bash
just pb_device=<your-device> deploy-usb <path-to-binary> <target-name>
```

## Bindings generation

See documentation for the `generate-bindings` just recipe.
