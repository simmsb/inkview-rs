# justfile for inkview-rs

# Cargo build profile.
cargo_profile := "dev"
# Additional cargo features
cargo_features := ""
# Pocketbook device identifier as it's folder name when connected with USB.
# - Pocketbook Inkpad 4: "PB743G"
# - Pocketbook Touch Lux 3: "PB626"
pb_device := "PB743G"
# Pocketbook libc version
# - Pocketbook Inkpad 4: "2.41"
# - Pocketbook Touch Lux 3: "2.23"
pb_libc_version := "2.41"
# Build target triple for Pocketbook device
pb_build_target := "armv7-unknown-linux-gnueabi"
# Pocketbook SDK version. Either "5.19", "6.5", "6.8", "6.10", "6.11"
pb_sdk_version := "6.11"
# GDB server port, used for debugging.
gdbserver_port := "10003"

[private]
pb_mount_root := if os() == "macos" { "/Volumes" } else { "/run/media/$USER" }
[private]
sdk_bindings_filename := "bindings_" + replace(pb_sdk_version, ".", "_") + ".rs"
[private]
build_target := pb_build_target
[private]
zigbuild_target := pb_build_target + "." + pb_libc_version
[private]
cargo_out_profile := if cargo_profile == "dev" { "debug" } else { cargo_profile }
[private]
cargo_sdk_feature := "sdk-" + replace(pb_sdk_version, ".", "-")
[private]
pb_sdk_sysroot := if pb_sdk_version == "5.19" {
    absolute_path("pocketbook-sdks/SDK-B288-5.19/SDK-B288/usr/arm-obreey-linux-gnueabi/sysroot")
} else if pb_sdk_version == "6.5" {
    absolute_path("pocketbook-sdks/SDK-B288-6.5/SDK-B288/usr/arm-obreey-linux-gnueabi/sysroot")
} else if pb_sdk_version == "6.8" {
    absolute_path("pocketbook-sdks/SDK-B288-6.8/SDK-B288/usr/arm-obreey-linux-gnueabi/sysroot")
} else if pb_sdk_version == "6.10" {
    absolute_path("pocketbook-sdks/SDK-B288-6.10/SDK-B288-6.10/usr/arm-obreey-linux-gnueabi/sysroot")
} else if pb_sdk_version == "6.11" {
    absolute_path("pocketbook-sdks/SDK-RK3566-6.11/arm-buildroot-linux-gnueabihf_sdk-buildroot/arm-buildroot-linux-gnueabihf/sysroot")
} else {
    error("SDK version must be one of: '5.19', '6.5', '6.8', '6.10', '6.11'.")
}
# Certain SDK versions need special clang arguments
[private]
bindgen_extra_clang_args := if pb_sdk_version == "5.19" {
    "--target=" + build_target + " --sysroot " + pb_sdk_sysroot + " -isystem" + pb_sdk_sysroot + "/usr/include/freetype2"
} else if pb_sdk_version == "6.11" {
    "--target=" + build_target + " --sysroot " + pb_sdk_sysroot + " -D__ARM_PCS_VFP"
} else {
    "--target=" + build_target + " --sysroot " + pb_sdk_sysroot
}

default:
    just --list

prerequisites:
    rustup target add {{build_target}}
    cargo install cargo-zigbuild

build-app name:
    cargo zigbuild --target {{zigbuild_target}} --profile {{cargo_profile}} -p {{name}} --no-default-features \
        --features="{{cargo_features}} {{cargo_sdk_feature}}"
    execstack -s "target/{{build_target / cargo_out_profile / name}}"

build-example crate name:
    cargo zigbuild --target {{zigbuild_target}} --profile {{cargo_profile}} -p {{crate}} --example {{name}} \
        --no-default-features --features="{{cargo_features}} {{cargo_sdk_feature}}"
    execstack -s "target/{{build_target / cargo_out_profile / 'examples' / name}}"

