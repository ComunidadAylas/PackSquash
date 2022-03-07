#!/bin/sh -e
# Creates an AppImage with the PackSquash CLI and its dependencies.
# AppImage is an universal Linux application distribution format, similar
# to macOS bundles, which packs an application and its dependencies to
# allow running it accross a wide variety of distros with no system changes
# and little user hassle.
#
# Relevant documentation:
# https://docs.appimage.org/packaging-guide/index.html
# https://appimage-builder.readthedocs.io/en/latest/index.html

# The directory where the generated AppImage bundles will be stored.
readonly TARGET_APPIMAGE_DIR='target/appimage'

alias wget='wget -nc -nv --show-progress -P "$APPIMAGE_WORKDIR"'

if ! command -v python3 >/dev/null 2>&1; then
	echo "! Python 3 was not found. Please make sure it is installed and in the PATH variable." >&2
	false
fi

# Create a temporary working directory for this AppImage script
APPIMAGE_WORKDIR=$(mktemp -d -t packsquash-appimagebuild.XXX)
readonly APPIMAGE_WORKDIR
trap 'rm -rf "$APPIMAGE_WORKDIR" || true' EXIT INT TERM

# Set up a virtual environment so that we do not pollute the global Python
# packages list with the packages we need to install
echo '> Setting up temporary Python virtual environment'
python3 -m venv "$APPIMAGE_WORKDIR/.venv"
. "$APPIMAGE_WORKDIR/.venv/bin/activate"

echo '> Install appimage-build in temporary Python virtual environment'
pip3 install -r /proc/self/fd/0 <<'REQUIREMENTS'
appimage-builder==0.9.2
certifi==2021.10.8
charset-normalizer==2.0.12
contextlib2==21.6.0
decorator==5.1.1
docker==5.0.3
emrichen==0.2.3
idna==3.3
jsonpath-rw==1.4.0
packaging==21.3
ply==3.11
prompt-toolkit==3.0.28
pyaml==21.10.1
pyparsing==3.0.7
PyYAML==6.0
questionary==1.10.0
requests==2.27.1
roam==0.3.1
ruamel.yaml==0.17.21
ruamel.yaml.clib==0.2.6
schema==0.7.5
six==1.16.0
urllib3==1.26.8
wcwidth==0.2.5
websocket-client==1.3.1
REQUIREMENTS

echo '> Downloading appimagetool'
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
mv "$APPIMAGE_WORKDIR/appimagetool-x86_64.AppImage" "$APPIMAGE_WORKDIR/appimagetool"
chmod +x "$APPIMAGE_WORKDIR/appimagetool"
export PATH="$PATH:$APPIMAGE_WORKDIR"

echo '> Running appimage-builder'

# Set the APPIMAGE_EXTRACT_AND_RUN environment variable to not depend on FUSE,
# which is troublesome in Docker containers
APPDIR="$APPIMAGE_WORKDIR/AppDir" REPO_DIR="$APPIMAGE_WORKDIR/repo" \
VERSION="$(git describe --tags --dirty=-custom)" \
APPIMAGE_EXTRACT_AND_RUN=1 \
appimage-builder --recipe appimage/recipe.yml --skip-tests

echo '> Cleaning up appimage-builder cache'
rm -rf appimage-builder-cache 2>/dev/null || true

echo "> Moving generated AppImage to $TARGET_APPIMAGE_DIR"
mkdir -p "$TARGET_APPIMAGE_DIR"
mv -f PackSquash-*.AppImage* "$TARGET_APPIMAGE_DIR"
