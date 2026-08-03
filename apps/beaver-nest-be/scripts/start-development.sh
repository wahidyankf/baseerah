#!/usr/bin/env bash

set -euo pipefail

beaver_nest_development_directory=${BEAVER_NEST_BE_DEVELOPMENT_DATA_DIRECTORY:-}

if [[ -z "$beaver_nest_development_directory" || ! -d "$beaver_nest_development_directory" || -L "$beaver_nest_development_directory" ]]; then
	echo "BEAVER_NEST_BE_DEVELOPMENT_DATA_DIRECTORY must name an existing, non-symlink directory" >&2
	exit 1
fi

beaver_nest_canonical_directory=$(cd "$beaver_nest_development_directory" && pwd -P)
beaver_nest_repository_root=$(git rev-parse --show-toplevel)

case "$beaver_nest_canonical_directory" in
/ | "$HOME" | "$beaver_nest_repository_root" | "$beaver_nest_repository_root"/*)
	echo "development data directory must be outside the repository, root, and home directory" >&2
	exit 1
	;;
esac

unset BEAVER_NEST_BE_HOST_DATA_DIRECTORY
unset BEAVER_NEST_BE_VPN_HOST_IP
unset BEAVER_NEST_BE_PUBLIC_PORT
unset BEAVER_NEST_BE_BACKUP_DIRECTORY

export BEAVER_NEST_BE_DATA_DIRECTORY="$beaver_nest_canonical_directory"
export BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS=127.0.0.1
export BEAVER_NEST_BE_HTTP_LISTEN_PORT=19320

exec dotnet watch run --project apps/beaver-nest-be/src/BeaverNestBe/BeaverNestBe.fsproj
