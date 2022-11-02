#!/bin/sh -e

readonly PACKAGE="$1"
shift

mkdir -p target/debian

scripts/generate_extended_deb_description.sh
cargo deb --no-strip -p "$PACKAGE" "$@"
