# Learnings — The Fundamentally Strong Software Engineer

Running knowledge-capture log for this plan. Append an entry whenever something is discovered during
execution that a future plan, agent, convention, skill, or doc should absorb. The **Phase 109 — Knowledge
Capture** step in [delivery.md](./delivery.md) triages every entry here to a durable home (convention /
agent / skill / docs) or explicitly discards it with a reason. No entry leaves this file untriaged.

## How to use

- One entry per learning. Keep it concrete and self-contained.
- Prefer capturing the **non-obvious** — surprises, gotchas, decisions and their rationale — not
  restatements of what the code or plan already says.
- At triage: mark each entry **→ Home: `<destination>`** (with a link) or **→ Discarded: `<reason>`**.

## Entry template

```markdown
### <short title>

- **What**: <the learning in 1-3 sentences>
- **Why it matters**: <the durable insight / cost of not knowing it>
- **Surfaced during**: <phase / topic / step>
- **Triage**: → Home: <convention|agent|skill|docs + link> _or_ → Discarded: <reason>
```

## Entries

### Intellectual layer over a breadth-first spiral

- **What**: The curriculum carries a JUDGMENT layer on top of the enumerated Items — a per-topic
  pedagogy opener (`## Why this exists · the big idea`: problem-before-solution +
  keep-this-if-you-forget-everything + big-ideas-touched), plus `## Tensions & trade-offs` and
  `## Lineage` on judgment topics — anchored by eight canonical Cross-Cutting Big Idea slugs
  (`abstraction-and-its-cost`, `taming-state`, `coupling-vs-cohesion`,
  `consistency-latency-throughput`, `mechanism-vs-policy`, `determinism-vs-emergence`,
  `correctness-vs-pragmatism`, `layering-and-leaks`).
- **Why it matters**: Breadth-first enumeration alone teaches _what_ exists, not _why_ or _when not_.
  The fixed slug set keeps the cross-cutting spine consistent across 90 topics instead of ad-hoc
  themes per file. This is the plan's defining pedagogical bet (DD-33).
- **Surfaced during**: Pedagogy design (items C–E) + weave passes.
- **Triage**: → pending (candidate home: a `docs/explanation/` note on the curriculum's pedagogy, or
  discard as plan-local if no reuse emerges).

### `## Read more` as mandatory penultimate section

- **What**: Every topic file ends with a `## Read more` section (after `## Capstone spec`, before the
  nav footer): `**Books**` then `**Papers & articles**`, each entry
  `- **Title** — Author(s) (Year). why-canonical.` + `<URL>` only for freely/legally-available works.
  Web-verified via `web-researcher`, 3–6 refs (up to 8 for rich topics), no fabrication.
- **Why it matters**: Anchors each topic to its canonical literature so a reader can go deep past the
  syllabus. The free-URL-only rule keeps links durable and legal. Codified in the authoring spec so
  every new + existing file gets it uniformly.
