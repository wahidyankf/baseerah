#!/usr/bin/env bash
# Shared readonly validation helpers for BeaverNest operational scripts.

beaver_nest_fail() {
	printf '%s\n' "$1" >&2
	return 1
}

beaver_nest_has_symlink_component() {
	local beaver_nest_path=$1
	local beaver_nest_component=$beaver_nest_path

	while [[ "$beaver_nest_component" != / && "$beaver_nest_component" != . ]]; do
		[[ -L "$beaver_nest_component" ]] && return 0
		beaver_nest_component=$(dirname -- "$beaver_nest_component")
	done

	return 1
}

beaver_nest_canonical_existing_directory() {
	local beaver_nest_path=$1
	[[ -n "$beaver_nest_path" && -d "$beaver_nest_path" && ! -L "$beaver_nest_path" ]] || return 1
	local beaver_nest_canonical
	beaver_nest_canonical=$(cd -P -- "$beaver_nest_path" && pwd -P) || return 1
	beaver_nest_has_symlink_component "$beaver_nest_canonical" && return 1
	printf '%s\n' "$beaver_nest_canonical"
}

beaver_nest_validate_safe_directory() {
	local beaver_nest_label=$1
	local beaver_nest_path=$2
	local beaver_nest_repository_root=$3
	local beaver_nest_canonical
	beaver_nest_canonical=$(beaver_nest_canonical_existing_directory "$beaver_nest_path") ||
		beaver_nest_fail "$beaver_nest_label must be an existing non-symlink directory" || return 1

	local beaver_nest_home
	beaver_nest_home=$(cd -P -- "$HOME" && pwd -P)
	[[ "$beaver_nest_canonical" != / && "$beaver_nest_canonical" != "$beaver_nest_home" && "$beaver_nest_canonical" != "$beaver_nest_repository_root" ]] ||
		beaver_nest_fail "$beaver_nest_label is not an allowed directory" || return 1

	case "$beaver_nest_repository_root" in
	"$beaver_nest_canonical"/*)
		beaver_nest_fail "$beaver_nest_label is not an allowed directory"
		return 1
		;;
	esac

	printf '%s\n' "$beaver_nest_canonical"
}

beaver_nest_validate_directory_mode() {
	local beaver_nest_label=$1
	local beaver_nest_path=$2
	local beaver_nest_mode
	beaver_nest_mode=$(stat -f '%Lp' "$beaver_nest_path" 2>/dev/null || stat -c '%a' "$beaver_nest_path")
	[[ "$beaver_nest_mode" == 700 ]] || beaver_nest_fail "$beaver_nest_label must have mode 0700"
}

beaver_nest_validate_operation_name() {
	local beaver_nest_name=$1
	[[ "$beaver_nest_name" =~ ^[A-Za-z0-9_-]+\.sqlite3$ ]] ||
		beaver_nest_fail 'operation name must be a basename ending in .sqlite3'
}
