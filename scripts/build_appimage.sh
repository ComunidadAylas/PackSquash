#!/bin/sh -e
# Creates an AppImage with the PackSquash CLI and its dependencies.
# AppImage is an universal Linux application distribution format, similar
# to macOS bundles, which packs an application and its dependencies to
# allow running it accross a wide variety of distros with no system changes
# and little user hassle.
#
# Relevant documentation:
# https://docs.appimage.org/packaging-guide/from-source/linuxdeploy-user-guide.html#packaging-binaries-and-other-resources-manually
# https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s06.html
# (.desktop file format)
# https://github.com/linuxdeploy/awesome-linuxdeploy#linuxdeploy-plugins
# (linuxdeploy plugins)

alias wget='wget -nc -nv --show-progress -P "$APPIMAGE_WORKDIR"'

# Create a temporary working directory for this AppImage script
APPIMAGE_WORKDIR=$(mktemp -d -t packsquash-appdir.XXX)
readonly APPIMAGE_WORKDIR
trap 'rm -rf "$APPIMAGE_WORKDIR" || true' EXIT INT TERM

# The directory where the generated AppImage bundles will be stored.
readonly TARGET_APPIMAGE_DIR='target/appimage'

echo '> Downloading linuxdeploy'
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x "$APPIMAGE_WORKDIR/linuxdeploy-x86_64.AppImage"
wget https://raw.githubusercontent.com/linuxdeploy/linuxdeploy-plugin-gstreamer/12e0e983f9e01f4e450ef2534cfec10e8aab2539/linuxdeploy-plugin-gstreamer.sh
chmod +x "$APPIMAGE_WORKDIR/linuxdeploy-plugin-gstreamer.sh"

echo '> Running linuxdeploy'
VERSION="$(git describe --tags --dirty=-custom)" GSTREAMER_INCLUDE_BAD_PLUGINS=1 NO_APPSTREAM=1 \
UPDATE_INFORMATION='gh-releases-zsync|ComunidadAylas|PackSquash|latest|PackSquash-*x86_64.AppImage.zsync' \
"$APPIMAGE_WORKDIR/linuxdeploy-x86_64.AppImage" --appdir="$APPIMAGE_WORKDIR/AppDir" \
--executable="target/${CARGO_BUILD_TARGET:-.}/release/packsquash" --desktop-file='common/assets/packsquash.desktop' \
--icon-file='common/assets/app_icon.png' --icon-filename=packsquash \
--plugin=gstreamer --output=appimage

echo "> Copying generated AppImage to $TARGET_APPIMAGE_DIR"
mkdir -p "$TARGET_APPIMAGE_DIR"
mv -f PackSquash-*.AppImage* "$TARGET_APPIMAGE_DIR"