- **Surfaced during**: User directive (read-more), authoring-spec update (#124).
- **Triage**: → pending (candidate home: fold into the syllabus per-topic template convention).

### CI/CD split out as a standalone topic (#52)

- **What**: CI/CD & Release Engineering became its own topic (v3 #52,
  `cicd-and-release-engineering`) rather than a fold-in of Cloud/IaC or Containers, and was woven into
  the `capstone-real-world-delivery` inter-topic capstone (build → test → deploy pipeline step).
- **Why it matters**: Release engineering is a distinct competence with its own judgment surface
  (pipeline design, gating, promotion) that a deploy-config bullet under IaC would flatten.
- **Surfaced during**: v3 renumber + capstone reconciliation (#121, #122).
- **Triage**: → Discarded (plan-local topic-set decision; no cross-plan convention needed).

### Per-topic commit+push cadence baked into delivery.md

- **What**: `delivery.md` (mode `main-to-origin-main` implementation) commits + pushes to
  `origin main` after **every** completed topic — a standing HARD RULE plus a per-gate checklist item
  before each phase's Pause Safety note, with a `main-ci`-green acceptance gate before the next phase
  begins. Final Phase 107 reframed as a catch-up push (subtrees already landed).
- **Why it matters**: Each topic lands green independently instead of one giant end-of-plan push —
  smaller blast radius, earlier CI signal, matches the worktree-to-pr "maximize parallelization"
  ethos even under direct-to-main. Encoded renumber-robustly so the pending #79 renumber preserves it.
- **Surfaced during**: User directive (per-topic push), delivery.md edit (#126).
- **Triage**: → pending (candidate home: a delivery-cadence note in the plans convention, if reused).

### Backtick display-number vs link-target staleness is its own drift class

- **What**: A prior renumber pipeline fixed markdown link _targets_ and slugs but left the
  human-readable backtick display numbers stale (e.g. ``[`13-software-testing`](./15-software-testing.md)``
  — link right, display wrong). 103 such instances across 52 files. A separate canonicalization sweep
  set display = target.
- **Why it matters**: Renumber tooling that only rewrites link paths silently leaves a second,
  reader-visible number wrong. Any future renumber must sweep BOTH the `](./NN-slug.md)` target AND
  the ``[`NN-slug`]`` display token, and inter-topic capstone `(NN)` inline refs decoded by meaning.
- **Surfaced during**: Inter-topic capstone renumber (#122) + repo-wide slug sweep.
- **Triage**: → pending (candidate home: a note in whatever renumber convention/tool this plan spawns).

### npm `clang-format` has no native Apple Silicon macOS binary

- **What**: The `clang-format` npm wrapper (angular/clang-format) only bundles `darwin_x64`; on
  arm64 macOS its own `getNativeBinary()` silently falls back to the x64 binary via Rosetta, which
  then throws `assertion failed [header->version <= kProjectSourceVersion]: runtime library is
newer than runtime` — a confirmed, upstream-acknowledged gap (the package's own error message
  says "Consider installing it with your native package manager instead"), not a local
  misconfiguration. DD-36 originally classified it Tier-1 (npm-direct); reclassified to Tier-2
  (Homebrew/apt via `npm run doctor -- --fix`) instead.
- **Why it matters**: Any future DD-36-style "Tier-1 npm-installable" formatter decision for a repo
  with Apple Silicon contributors should verify the npm package ships (or builds) a native
  `darwin-arm64` binary before assuming npm-direct wiring is viable — `npm view <pkg>` alone won't
  surface this; the platform-fallback logic has to be read.
- **Surfaced during**: Phase 0, lint-staged formatter coverage item.
- **Triage**: → pending (candidate home: a note in the dependency-bump / tooling-selection
  convention about verifying native-arch binary coverage before choosing an npm-wrapper formatter).

### `_index.md` naming-validator gap was a live landmine, not a hypothetical

- **What**: rhino-cli's `md naming validate` always-exempted `README.md`/`SKILL.md`/`AGENTS.md`/
  `CLAUDE.md` but not Hugo's reserved `_index.md`. 268 pre-existing `_index.md` files across
  `ayokoding-www/content` alone already "violated" the rule undetected, because the lint-staged
  invocation only checks newly-staged files — the first time in this repo's history a _new_
  `_index.md` was staged, pre-commit blocked it. Fixed by adding `_index.md` to the same
  always-exempt list (same shape as the pre-existing `AGENTS.md`/`CLAUDE.md` regression fix its own
  test file documents).
- **Why it matters**: A validator with an incomplete allow-list can sit dormant for a long time —
  268 files' worth — until the exact right new file trips it. Any content-scaffold step in a plan
  should expect to be the one that finally exercises an untested code path in a shared repo-wide
  gate, and should treat that as a preexisting bug to root-cause-fix (Iron Rule 3), not a "just
  rename the file" workaround (impossible here — `_index.md` is Hugo-structural).
- **Surfaced during**: Phase 0, section-root scaffold commit (pre-commit hook failure).
- **Triage**: → pending (candidate home: none needed beyond the code fix itself — already landed;
  consider whether other ecosystem-reserved filenames, e.g. future non-Hugo apps, need a similar
  proactive audit rather than reactive discovery).
