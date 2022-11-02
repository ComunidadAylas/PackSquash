#!/bin/sh -e

cd "$(git rev-parse --show-toplevel)"

mkdir -p target/debian

markdown README.md | \
html2text -nometa -utf8 -style pretty -rcfile scripts/README.html2textrc | \
tail -n +11 > target/debian/extended_package_description.txt
