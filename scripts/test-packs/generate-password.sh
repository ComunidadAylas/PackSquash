#!/bin/sh -e
# Script to generate a long, cryptographically-secure random password.

# The number of cryptographically-secure random bytes that the generated password will have.
readonly PASSWORD_BYTES=256

exec 2>/dev/null
dd if=/dev/random bs=1 count="$PASSWORD_BYTES" | od -v -An -txC | LC_ALL=C tr -d '[:space:]'
echo
