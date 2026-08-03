#!/usr/bin/env bash
# Validates the combined-runtime environment contract. The Vite client is
# same-origin and therefore owns no runtime environment template.
# Keep this value-free: committed templates may declare placeholders and defaults only.
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
backend_environment_template="$repository_root/apps/beaver-nest-be/.env.example"
backend_project="$repository_root/apps/beaver-nest-be/project.json"
compose_definition="$repository_root/infra/dev/beaver-nest-app/docker-compose.yml"
repository_config="$repository_root/repo-config.yml"

assert_contains() {
	local file_path="$1"
	local expected="$2"

	if ! grep -Fqx -- "$expected" "$file_path"; then
		printf 'Expected %s in %s\n' "$expected" "$file_path" >&2
		exit 1
	fi
}

assert_environment_key() {
	local key="$1"

	if ! grep -Eq "^${key}=" "$backend_environment_template"; then
		printf 'Expected environment key %s in %s\n' "$key" "$backend_environment_template" >&2
		exit 1
	fi
}

assert_environment_key BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS
assert_environment_key BEAVER_NEST_BE_HTTP_LISTEN_PORT
assert_environment_key BEAVER_NEST_BE_DEVELOPMENT_DATA_DIRECTORY
assert_environment_key BEAVER_NEST_BE_DATA_DIRECTORY
assert_environment_key BEAVER_NEST_BE_HOST_DATA_DIRECTORY
assert_environment_key BEAVER_NEST_BE_SQLITE_BUSY_TIMEOUT_MILLISECONDS
assert_environment_key BEAVER_NEST_BE_VPN_HOST_IP
assert_environment_key BEAVER_NEST_BE_PUBLIC_PORT
assert_environment_key BEAVER_NEST_BE_BACKUP_DIRECTORY

while IFS= read -r declared_key; do
	case "$declared_key" in
	BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS | BEAVER_NEST_BE_HTTP_LISTEN_PORT | BEAVER_NEST_BE_DEVELOPMENT_DATA_DIRECTORY | BEAVER_NEST_BE_DATA_DIRECTORY | BEAVER_NEST_BE_HOST_DATA_DIRECTORY | BEAVER_NEST_BE_SQLITE_BUSY_TIMEOUT_MILLISECONDS | BEAVER_NEST_BE_VPN_HOST_IP | BEAVER_NEST_BE_PUBLIC_PORT | BEAVER_NEST_BE_BACKUP_DIRECTORY) ;;
	*)
		printf 'Unexpected backend environment key %s in %s\n' "$declared_key" "$backend_environment_template" >&2
		exit 1
		;;
	esac
done < <(sed -nE 's/^([A-Z][A-Z0-9_]*)=.*/\1/p' "$backend_environment_template")

assert_contains "$repository_config" '    - root: apps/beaver-nest-be'
assert_contains "$repository_config" '      lang: fsharp'
assert_contains "$repository_config" '      runtime: { local: env-local, local-ci: compose }'
assert_contains "$repository_config" '      keys-from: apps/beaver-nest-be/.env.example'
assert_contains "$backend_project" '        "command": "scripts/start-development.sh"'
assert_contains "$compose_definition" '    BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS: 0.0.0.0'
assert_contains "$compose_definition" '    BEAVER_NEST_BE_HTTP_LISTEN_PORT: "19300"'
assert_contains "$compose_definition" '        source: ${BEAVER_NEST_BE_HOST_DATA_DIRECTORY:-/tmp/beaver-nest-unconfigured-data}'
assert_contains "$compose_definition" '        source: ${BEAVER_NEST_BE_BACKUP_DIRECTORY:-/tmp/beaver-nest-unconfigured-backups}'

for deferred_key in \
	BEAVER_NEST_BE_DEVELOPMENT_DATA_DIRECTORY \
	BEAVER_NEST_BE_HOST_DATA_DIRECTORY \
	BEAVER_NEST_BE_VPN_HOST_IP \
	BEAVER_NEST_BE_PUBLIC_PORT \
	BEAVER_NEST_BE_BACKUP_DIRECTORY; do
	assert_contains "$repository_config" "        - ${deferred_key}"
done

if [[ -e "$repository_root/apps/beaver-nest-fe/.env.example" ]] || rg -Fq 'root: apps/beaver-nest-fe' "$repository_config"; then

	printf '%s\n' 'The same-origin Vite client must not own a runtime environment surface' >&2
	exit 1
fi
