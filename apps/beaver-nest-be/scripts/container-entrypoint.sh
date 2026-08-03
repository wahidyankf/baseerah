#!/usr/bin/env bash

# Validate persistent storage before the unprivileged application starts. The
# image account is intentionally stable so an operator can prepare the bind
# source without granting the container broader host access.
set -euo pipefail

readonly beaver_nest_expected_owner='10001:10001'
readonly beaver_nest_directory_mode='700'
readonly beaver_nest_file_mode='600'

fail() {
	printf 'container-entrypoint: %s\n' "$1" >&2
	exit 1
}

stat_owner() {
	/usr/bin/stat --format='%u:%g' -- "$1"
}

stat_mode() {
	/usr/bin/stat --format='%a' -- "$1"
}

validate_owner() {
	local beaver_nest_path="$1"

	[[ "$(stat_owner "$beaver_nest_path")" == "$beaver_nest_expected_owner" ]] ||
		fail "unsafe ownership: $beaver_nest_path must be owned by $beaver_nest_expected_owner"
}

validate_mode() {
	local beaver_nest_path="$1"
	local beaver_nest_expected_mode="$2"

	[[ "$(stat_mode "$beaver_nest_path")" == "$beaver_nest_expected_mode" ]] ||
		fail "unsafe mode: $beaver_nest_path must have mode $beaver_nest_expected_mode"
}

validate_directory_tree() {
	local beaver_nest_directory="$1"
	local beaver_nest_path=''

	[[ -d "$beaver_nest_directory" ]] || fail "data directory does not exist: $beaver_nest_directory"
	[[ ! -L "$beaver_nest_directory" ]] || fail "data directory must not be a symlink: $beaver_nest_directory"
	[[ -w "$beaver_nest_directory" ]] || fail "data directory is not writable: $beaver_nest_directory"
	validate_owner "$beaver_nest_directory"
	validate_mode "$beaver_nest_directory" "$beaver_nest_directory_mode"

	while IFS= read -r -d '' beaver_nest_path; do
		[[ ! -L "$beaver_nest_path" ]] || fail "persistent storage must not contain symlinks: $beaver_nest_path"
		validate_owner "$beaver_nest_path"

		if [[ -d "$beaver_nest_path" ]]; then
			validate_mode "$beaver_nest_path" "$beaver_nest_directory_mode"
		elif [[ -f "$beaver_nest_path" ]]; then
			validate_mode "$beaver_nest_path" "$beaver_nest_file_mode"
		else
			fail "persistent storage contains unsupported path type: $beaver_nest_path"
		fi
	done < <(/usr/bin/find -P "$beaver_nest_directory" -mindepth 1 -print0)
}

umask 0077

readonly beaver_nest_data_directory="${BEAVER_NEST_BE_DATA_DIRECTORY:-/var/lib/beaver-nest}"
validate_directory_tree "$beaver_nest_data_directory"

if [[ -n "${BEAVER_NEST_BE_BACKUP_DIRECTORY:-}" ]]; then
	validate_directory_tree "$BEAVER_NEST_BE_BACKUP_DIRECTORY"
fi

exec "$@"
