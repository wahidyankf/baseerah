# repo-config.yml — canonical header comment block (ose-public)

Recorded verbatim (2026-07-02) as the copy-source for Phases 3-4's header canonicalization in
primer (drops the `env-injection` bullet) and infra (rewords the `coverage`/`specs`/`instruction-size`
comments). This is the exact block — lines 1-13 of `repo-config.yml` — do not paraphrase when copying.

```yaml
# repo-config.yml — schema: rhino-cli/repo-config/v1
#
# Unified repository configuration. Structure is byte-identical across all three
# repos (ose-public, ose-primer, ose-infra); values reflect each repo's actual
# project set and surfaces. rhino-cli commands read their relevant section.
#
# Sections defined here:
#   harness          — all-harness binding registry (§3.2); EVERY `harness` command reads this list
#   coverage         — per-project test-level registry (specs:behavior:coverage + specs:domain:coverage)
#   specs            — spec-tree structure config (specs:structure-validation)
#   instruction-size — per-surface instruction-file byte budgets (rhino-cli harness instruction-size validate)
#   env-contract     — surface registry for rhino-cli env validate (code↔config drift detection)
#   env-injection    — value-less injection manifest for rhino-cli env validate (manifest-consistency)
```

Verification: `grep -c '^#   env-injection' repo-config.yml` returns `1` (the header-summary bullet
line only — 2 other substring matches exist elsewhere in the file: the `env-injection:` companion
comment near its section body, and the actual top-level `env-injection:` key — both legitimate, not
part of this header block).
