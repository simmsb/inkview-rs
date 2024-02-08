# Rust bindings for pocketbook ereader devices (inkview)

This repo contains bindings for libinkview, which is used by pocketbook devices.

We load libinkview dynamically rather than linking it so that users of this
crate don't have to setup the pocketbook SDK. Instead you just need to
cross-compile to `armv7-unknown-linux-gnueabi.2.23` (`cargo zigbuild` works well
for this) and your binary will run on a pocketbook ereader.

Also in this repo is `inkview-slint` which provides a slint `Backend` that works
with inkview. And a demo application `inkview-slint-demo`.

# inkview-eg

[embedded-graphics-core](https://crates.io/crates/embedded-graphics-core) driver for inkview-rs

# Generate bindings

Clone the SDK from here: https://github.com/pocketbook/SDK_6.3.0 and place is next to the `inkview-rs` directory.  
Check out the SDK version by switching branch. (currently: 5.19 or 6.5)

Execute the following from the `inkview-rs` directory to regenerate the bindings.
```bash
just pb_sdk_version=<sdk-version> generate-bindings
```

# Examples

Build examples like so:

```bash
just pb_sdk_version=<sdk-version> build-example <crate> <example-name>
```

The built binary will be in `target/armv7-unknown-linux-gnueabi/debug/<example-name>`.
