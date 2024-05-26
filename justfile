# justfile for building for and transferring apps to a pocketbook device

# the SDK version. Either "5.19" or "6.5"
pb_sdk_version := "6.5"
pb_device := "PB626"
pb_sdk_cross_path := `realpath ../SDK_6.3.0/SDK-B288/usr/arm-obreey-linux-gnueabi`
gdbserver_port := "10003"

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
transfer-app path_to_binary target_app_name:
    cp "target/{{build_target / cargo_out_profile / path_to_binary}}" \
        {{"/run/media/$USER" / pb_device / "applications" / target_app_name}}
    sync {{"/run/media/$USER" / pb_device}}

# Launch `app-receiver.app` first on the device. Uses `utils/app-sender.sh` to send the application.
transfer-app-remote path_to_binary remote_app_name remote_ip remote_port="19991":
    echo "Sending application '{{path_to_binary}}' .."
    ./utils/app-sender.sh {{ "target" / build_target / cargo_out_profile / path_to_binary}} {{remote_app_name}} {{remote_ip}} {{remote_port}}
    echo "Sending application was successfull!"

start-app-gdbserver ssh_target ssh_port executable *args:
    ssh {{ssh_target}} -p {{ssh_port}} "RUST_LOG=debug RUST_BACKTRACE=1 gdbserver 0.0.0.0:{{gdbserver_port}} /mnt/ext1/applications/{{executable}} {{args}}"

generate-bindings:
    #!/usr/bin/env bash
    set -euxo pipefail
    export BINDGEN_EXTRA_CLANG_ARGS="{{bindgen_extra_clang_args}}"
    bindgen --dynamic-loading inkview {{pb_sdk_sysroot}}/usr/local/include/inkview.h -o inkview/src/bindings/bindings_{{replace(pb_sdk_version, ".", "_")}}.rs

[confirm]
clean:
    cargo clean
