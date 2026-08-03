#!/usr/bin/env bash

set -euo pipefail

if ! jq -e '.implicitDependencies | index("beaver-nest-fe")' apps/beaver-nest-be/project.json >/dev/null; then
	echo "beaver-nest-be must declare its combined-image dependency on beaver-nest-fe" >&2
	exit 1
fi

beaver_nest_affected_projects=$(npm exec -- nx show projects --affected --base=origin/main --head=HEAD)

if ! grep -Fxq "beaver-nest-fe" <<<"$beaver_nest_affected_projects"; then
	echo "expected the Vite workspace to be affected by the current delivery branch" >&2
	exit 1
fi

if ! grep -Fxq "beaver-nest-be" <<<"$beaver_nest_affected_projects"; then
	echo "expected the combined runtime to be affected by its frontend dependency" >&2
	exit 1
fi
