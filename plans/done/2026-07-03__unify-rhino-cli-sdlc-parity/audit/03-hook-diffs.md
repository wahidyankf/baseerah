# Hook diffs

## .husky/pre-commit (public vs primer)

```diff

```

## .husky/pre-commit (public vs infra)

```diff
1,4d0
< #!/usr/bin/env sh
< set -e
<
< # Step 1: Environment staged guard — reject staged real .env files
7c3,59
< # Step 2: Per-file formatters + tool-linters + per-file validators (file-type dispatch)
---
> # ShellCheck staged shell scripts (warning threshold). CI's `shell` job is the
> # hard gate; locally we lint when shellcheck is installed and skip with a hint
> # otherwise so a fresh checkout can still commit before `doctor --fix` runs.
> staged_sh=$(git diff --cached --name-only --diff-filter=ACM | grep '\.sh$' | grep -v 'husky/_/' | grep -v '^archived/' || true)
> if [ -n "$staged_sh" ]; then
>   if command -v shellcheck >/dev/null 2>&1; then
>     echo "$staged_sh" | xargs shellcheck --severity=warning
>   else
>     echo "shellcheck not found — skipping local shell lint (run: npm run doctor -- --fix). CI will enforce."
>   fi
> fi
>
> # hadolint changed Dockerfiles (warning threshold). CI's `dockerfile` job is the
> # hard gate; locally we lint when hadolint is installed and skip with a hint
> # otherwise so a fresh checkout can still commit before `doctor --fix` runs.
> staged_df=$(git diff --cached --name-only --diff-filter=ACM | grep -iE 'Dockerfile' | grep -v '^archived/' | grep -v '\.dockerignore$' || true)
> if [ -n "$staged_df" ]; then
>   if command -v hadolint >/dev/null 2>&1; then
>     echo "$staged_df" | xargs hadolint --failure-threshold warning
>   else
>     echo "hadolint not found — skipping local Dockerfile lint (run: npm run doctor -- --fix). CI will enforce."
>   fi
> fi
>
> # actionlint changed GitHub Actions workflows. CI's `actions` job is the hard
> # gate; locally we lint when actionlint is installed and skip with a hint
> # otherwise so a fresh checkout can still commit before `doctor --fix` runs.
> staged_wf=$(git diff --cached --name-only --diff-filter=ACM | grep -E '^\.github/workflows/.*\.ya?ml$' || true)
> if [ -n "$staged_wf" ]; then
>   if command -v actionlint >/dev/null 2>&1; then
>     echo "$staged_wf" | xargs actionlint
>   else
>     echo "actionlint not found — skipping local workflow lint (run: npm run doctor -- --fix). CI will enforce."
>   fi
> fi
>
> # D9 — IaC staged-file lint (infra-only deviation): terraform fmt + yamllint for staged
> # IaC files. Graceful skip if tools absent so a fresh clone can still commit.
> staged_tf=$(git diff --cached --name-only --diff-filter=ACM | grep -E '^infra/on-premise/terraform/.*\.tf$' || true)
> if [ -n "$staged_tf" ]; then
>   if command -v terraform >/dev/null 2>&1; then
>     terraform fmt -check -recursive infra/on-premise/terraform/ || true
>   else
>     echo "terraform not found — skipping local Terraform fmt check. CI will enforce."
>   fi
> fi
> staged_yaml=$(git diff --cached --name-only --diff-filter=ACM | grep -E '^infra/on-premise/ansible/|^\.github/.*\.ya?ml$|^\.yamllint\.yml$' || true)
> if [ -n "$staged_yaml" ]; then
>   if command -v yamllint >/dev/null 2>&1; then
>     yamllint infra/on-premise/ansible/ .github/
>   else
>     echo "yamllint not found — skipping local YAML lint. CI will enforce."
>   fi
> fi
>
> # Per-file formatters + tool-linters (file-type dispatch) — now includes *.rs via
> # rustfmt since the fmt/format:check Nx targets were dropped in favor of lint-staged.
10c62
< # Step 3: Regenerate + auto-stage harness bindings (config-sync)
---
> # Regenerate + auto-stage harness bindings (config-sync)
13c65
< # Step 4: Lockfile sync — regenerate + stage package-lock.json for staged apps
---
> # Lockfile sync — regenerate + stage package-lock.json for staged apps
```

## .husky/pre-push (public vs primer)

```diff
4c4
< # Pre-push: test:quick + spec validators + repo-wide cross-file gates + governance validators.
---
> # Pre-push: test:quick + compat:min-version + spec validators + repo-wide cross-file gates + governance validators.
11c11
< cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done --exclude apps/ayokoding-www/content --exclude apps/ose-www/content
---
> cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done --exclude apps/crud-be-elixir-phoenix/deps --exclude libs/elixir-cabbage/deps --exclude libs/elixir-gherkin/deps --exclude libs/elixir-openapi-codegen/deps --exclude apps/crud-be-kotlin-ktor/build --exclude libs/elixir-openapi-codegen/test --exclude apps/crud-be-kotlin-ktor/target
```

