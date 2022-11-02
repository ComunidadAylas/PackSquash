#!/bin/sh -e

cd "$(git rev-parse --show-toplevel)"

mkdir -p target/debian

markdown README.md | \
sed -n '/^<h2>✨ Contributors<\/h2>$/q;p' | \
html2text -nometa -utf8 -style pretty -rcfile scripts/README.html2textrc | \
tail -n +12 > target/debian/extended_package_description.txt
