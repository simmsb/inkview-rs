<h1 align="center">Inkview-rs</h1>

<h2 align="center">Rust bindings for pocketbook E-Reader devices (inkview)</h2>

<p align="center">
  <a href="https://crates.io/crates/inkview"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/inkview"></a>
  <a href="https://docs.rs/inkview/"><img alt="docs.rs" src="https://img.shields.io/docsrs/inkview"></a>
  <a href="https://discord.gg/s4nprWTYqF"><img alt="Discord" src="https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fdiscord.com%2Fapi%2Finvites%2Fs4nprWTYqF%3Fwith_counts%3Dtrue&query=%24.approximate_member_count&logo=discord&logoColor=white&color=green&label=Discord"></a>
</p>


This repo contains bindings for libinkview, which is used by pocketbook devices.

We load libinkview dynamically rather than linking it so that users of this
crate don't have to setup the pocketbook SDK. Instead you just need to
cross-compile to `armv7-unknown-linux-gnueabi.2.23` (`cargo zigbuild` works well
for this) and your binary will run on a pocketbook E-Reader.

Also in this repo is `inkview-slint` which provides a slint `Backend` that works
with inkview. And a demo application `inkview-slint-demo`.

`inkview-eg` is a [embedded-graphics-core](https://crates.io/crates/embedded-graphics-core) driver for inkview-rs.

## Prerequisites

[Zig](https://ziglang.org/learn/getting-started/#installing-zig) must be installed.

To run recipes from the justfile, install the [just](https://github.com/casey/just) command runner.

Available recipes can be listed with:

```bash
just --list
```

Then execute the following to install the `armv7-unknown-linux-gnueabi` rustc target and `cargo-zigbuild`:

```bash
just prerequisites
```

## Build

To build a binary crate located in this repo, run:

```bash
just pb_sdk_version=<sdk-version> build-app <name>
```

To build an example:

```bash
just pb_sdk_version=<sdk-version> build-example <crate> <name>
```

## Deploy a binary

To deploy a built binary to the device over USB, run the following,  
assuming the device is connected and appears in path `/run/media/$USER/<pb-device>`:

The path argument is the relative path from `target/armv7-unknown-linux-gnueabi/<cargo-profile>`,  
so for example: `examples/hello_world`

```bash
just pb_device=<your-device> deploy-usb <path-to-binary> <target-name>
```

## Generate bindings

See documentation for the `generate-bindings` just recipe.
