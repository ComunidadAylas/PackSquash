#!/bin/sh -e
mkdir -p target/debian

# Generate extended package description from README.md
markdown README.md | \
html2text -nometa -utf8 -style pretty -rcfile README.html2textrc | \
tail -n +11 > target/debian/extended_package_description.txt

# Build the Debian packages
cargo deb --no-strip "$@"
rm -f target/debian/extended_package_description.txt || true
