# Rust bindings for pocketbook ereader devices (inkview)

This repo contains bindings for libinkview, which is used by pocketbook devices.

We load libinkview dynamically rather than linking it so that users of this
crate don't have to setup the pocketbook SDK. Instead you just need to
cross-compile to `armv7-unknown-linux-gnueabi.2.23` (`cargo zigbuild` works well
for this) and your binary will run on a pocketbook ereader.

Also in this repo is `inkview-slint` which provides a slint `Backend` that works
with inkview. And a demo application `inkview-slint-demo`.

`inkview-eg` is a [embedded-graphics-core](https://crates.io/crates/embedded-graphics-core) driver for inkview-rs.

## Prerequisites

[Zig](https://ziglang.org/learn/getting-started/#installing-zig) must be installed.

To run the justfile, install the [just](https://github.com/casey/just) command runner.

Available recipes can be listed with:

```bash
just --list
```

Then run the following recipe to install the `armv7-unknown-linux-gnueabi` rustc target and `cargo-zigbuild`:

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

## Transfer binary

To transfer a built binary to the device over USB, run the following,  
assuming the device is connected and appears in path `/run/media/$USER/<pb-device>`:

The path argument is the relative path from `target/armv7-unknown-linux-gnueabi`,  
so for example: `examples/hello_world`

```bash
just pb_device=<your-device> transfer-app <path-to-binary>
```

## Generate bindings

Clone the SDK from here: https://github.com/pocketbook/SDK_6.3.0 and place it next to the `inkview-rs` directory.  
Check out the SDK version by switching branch. (currently: 5.19 or 6.5)

Execute the following from the `inkview-rs` directory to regenerate the bindings.
```bash
just pb_sdk_version=<sdk-version> generate-bindings
```
