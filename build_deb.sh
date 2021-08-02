#!/bin/sh
mkdir -p target/debian && \
markdown README.md | \
html2text -nometa -utf8 -style pretty -rcfile .html2textrc | \
tail -n +11 > target/debian/extended_package_description.txt && \
cargo deb --no-strip "$@"
