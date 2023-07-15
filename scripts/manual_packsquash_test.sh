#!/bin/sh -e
# Builds the current PackSquash source in release mode and runs it with a
# predefined set of options against some pack. Useful for manual, quick
# one-off tests (just drop the assets to a folder and run). The pack
# directory will default to "test_pack" (without the quotations)

while getopts v:d: option; do
	case $option in
		v)	readonly PACK_FORMAT="$OPTARG";;
		d)	PACK_DIRECTORY="$OPTARG";;
		*)	echo "Syntax: $0 [-v <pack format version>] [-d <pack directory>]" >&2
			exit 1;;
	esac
done
readonly PACK_DIRECTORY="${PACK_DIRECTORY:-test_pack}"

# Make sure that the pack directory exists
mkdir -p "$PACK_DIRECTORY"

# Create the pack metadata file if it does not exist
if ! [ -f "$PACK_DIRECTORY/pack.mcmeta" ]; then
	cat <<-MCMETA > "$PACK_DIRECTORY/pack.mcmeta"
	{
		"pack": {
			"pack_format": ${PACK_FORMAT:-15},
			"description": "Test pack"
		}
	}
	MCMETA
fi

# Build and run PackSquash. These options can be tweaked as desired.
# Keep in mind that the text is expanded by the shell
cargo run --release <<OPTIONS
pack_directory = '$PACK_DIRECTORY'
OPTIONS
