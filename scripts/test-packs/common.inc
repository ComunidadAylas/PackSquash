# shellcheck shell=sh

if ! command -v gpg >/dev/null 2>&1; then
	echo "! GnuPG was not found. Please make sure it is installed and in the PATH variable." >&2
	false
fi

# Set up GPG aliases and home directory for better unattended operation.
# Make sure we clean up any temporary data when we finish execution
alias gpg='gpg --batch --quiet --no-options --yes'
GNUPGHOME=$(mktemp -d -t gnupg.XXX)
export readonly GNUPGHOME
trap 'rm -rf "$GNUPGHOME" || true' EXIT INT TERM

# POSIX equivalent to readlink -f.
# From https://github.com/ko1nksm/readlinkf
readlinkf() {
	[ "${1:-}" ] || return 1
	max_symlinks=40
	CDPATH='' # to avoid changing to an unexpected directory

	target=$1
	[ -e "${target%/}" ] || target=${1%"${1##*[!/]}"} # trim trailing slashes
	[ -d "${target:-/}" ] && target="$target/"

	cd -P . 2>/dev/null || return 1
	while [ "$max_symlinks" -ge 0 ] && max_symlinks=$((max_symlinks - 1)); do
		if [ ! "$target" = "${target%/*}" ]; then
			case $target in
				/*) cd -P "${target%/*}/" 2>/dev/null || break ;;
				*) cd -P "./${target%/*}" 2>/dev/null || break ;;
			esac
			target=${target##*/}
		fi

		if [ ! -L "$target" ]; then
			target="${PWD%/}${target:+/}${target}"
			printf '%s\n' "${target:-/}"
			return 0
		fi

		# `ls -dl` format: "%s %u %s %s %u %s %s -> %s\n",
		#   <file mode>, <number of links>, <owner name>, <group name>,
		#   <size>, <date and time>, <pathname of link>, <contents of link>
		# https://pubs.opengroup.org/onlinepubs/9699919799/utilities/ls.html
		link=$(ls -dl -- "$target" 2>/dev/null) || break
		target=${link#*" $target -> "}
	done
	return 1
}

# Prints the root directory of the repository, if the working directory is
# a descendant of that directory. An error status code is returned if no
# repository is found.
find_repo_directory() {
	git_dir=$(readlinkf "$PWD")

	while
	next_git_dir=$(readlinkf "$git_dir/..")
	{ ! [ -d "$git_dir/.git" ]; } && [ "$next_git_dir" != "$git_dir" ]
	do
		git_dir="$next_git_dir"
	done

	[ -d "$git_dir/.git" ] && echo "$git_dir"
}

if ! ROOT_DIR=$(find_repo_directory); then
	echo "Could not find the root Git repository directory. Was this script moved out of its repository?" >&2
	exit 1
fi

# The root directory of the repository.
readonly ROOT_DIR

# The directory that contains the public test packs and encrypted private test packs.
readonly TEST_PACKS_PATH=packages/packsquash/benches/data/packs
# The directory that contains private test packs that were decrypted, and MUST
# be kept secret.
readonly DECRYPTED_TEST_PACKS_PATH=packages/packsquash/benches/data/packs/private
