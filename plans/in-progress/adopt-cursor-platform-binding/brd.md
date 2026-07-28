# Business Requirements — Adopt a Cursor Platform Binding

## Business Goal

Stop Cursor-launched delegated subagents from silently running on a six-times-more-expensive
inference tier, and stop Cursor from receiving agent definitions written in a model vocabulary it
does not understand — by generating a Cursor-native agent binding from the same `.claude/agents/`
source of truth that already feeds OpenCode and Amazon Q.

**In all three sibling repositories.** `ose-public`, `ose-primer`, and `ose-infra` each carry their
own agent roster, their own Cursor sessions, and their own inference bill. Fixing one and leaving two
would leave two thirds of the cost exposure in place — and, because `apps/rhino-cli` is byte-identical
across all three with pre-commit generation wired in every tree, would leave two repositories emitting
an undocumented generated directory the moment the shared source propagates.

## Business Rationale

### The cost asymmetry is the whole point

Cursor's Composer 2.5 ships in two toggles that run the same weights at different latency and price
[Web-cited — <https://cursor.com/blog/composer-2-5>, accessed 2026-07-28]:

| Toggle              | Input per MTok | Output per MTok |
| ------------------- | -------------- | --------------- |
| Composer 2.5        | $0.50          | $2.50           |
| Composer 2.5 (fast) | $3.00          | $15.00          |

Cursor's own blog states that **fast is the default option**, and that both tiers "run the same model
with the same intelligence" — fast is an inference-hardware choice, not a smarter model. Cursor's own
guidance names the standard tier as "the right pick for cloud agents, scheduled jobs, and CI
workflows", which describes this repository's delegated-subagent usage precisely.

The business consequence: every delegated subagent this repo launches inside Cursor is, by default,
billed at 6x input and 6x output for no capability gain. That is the pain point.

### The vocabulary mismatch is the second-order problem

Cursor resolves agent definitions in the order `.cursor/agents/` > `.claude/agents/` >
`.codex/agents/` [Web-cited — <https://cursor.com/docs/subagents>, accessed 2026-07-28]. With no
`.cursor/agents/` present in any of the three repositories [Repo-grounded — `test -e .cursor` returns
non-zero in all three trees], Cursor currently reads each repo's Claude agent files directly and
receives `model: sonnet`, `model: haiku`, `model: opus`, or no `model:` field at all. None of those is
a documented Cursor model ID.

The exposure is proportional to each repo's roster, and the rosters are not the same
[Repo-grounded — per-file `model:` scan of each `.claude/agents/` tree]:

| Repository   | Agents | `opus` | `sonnet` | `haiku` | `model:` omitted | Non-fast-pinned after this plan |
| ------------ | ------ | ------ | -------- | ------- | ---------------- | ------------------------------- |
| `ose-public` | 90     | 1      | 75       | 11      | 3                | 79                              |
| `ose-primer` | 64     | 1      | 58       | 2       | 3                | 62                              |
| `ose-infra`  | 53     | 1      | 47       | 2       | 3                | 51                              |

Do not treat `ose-public`'s roster as representative: `ose-primer` carries roughly two thirds of it
and `ose-infra` roughly half, and the fast-tier share differs by more than fivefold.

Cursor's documentation never states what it does with an unrecognised `model:` value. Silently
falling back to `inherit`, erroring, or mapping onto some Claude release are all plausible and none
is documented. Whatever it does today is undefined behaviour this repo is relying on by accident.

### Why a generated binding rather than a hand-written one

The repository already answers this question for two other harnesses. Hand-maintaining a binding
file guarantees eventual drift, which is exactly why
[Rule 4 of the Multi-Harness Binding Convention](../../../repo-governance/conventions/structure/multi-harness-binding.md)
mandates mechanical generation and Rule 5 mandates a deterministic byte-parity guard. Adding Cursor
as a third generated binding costs one converter and one registry entry; adding it as a hand-written
directory costs a permanent drift liability.

## Business Impact

### Pain points addressed

| Pain                                                               | Who feels it                     | How this plan addresses it                                         |
| ------------------------------------------------------------------ | -------------------------------- | ------------------------------------------------------------------ |
| Delegated Cursor subagents default to the 6x-priced fast toggle    | The maintainer paying the bill   | The non-fast pin emitted into every agent file, in all three repos |
| Cursor receives model aliases it does not document                 | Every Cursor-run agent           | Cursor-native model IDs replace the Anthropic tier aliases         |
| Cursor is the only major harness with no committed binding         | Future contributors on Cursor    | A catalogued, generated, guarded binding directory in each repo    |
| The standing "no thin pointer files" decision silently blocks this | Anyone re-reading the governance | An explicit amendment note per repo, not a silent deletion         |
| No repo surface says which Cursor surfaces are unreachable         | Anyone assuming full coverage    | An onboarding note naming the four out-of-reach surfaces, per repo |
| A propagated emitter would generate an undocumented directory      | The two sibling repos            | Each repo's landing carries emitter, output, and governance as one |

### Expected benefits

- **Cost**: the default inference tier for delegated subagent work moves from fast to standard.
  The magnitude of the saving depends entirely on Cursor-subagent usage volume, which this repo does
  not currently measure. _Judgment call:_ the change is directionally correct and low-risk; no
  baseline usage measurement exists, so no percentage is claimed.
- **Determinism**: Cursor stops relying on undefined handling of an unrecognised `model:` value.
- **Consistency**: three generated bindings, one generator, one guard, one registry.

### Cross-vendor cost caveat, stated honestly

The fast tier maps to Gemini 2.5 Flash at $0.30 input / $2.50 output [Web-cited —
<https://cursor.com/docs/models/cursor-composer-2-5>, accessed 2026-07-28]. Against Composer 2.5
standard's $0.50 / $2.50, the saving is **input-side only** — output price is identical. That choice
also introduces a **cross-vendor dependency** into the binding: the fast tier is no longer a Cursor
first-party model. Both facts are stated in the mapping table rather than buried.

## Affected Roles

These are solo-maintainer repositories; the roles below are hats worn by the same person plus the
agents that consume the changed files. There is no sign-off ceremony — the PR in each repo is that
repo's only gate.

| Hat / consumer                                        | What changes for them                                                         |
| ----------------------------------------------------- | ----------------------------------------------------------------------------- |
| Maintainer working inside Cursor, in any of the repos | Delegated subagents run on the pinned model; interactive sessions do not      |
| Maintainer paying the Cursor bill                     | Subagent inference moves off the 6x tier across all three repositories        |
| `repo-harness-compatibility-checker` (present in all) | Gains a third generated binding to audit, plus a new model-pin drift axis     |
| `rhino-cli` (`harness bindings` group)                | Gains a third emit target and a third parity check, identically in all repos  |
| Anyone reading a repo's platform-bindings catalog     | Cursor moves from its current status to a catalogued Tier-2 generated binding |
| `ose-primer` maintainer                               | Receives the emitter **and** a 64-file `.cursor/agents/` plus catalog updates |
| `ose-infra` maintainer                                | Receives the emitter **and** a 53-file `.cursor/agents/` plus catalog updates |

## Business-Level Success Signals

Every signal below is an observable check or an explicitly-labelled judgment call. No numeric target
is asserted as a measured fact, because no baseline exists. Signals 1, 2, 3, 5, and 7 are evaluated
**once per repository** — a green `ose-public` does not evidence `ose-primer` or `ose-infra`.

1. **Observable, per repo** — `.cursor/agents/` contains exactly one file per `.claude/agents/*.md`
   agent (excluding `README.md`): 90 in `ose-public`, 64 in `ose-primer`, 53 in `ose-infra`,
   verifiable by comparing the two counts in each tree.
2. **Observable, per repo** — every emitted `.cursor/agents/*.md` file whose Claude source is
   thinking-grade, execution-grade, or model-omitted carries the non-fast Composer 2.5 identifier:
   79 files in `ose-public`, 62 in `ose-primer`, 51 in `ose-infra`, verifiable with a count.
3. **Observable, per repo** — `cargo run --quiet --manifest-path apps/rhino-cli/Cargo.toml -- harness bindings validate`
   and `… -- harness naming validate` both exit 0 with the Cursor mirror present, and
   `harness naming validate` exits non-zero after a single file is removed from the mirror.
4. **Observable, once** — a real Cursor subagent launched from a `.cursor/agents/` definition is
   served by a non-fast Composer 2.5 model, evidenced by a captured session record committed under
   `evidence/`. This is a fact about Cursor, not about a repository, so one probe answers it for all
   three; each repo separately asserts that its own generated files carry the pinned literal.
5. **Observable, per repo** — no surface in that repository still describes Cursor as a harness with
   no committed binding; that repo's own verdict table in [`tech-docs.md`](./tech-docs.md) covers
   every occurrence in that tree.
6. **Judgment call** — the maintainer's Cursor inference spend on delegated subagent work drops.
   No baseline is measured and no percentage is claimed; the direction follows arithmetically from
   the published price table above.
7. **Observable, per repo** — `apps/rhino-cli/src/` and
   `specs/apps/rhino/behavior/rhino-cli/gherkin/` remain byte-identical across all three
   repositories after every landing, verifiable by checksum comparison.

## Business-Scope Non-Goals

- **Guaranteeing "always".** Interactive Cursor sessions, the `cursor-agent` CLI default, and Auto
  mode are outside any repo file's reach. This plan documents them; it does not claim them.
- **Working around Cursor's confirmed defects.** Two staff-confirmed bugs can defeat the pin. The
  plan verifies empirically and records the result; it does not attempt a workaround beyond the
  staff-recommended bracket syntax.
- **Configuring Enterprise Model Access Control.** A web-dashboard action, not a git artifact. It is
  named as the only real org-wide lever and left there.
- **Adding a Cursor instruction, MCP, or skills surface.** `.cursor/rules/`, `.cursor/mcp.json`, and
  `.cursor/skills/` stay absent; Cursor continues to read the root `AGENTS.md` natively for
  instructions. Adding an instruction surface would re-open the no-shadowing question this plan has
  no reason to touch.
- **Deriving `readonly` / `is_background` from Claude tool arrays.** Cursor documents both fields,
  but this repo has no unambiguous source for either, and inferring them would ship an unverified
  semantic alongside a verified one. The emitter omits both and lets Cursor default. Revisit only if
  a concrete need appears.
- **Measuring the saving.** No usage telemetry exists and none is added.
- **Repairing pre-existing per-repo drift.** `ose-infra` carries
  `.opencode/agents/ci-monitor-subagent.md` with no `.claude/agents/` source, tolerated today only by
  a hardcoded filename carve-out in `rhino-cli`'s `list_agent_files` [Repo-grounded]. That is a
  pre-existing condition this plan observes and records; it is routed as a separate backlog item, not
  fixed inside this plan's commits.
- **Unifying the three repositories' governance documents.** `docs/reference/platform-bindings.md`
  and `multi-harness-binding.md` have genuinely different structures per repo. This plan amends each
  in place, in that repo's own shape. Converging them is a different piece of work.

## Business Risks and Mitigations

| Risk                                                                              | Likelihood         | Mitigation                                                                                                   |
| --------------------------------------------------------------------------------- | ------------------ | ------------------------------------------------------------------------------------------------------------ |
| Cursor ignores the `model:` frontmatter (staff-confirmed defect)                  | Confirmed to occur | Phase 5 verifies empirically against a live subagent; Phase 1 re-checks the changelog before locking in      |
| The canonical model-ID slug differs from `composer-2.5`                           | Unknown            | Phase 1 spike U1 resolves it with a stated fallback; no emitter code is written until it is answered         |
| Cursor rejects or mishandles the bracket parameter syntax in a file               | Unknown            | Phase 1 spike U2 resolves it; fallback is to emit the bare slug and document the residual exposure           |
| Cursor renames or retires the Composer 2.5 slug later                             | Plausible          | The pin lives in one function and one governance table; `repo-harness-compatibility-checker` gains this axis |
| Adding a cross-vendor model to the fast tier couples the binding to a third party | Certain            | Stated explicitly in the mapping table and in this document rather than buried                               |
| `apps/rhino-cli` drifts out of byte-identity across the three repos               | Real               | Phases 6 and 7 land the identical source and Gherkin; the SDLC Gate Standard boundary is cited in each       |
| The generated directory trips pre-commit Prettier and breaks byte-equality        | Real precedent     | Phase 3 verifies with a falsifiable check before deciding whether `.prettierignore` needs an entry           |
| A sibling repo lands the emitter but not its governance record                    | Real               | Each repo's landing is one PR carrying emitter, generated output, and that repo's verdict table together     |
| One repo's honesty caveat is written and the other two overstate the guarantee    | Real               | The out-of-reach note is a per-repo delivery step with its own acceptance check in each landing phase        |
| A shared step assumes symmetry and fails in a repo whose document differs         | Confirmed to occur | Every governance surface differs per repo; the plan carries three verdict tables, not one shared step list   |
| Sibling-repo git topology is assumed rather than detected                         | Real               | Each landing phase detects topology with `git worktree list` and branches on the `(bare)` marker             |

## Related

- [`prd.md`](./prd.md) — the testable scenarios behind success signals 1 through 4
- [`tech-docs.md`](./tech-docs.md) — the design decisions and the per-file governance verdict table
- [Multi-Harness Binding Convention](../../../repo-governance/conventions/structure/multi-harness-binding.md)
- [Platform Bindings Catalog](../../../docs/reference/platform-bindings.md)
