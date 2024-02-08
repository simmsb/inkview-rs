#!/usr/bin/env bash

# Usage:
#
# $ inkview/bindgen.sh </abs/path/to/sdk/cross/path> <sdk-version>
#
# Example:
#
# $ inkview/bindgen.sh /mnt/Daten/source/pocketbook/SDK_6.3.0/SDK-B288/usr/arm-obreey-linux-gnueabi 6.5

pb_sdk_cross_path="${1}"
pb_sdk_version="${2}"
pb_sdk_sysroot="${pb_sdk_cross_path}/sysroot"

case $pb_sdk_version in
5.19)
echo "Selected SDK version: 5.19"
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot ${pb_sdk_sysroot} --target=armv7-unknown-linux-gnueabi -isystem/usr/include/freetype2"
;;
6.5)
echo "Selected SDK version: 6.5"
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot ${pb_sdk_sysroot} --target=armv7-unknown-linux-gnueabi"
;;
*)
echo "Supplied Sdk version is unsupported."
exit 1
;;
esac

bindgen --dynamic-loading inkview ${pb_sdk_sysroot}/usr/local/include/inkview.h -o inkview/src/bindings/bindings_${pb_sdk_version//./_}.rs