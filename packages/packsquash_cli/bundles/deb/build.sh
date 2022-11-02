#!/bin/sh -e

cd "$(git rev-parse --show-toplevel)"

scripts/bundles/deb/build.sh packsquash_cli "$@"
