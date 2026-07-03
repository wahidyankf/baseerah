# rhino-cli Byte-Identity Matrix (Phase 5)

Verified 2026-07-02, after Phase 3 (primer) and Phase 4 (infra) both closed out.

| Path                          | public ↔ primer | public ↔ infra |
| ----------------------------- | :-------------: | :------------: |
| `apps/rhino-cli/src/`         |       ✅        |       ✅       |
| `apps/rhino-cli/tests/`       |       ✅        |       ✅       |
| `apps/rhino-cli/Cargo.toml`   |       ✅        |       ✅       |
| `apps/rhino-cli/Cargo.lock`   |       ✅        |       ✅       |
| `apps/rhino-cli/project.json` |       ✅        |       ✅       |
| `apps/rhino-cli/LICENSE`      |       ✅        |       ✅       |

All six rows: `diff -rq`/`diff -q` empty (zero carve-outs). `LICENSE` in ose-infra is MIT (relicensed
per Decision 7/Task "P4: regenerate apps/rhino-cli to canonical, relicense MIT"), matching
`Cargo.toml`'s `license = "MIT"` in all three repos.

**Deliberately excluded from this matrix** (not part of the byte-identity boundary — see
[nx-targets.md §Cross-Repo rhino-cli Byte-Identity Standard](../../../repo-governance/development/infra/nx-targets.md#cross-repo-rhino-cli-byte-identity-standard),
rule 1): `README.md` (legitimately carries repo-specific "See also" links to each repo's own historical
migration docs — restored to each repo's original after an early rsync briefly, incorrectly, overwrote
it), `rust-toolchain.toml`/`.gitignore`/`deny.toml`/`scripts/deny-check.sh` (auxiliary build-tooling
config, converged separately during Phase 4 as a discovered gap since they track the same
byte-identical dependency tree, not part of the formal boundary).

## Commands used

```bash
PUB=<ose-public>/apps/rhino-cli
PRI=<ose-primer>/apps/rhino-cli
INF=<ose-infra>/apps/rhino-cli
diff -rq "$PUB/src" "$PRI/src"; diff -rq "$PUB/src" "$INF/src"
diff -rq "$PUB/tests" "$PRI/tests"; diff -rq "$PUB/tests" "$INF/tests"
for f in Cargo.toml Cargo.lock project.json LICENSE; do
  diff -q "$PUB/$f" "$PRI/$f"; diff -q "$PUB/$f" "$INF/$f"
done
```

All exit 0 (no output = no differences).
