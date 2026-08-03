#!/usr/bin/env bash

# Build the production image from a source-only copy. This prevents an image
# build from accidentally consuming generated clients or frontend output left
# in a developer's working tree.
set -euo pipefail

beaver_nest_repository_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)"
beaver_nest_build_root="$(mktemp -d)"
beaver_nest_image="beaver-nest-app:clean-image-build"

cleanup() {
	rm -rf -- "$beaver_nest_build_root"
	docker image rm --force "$beaver_nest_image" >/dev/null 2>&1 || true
}
trap cleanup EXIT

rsync --archive --delete \
	--exclude='.git/' \
	--exclude='node_modules/' \
	--exclude='.nx/' \
	--exclude='dist/' \
	--exclude='.next/' \
	--exclude='coverage/' \
	--exclude='generated-reports/' \
	--exclude='apps/beaver-nest-fe/src/generated-contracts/' \
	--exclude='specs/apps/beaver-nest/containers/contracts/generated/' \
	"$beaver_nest_repository_root/" \
	"$beaver_nest_build_root/"

docker build \
	--file "$beaver_nest_build_root/apps/beaver-nest-be/Dockerfile" \
	--tag "$beaver_nest_image" \
	"$beaver_nest_build_root"

docker run --rm --entrypoint sh "$beaver_nest_image" \
	-c 'test ! -x /usr/bin/node && test "$(id -u):$(id -g)" = "10001:10001"'

printf '%s\n' 'PASS: production image builds from source-only inputs as 10001:10001'
