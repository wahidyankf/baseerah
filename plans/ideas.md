# Ideas

Quick ideas and todos that haven't been formalized into plans yet.

When an idea is ready for implementation, create a proper plan folder in `backlog/` and remove it from this list.

## Ideas List

### Rust Governance (added 2026-05-23 as rust-governance-audit after-action)

- Future plan: promote `tech-docs.md §4` (Rust crate structural checklist) to
  `repo-governance/development/quality/rust-crate-structural-checklist.md` once a second Rust crate
  is added to `ose-public`. Single-crate evidence is insufficient to validate the abstraction level.

### AyoKoding Web (added 2026-05-22 as ayokoding-web-learn-reorg after-action)

- Future plan: add canonical shape enforcement rules to `apps-ayokoding-www-by-example-checker` and `apps-ayokoding-www-in-the-field-checker` — validate that every checked topic follows the `<domain>/<area>/<topic>/{overview.md,by-example/,by-concept/,in-the-field/}` tree shape.
- Future plan: consider creating `apps-ayokoding-www-by-concept-checker` agent once the by-concept track has sufficient coverage to warrant dedicated structural validation.
- Indonesian content reorg (id/) is not needed: `content/id/` uses `belajar/` with Indonesian-named dirs and has no parallel platform-\*/human/ structure. No action required.

### Infrastructure

- Create IAM (Identity and Access Management) service/module for authentication and authorization

### Demo Apps

- Recheck all standards.

### Development Experience

- Standardize CIs
- .env backup scripts for rhino-cli
- simplify ayokoding-cli and ose-cli
- libraries update
- **Source-code credential scanning** — evaluate Betterleaks (gitleaks successor, MIT, v1.0.0 early 2026) for pre-commit + CI detection of hard-coded credentials in `.rs`/`.ts`/`.tf` source files once
  it reaches stable production use. This public repo already has free GitHub Secret Scanning
  post-push coverage (700+ partner patterns + AI-backed generic detection). Gitleaks itself is
  feature-frozen with an unresolved entropy false-positive regression
  ([#1830](https://github.com/gitleaks/gitleaks/issues/1830)) affecting Rust config struct field
  names. Re-evaluate after Betterleaks has 60+ days of production soak.
- Split mermaid diagrams in `plans/done/2026-04-26__organiclever-ci-staging-split/tech-docs.md` to satisfy validator rules (surfaced 2026-04-26 by `rhino-cli-mermaid-fixes`): 7 label_too_long + 2 width_exceeded violations across blocks 0 (line 7) and 1 (line 40), plus 2 subgraph_density warnings on 7-child WF subgraphs. Follow-up to `2026-04-26__rhino-cli-mermaid-fixes`.

### Stack Update Deferrals (added 2026-05-16 by stack-update plan)

- Future plan: migrate `aws-sdk-go` v1 → v2 (currently transitive via `narqo/go-badge`; v1 EOL 2025-07-31; S3-crypto CVEs CVE-2020-8911/8912 only affect `s3crypto` codepaths which our CLIs do not use).
- Future plan: TypeScript 6.0 migration once TS 6.x has 60+ days of soak (eligible after ~2026-05-23).
- Future plan: ESLint 10 + react-hooks 7 migration once those versions have 60+ days of soak.
- Future plan: Zod 4.x migration (post-cutoff; eligible after 60-day soak).
- Future plan: lucide-react 1.x migration (post-cutoff; eligible after 60-day soak).
- Future plan: @xstate/react 6.x migration (post-cutoff; eligible after 60-day soak).
- Future plan: TailwindCSS 4.3.x migration (post-cutoff; eligible after 60-day soak).
- Future plan: @effect/platform 0.96.x + effect 4.x migration (post-cutoff; eligible after 60-day soak).
- Future plan: Storybook 10.3/10.4 adoption (post-cutoff; downgrade in this plan to 10.2.10 for CVE clearance).
- Future plan: Volta → mise migration (volta last release Dec 2024).
- Future plan: Microsoft Defender / dotnet 10.0.300 brew bottle availability (currently install via dotnet-install.sh to ~/.dotnet).
- Future plan: bump `vite` to 7.4+ across all consumers, then adopt `@vitejs/plugin-react 6.0.1` (this plan reverted plugin-react 6.0.1 → `^5.1.4` because plugin-react 6 requires vite's `./internal` subpath which is unavailable on the installed transitive vite 7.3.1). Caret retained pending the vite bump.

### CI Flakes (added 2026-07-02 by unify-rhino-cli-sdlc-parity after-action)

- Investigate the recurring `*-test-local-deploy-prod` scheduled-workflow E2E failure (`Error: http://localhost:3101 is already used`) hitting `ayokoding-www-test-local-deploy-prod` and `wahidyankf-www-test-local-deploy-prod` — consistently failing on every scheduled run since at least 2026-06-28 regardless of commit SHA, while unit/lint/integration/specs jobs on the same runs pass. Push-triggered gates (`pr-quality-gate`, `main-ci`, `validate-env`) are unaffected and green. Likely a stale dev-server process or self-hosted-runner port collision from a prior/concurrent scheduled run, not a code regression — needs a dedicated investigation, out of scope for `unify-rhino-cli-sdlc-parity`.
