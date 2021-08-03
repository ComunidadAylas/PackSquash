#!/bin/sh
mkdir -p target/debian && \
markdown README.md | \
html2text -nometa -utf8 -style pretty -rcfile .html2textrc | \
tail -n +11 > target/debian/extended_package_description.txt && \
cargo deb --no-strip "$@" && \
{ rm -f target/debian/extended_package_description.txt; true; } && \
cd target && \
# This proper way to get dependencies was commented by H-M-H at
# https://github.com/mmstick/cargo-deb/issues/170#issuecomment-891749827
touch debian/control && \
find . -iname 'packsquash' -type f -perm /ug=x | while read -r executable; do
	printf 'Dependencies for %s: ' "$executable" && \
	dpkg-shlibdeps "$executable" -O 2>/dev/null
done && \
printf 'GStreamer version: ' && \
apt-cache show gstreamer1.0-plugins-base | sed -n '/^Version:/{s/^Version: //;p;q}' && \
{ rm -f debian/control; true; } && \
cd ..
