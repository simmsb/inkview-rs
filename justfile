# Configurable options
# the SDK version. Either "5.19" or "6.5"
pb_sdk_version := "6.5"
pb_device := "PB626"
pb_sdk_cross_path := `realpath ../SDK_6.3.0/SDK-B288/usr/arm-obreey-linux-gnueabi`

pb_sdk_sysroot := pb_sdk_cross_path / "sysroot"
cargo_sdk_feature := "sdk-" + replace(pb_sdk_version, ".", "-")
cargo_profile := "dev"
build_target := "armv7-unknown-linux-gnueabi"
zigbuild_target := "armv7-unknown-linux-gnueabi.2.23"
cargo_out_profile := if cargo_profile == "dev" { "debug" } else { cargo_profile }
bindgen_extra_clang_args := if pb_sdk_version == "5.19" {
    "--target=" + build_target + " --sysroot " + pb_sdk_sysroot + " -isystem" + pb_sdk_sysroot + "/usr/include/freetype2"
} else if pb_sdk_version == "6.5" {
    "--target=" + build_target + " --sysroot " + pb_sdk_sysroot
} else {
    error("SDK version must be one of: '5.19', '6.5'.")
}

default:
    just --list

prerequisites:
    rustup target add {{build_target}}
    cargo install cargo-zigbuild

build-app name:
    cargo zigbuild --target {{zigbuild_target}} --profile {{cargo_profile}} -p {{name}} --features={{cargo_sdk_feature}}

build-example crate name:
    cargo zigbuild --target {{zigbuild_target}} --profile {{cargo_profile}} -p {{crate}} --example {{name}} --features={{cargo_sdk_feature}}

# Transfer a built app to the device. 'path_to_binary' is a relative path from 'target/<build_target>/<cargo_out_profile>/'.
transfer-app path_to_binary:
    cp "target/{{build_target / cargo_out_profile / path_to_binary}}" {{"/run/media/$USER" / pb_device / "applications" / file_name(path_to_binary) + ".app"}}
    sync

generate-bindings:
    #!/usr/bin/env bash
    set -euxo pipefail
    export BINDGEN_EXTRA_CLANG_ARGS="{{bindgen_extra_clang_args}}"
    bindgen --dynamic-loading inkview {{pb_sdk_sysroot}}/usr/local/include/inkview.h -o inkview/src/bindings/bindings_{{replace(pb_sdk_version, ".", "_")}}.rs

[confirm]
clean:
    cargo clean
