#!/bin/sh -e

cd "$(git rev-parse --show-toplevel)"

scripts/bundles/appimage/build.sh packages/packsquash_cli/bundles/appimage/recipe.yml
