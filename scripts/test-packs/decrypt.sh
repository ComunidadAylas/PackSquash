#!/bin/sh -e
# A script to decrypt all the private test packs in the test packs directory to
# a specific private test packs directory.
#
# The password used for decryption is read from the PRIVATE_TEST_PACKS_PASSWORD
# environment variable.

. "$(dirname -- "$0")"/common.inc

mkdir -p "$ROOT_DIR/$DECRYPTED_TEST_PACKS_PATH"

for encrypted_pack_file in "$ROOT_DIR"/"$TEST_PACKS_PATH"/*.gpg; do
	encrypted_pack_file_name=$(basename -- "$encrypted_pack_file" .gpg)

	if [ -f "$encrypted_pack_file" ] && [ "$encrypted_pack_file_name" != '*' ]; then
		echo "> Decrypting $encrypted_pack_file_name..."
		printf '%s' "${PRIVATE_TEST_PACKS_PASSWORD:?}" | gpg --passphrase-fd 0 \
			-o "$ROOT_DIR/$DECRYPTED_TEST_PACKS_PATH/$encrypted_pack_file_name" \
			--decrypt "$encrypted_pack_file"

		rm -f "$encrypted_pack_file"
	fi
done

echo
echo "All the private test packs at $TEST_PACKS_PATH were decrypted to $DECRYPTED_TEST_PACKS_PATH. Any decrypted pack is private and MUST NOT be uploaded or copied as-is anywhere."
echo
