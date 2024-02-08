# Rust bindings for pocketbook ereader devices (inkview)

This repo contains bindings for libinkview, which is used by pocketbook devices.

We load libinkview dynamically rather than linking it so that users of this
crate don't have to setup the pocketbook SDK. Instead you just need to
cross-compile to `armv7-unknown-linux-gnueabi.2.23` (`cargo zigbuild` works well
for this) and your binary will run on a pocketbook ereader.

Also in this repo is `inkview-slint` which provides a slint `Backend` that works
with inkview. And a demo application `inkview-slint-demo`.

# Generate bindings

Pull the SDK from here: https://github.com/pocketbook/SDK_6.3.0 and check out the version by branch.
(currently: v5.19 or 6.5)

Execute `bindgen.sh` to regenerate the bindings. See the script source for how to invoke it.

# inkview-eg

embedded-graphics-core driver for inkview-rs

## Examples

Build examples like so (for SDK v5.19):

```bash
cargo zigbuild --target armv7-unknown-linux-gnueabi.2.23 -p inkview-eg --example icons --features="sdk-5-19"
```