## .husky/pre-push (public vs infra)

```diff
4,5c4,7
< # Pre-push: test:quick + spec validators + repo-wide cross-file gates + governance validators.
< # Heavy integration + e2e suites are CRON-only (scheduled tiered pipelines) — never in this hook.
---
> # Pre-push: per-project test:quick (typecheck/lint/test:unit/test:coverage/test:specs) +
> # compat:min-version, plus repo-wide markdown lint, env-contract validation, and
> # changed-path-gated governance validators.
> # test:integration/test:e2e are CRON-only — NOT here.
10,12c12,14
< cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- env validate
< cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done --exclude apps/ayokoding-www/content --exclude apps/ose-www/content
< cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md readme-index validate
---
> cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- specs structure validate
> npm run lint:md
> npx nx run rhino-cli:env:validation
13a16,17
> cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done
> cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md readme-index validate
15c19
< # Naming validators — scoped to pushes that touch the relevant trees.
---
> # Range used by the scoped checks below.
20,21c24,27
<   if echo "$CHANGED" | grep -qE '^(\.claude/agents/|\.opencode/agent/)'; then
<     cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- harness naming validate
---
>
>   # Naming validators — scoped to pushes that touch the relevant trees.
>   if echo "$CHANGED" | grep -qE '^(\.claude/agents/|\.opencode/agents/)'; then
>     npx nx run rhino-cli:naming:harness-validation
24c30
<     cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance workflows naming validate
---
>     npx nx run rhino-cli:naming:workflows-validation
26,27c32,35
<   if echo "$CHANGED" | grep -qE '^repo-governance/.*\.md$'; then
<     cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor validate repo-governance/
---
>
>   # Vendored repo-governance audit — scoped to pushes that touch governance docs or AGENTS.md.
>   if echo "$CHANGED" | grep -qE 'repo-governance/.*\.md$|^AGENTS\.md'; then
>     npx nx run rhino-cli:governance:vendor-audit-validation
30,33c38,43
<   # Harness binding-parity guard + cross-vendor invariants (color/tier maps absorbed):
<   # run when binding or parity-relevant surfaces change.
<   if echo "$CHANGED" | grep -qE '^(\.amazonq/|AGENTS\.md$|CLAUDE\.md$|docs/reference/platform-bindings\.md$|repo-governance/.*\.md$|\.claude/|\.opencode/|\.codex/|\.github/|\.cursor/|\.windsurf/|\.junie/|GEMINI\.md$|CONVENTIONS\.md$)'; then
<     cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- harness bindings validate
---
>
>   # Amazon Q binding bridge parity (also absorbs the former multi-harness-parity
>   # invariants — harness bindings validate covers the same binding-parity ground) —
>   # scoped to pushes that touch the binding surface.
>   if echo "$CHANGED" | grep -qE '^(\.amazonq/|AGENTS\.md$|docs/reference/platform-bindings\.md$|\.claude/|\.opencode/|\.codex/|\.github/)'; then
>     npm run validate:harness-bindings
34a45,52
>
>   # D9 — Terraform lint gate (terraform fmt -check + validate + tflint), scoped to
>   # pushes that touch the Terraform sources. IaC linting is heavier than the
>   # pre-commit config linters, so it also runs here at pre-push (infra-only deviation).
>   if echo "$CHANGED" | grep -qE '^infra/on-premise/terraform/'; then
>     ./scripts/lint-terraform.sh
>   fi
>
37c55
<     cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- harness instruction-size validate
---
>     npx nx run rhino-cli:instruction-size:validation
38a57,74
>
>   # D9 — Ansible + YAML lint (ansible-lint + yamllint), scoped to pushes that touch
>   # the Ansible sources, workflow YAML, or the yamllint config (infra-only deviation).
>   if echo "$CHANGED" | grep -qE '^infra/on-premise/ansible/|^\.github/.*\.ya?ml$|^\.yamllint\.yml$'; then
>     # Prefer a self-consistent pipx toolchain in ~/.local/bin and expand any
>     # literal ~ in PATH (ansible-lint spawns ansible-config as a subprocess).
>     PATH="$HOME/.local/bin:$(printf '%s' "$PATH" | sed "s|~|$HOME|g")"
>     export PATH
>     # ansible-lint depends on a complete, version-matched local ansible toolchain
>     # that not every contributor environment provides. Run it as a local advisory
>     # — the CI `infra-lint` job enforces ansible-lint authoritatively (hard gate),
>     # so a broken local ansible install must not block the push. yamllint below
>     # stays a hard local gate (it has no such external-toolchain dependency).
>     if ! ( cd infra/on-premise/ansible && ansible-lint ); then
>       echo "warning: ansible-lint did not pass locally; the CI infra-lint job enforces it — verify before merge." >&2
>     fi
>     yamllint infra/on-premise/ansible/ .github/
>   fi
```
