#!/bin/sh -e
# A script to download all the test packs.

if ! command -v python3 >/dev/null 2>&1; then
	echo "! Python 3 was not found. Please make sure it is installed and in the PATH variable." >&2
	false
fi

# Get the directory where this script resides
SCRIPT_DIR="$(dirname -- "$0")"
readonly SCRIPT_DIR

# Create a working directory for our Python download script and move to it
VENV_DIR=$(mktemp -d -t packdownload.XXX)
readonly VENV_DIR
trap 'rm -rf "$VENV_DIR" || true' EXIT INT TERM

# Set up a virtual environment so that we do not pollute the global Python
# packages list
echo "> Setting up temporary Python virtual environment"
python3 -m venv "$VENV_DIR"
. "$VENV_DIR/bin/activate"

pip3 install -r "$SCRIPT_DIR"/requirements.txt
"$SCRIPT_DIR"/download-gdrive-packs-folder.py "$@" 2>&1
