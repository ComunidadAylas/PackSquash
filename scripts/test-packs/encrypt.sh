#!/bin/sh -e
# A script to encrypt all the private test packs, in the private test packs
# directory, to the test packs directory.
#
# The password used for encryption is read from the PRIVATE_TEST_PACKS_PASSWORD
# environment variable.

. "$(dirname -- "$0")"/common.inc

# The symmetric-key algorithm that will be used. It must be supported by the
# used gpg version.
readonly CIPHER=AES256
# The digest algorithm that will be used when converting the password to a key.
readonly DIGEST=SHA512

for decrypted_pack_file in "$ROOT_DIR"/"$DECRYPTED_TEST_PACKS_PATH"/*; do
	decrypted_pack_file_name=$(basename -- "$decrypted_pack_file")

	if [ -f "$decrypted_pack_file" ] && [ "$decrypted_pack_file_name" != '*' ]; then
		echo "> Encrypting $decrypted_pack_file..."
		printf '%s' "${PRIVATE_TEST_PACKS_PASSWORD:?}" | gpg -c --passphrase-fd 0 \
			--cipher-algo "$CIPHER" --s2k-digest-algo "$DIGEST" \
			-o "$ROOT_DIR/$TEST_PACKS_PATH/$decrypted_pack_file_name.gpg" \
			"$decrypted_pack_file"

		rm -f "$decrypted_pack_file"
	fi
done

echo
echo "All the private test packs at $DECRYPTED_TEST_PACKS_PATH were encrypted. You may now upload $TEST_PACKS_PATH publicly."
echo