[doc("""
Transfer a built binary to the device via USB.
Options:
- `binary` - a file path relative to "target/<build_target>/<cargo_out_profile>/".
- `target_app_name` - name of the application (as is) on the targeted device
                      the `binary` is going to be renamed to.
                      For example: inkview-slint-demo.app
Command example:
`just cargo_profile=release pb_device=POCKETBOOK deploy-usb inkview-slint-demo inkview-slint-demo.app`
""")]
deploy-usb binary target_app_name:
    # 1. Copying the binary to the device
    cp "target/{{build_target / cargo_out_profile / binary}}" \
        {{ pb_mount_root / pb_device / "applications" / target_app_name }}
    # 2. Cleaning up macOS metadata files (if applicable)
    {{ if os() == "macos" { "dot_clean " + (pb_mount_root / pb_device / "applications") } else {"echo 'No cleanup needed'"} }}
    # 3. Flushing the filesytem cache
    sync {{pb_mount_root / pb_device}}
    @echo "Deployment successful!"

[doc("""
Transfer a built binary to the device via Wi-Fi.
Launch `app-receiver.app` first on the device.
Uses `utils/app-sender.sh` to send the application.
Options:
- `binary` - a file path relative to "target/<build_target>/<cargo_out_profile>/".
- `target_app_name` - name of the application (as is) on the targeted device
                      the `binary` is going to be renamed to.
                      For example: inkview-slint-demo.app
Command example:
`just cargo_profile=release pb_device=POCKETBOOK deploy-remote inkview-slint-demo inkview-slint-demo.app 192.168.1.27`
""")]
deploy-remote binary remote_app_name remote_ip remote_port="19991":
    echo "Sending application '{{binary}}' .."
    ./utils/app-sender.sh {{"target" / build_target / cargo_out_profile / binary}} {{remote_app_name}} {{remote_ip}} {{remote_port}}
    echo "Sending application was successfull!"

start-gdbserver ssh_target ssh_port executable *args:
    ssh {{ssh_target}} -p {{ssh_port}} "RUST_LOG=debug RUST_BACKTRACE=1 gdbserver 0.0.0.0:{{gdbserver_port}} /mnt/ext1/applications/{{executable}} {{args}}"

[doc("""
(Re-)generates SDK bindings.
First download and extract SDKs by initializing the git submodule through `git submodule update --init --recursive,
then executing "./pocketbook-sdks/extract.sh".
Then adjust variable `pb_sdk_sysroot`.
Requires tool "7z" and "bindgen" (install with `cargo install bindgen-cli`).
""")]
generate-bindings:
    #!/usr/bin/env bash
    set -euxo pipefail
    export BINDGEN_EXTRA_CLANG_ARGS="{{bindgen_extra_clang_args}}"
    inkview_h="{{pb_sdk_sysroot}}/usr/local/include/inkview.h"

    # Injecting this into the header breaks generating bindings for SDKs >= v6.10.
    # Manually enable this when needed.
    #inkview_h_tampered="$(mktemp)"
    #cp "${inkview_h}" "${inkview_h_tampered}"
    #printf "\nvoid do_partial_update(int x, int y, int w, int h, int flags0, int flags1);\n" >> "${inkview_h_tampered}"
    #inkview_h="${inkview_h_tampered}"

    bindgen \
        --dynamic-loading inkview \
        --no-layout-tests \
        --blocklist-function "_.*" \
        --blocklist-item "timer.*" \
        --blocklist-item ".*sigevent.*" \
        --blocklist-item ".*PTHREAD.*" \
        --blocklist-item ".*pthread.*" \
        --blocklist-var ".*pthread.*" \
        --blocklist-function ".*pthread.*" \
        --blocklist-type ".*pthread.*" \
        "${inkview_h}" \
        -o inkview/src/bindings/{{sdk_bindings_filename}}

    #rm "${inkview_h_tampered}"

[confirm]
clean:
    cargo clean

# Generate changelog for `target`
changelog target:
  cd {{target}} && git cliff -o CHANGELOG.md

# Generate changelog for all
changelog-all: (changelog 'inkview') (changelog 'inkview-eg') (changelog 'inkview-slint')
