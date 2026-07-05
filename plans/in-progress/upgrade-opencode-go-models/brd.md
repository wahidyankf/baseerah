# Business Requirements Document

## Business Goal

Every AI agent invoked through the OpenCode secondary platform binding should reason at the
strongest tier the `opencode-go` marketplace can offer for each of this repo's three model tiers —
**thinking** (`opus`), **execution** (`sonnet`/omitted), and **fast** (`haiku`) — mirroring the
tier philosophy this repo already applies to the primary (Claude Code) binding
(`repo-governance/development/agents/model-selection.md`'s "planning-grade"/"execution-grade"/"fast"
naming). Today that guarantee is silently broken: OpenCode's `haiku`-tier mapping
(`opencode-go/glm-5`) points at a retired model ID, and the `opus`/`sonnet`-tier mapping
(`opencode-go/minimax-m2.7`) has fallen ~7-23 points behind current Claude Sonnet on every published
coding/agentic benchmark. A developer running the same agent catalog through OpenCode instead of
Claude Code is unknowingly getting materially weaker output — with no signal that this happened,
since both bindings claim tier-equivalence in their own docs.

**Revised scope (user directive, 2026-07-05)**: rather than a 2-tier mapping (thinking and
execution merged), this plan implements 3 explicit tiers, matching Claude Code's own `opus`/
`sonnet`/`haiku` distinction structurally — even though, in the current `opencode-go` roster, no
model separately clears Claude Opus 4.8's tier, so the thinking tier's target collapses onto the
execution tier's (explicitly permitted by the user: "it is okay to use same model on multiple tiers
if no other options exist").

The same principle extends to any other genuinely provider-agnostic ("bring-your-own-model")
harness this repo's catalog already tracks: if a developer picks up `pi.dev` (a cataloged but
not-yet-adopted terminal coding-agent CLI) instead of OpenCode or Claude Code, it should default to
the same cheap-but-capable model rather than whatever the tool's own factory default happens to be
(user directive, 2026-07-05). This plan pre-seeds that pin for Pi specifically — see Business Scope
Non-Goals below for why the other cataloged harnesses are not included.

## Why This Matters

- **Silent capability regression**: the repo's own `model-selection.md` documents OpenCode's mapping
  as tier-equivalent to Claude Code's. That claim is currently false for both OpenCode tiers, and
  nothing detects the drift — model rosters change on their own schedule, independent of this
  repo's release cycle.
- **A dead model ID in production config**: `opencode-go/glm-5` no longer resolves in the live
  roster (confirmed via a live `opencode models` CLI check, not just docs). Any OpenCode session
  using the `haiku`-mapped tier, or the `small_model` fallback in `.opencode/opencode.json`, is
  either failing outright or silently falling back to undocumented default behavior — neither is
  acceptable for a config claiming a specific, deliberate model choice.
- **Provider drift already exists undetected**: `ose-infra`'s `.opencode/opencode.json` has quietly
  diverged onto an entirely different provider (`zai-coding-plan/*`) from the other two repos, with
  no plan or commit message explaining why. Left alone, this kind of silent per-repo drift compounds
  over time and erodes the multi-harness parity this repo otherwise treats as a hard rule.

## Affected Roles

- **The maintainer (solo)**, when working through OpenCode instead of Claude Code — gets the actual
  capability tier they believe they're getting.
- **Future AI agents and contributors reading `model-selection.md`/`ai-model-benchmarks.md`** — get
  an accurate, current citation trail instead of a doc frozen at a superseded model generation.

## Business-Level Success Metrics

- Zero OpenCode `model:`/`small_model` references to a model ID that does not resolve in the live
  `opencode models` roster, across all 3 repos.
- The execution (`sonnet`/omitted) tier benchmarks at or above Claude Sonnet 5 on at least one
  credible published coding/agentic benchmark (SWE-bench Pro or Terminal-Bench 2.1), confirmed by
  direct comparison against Anthropic's currently-shipping Sonnet model at time of execution. The
  thinking (`opus`) tier uses the strongest available roster model, benchmarked explicitly against
  Claude **Opus 4.8** (not a fabricated "Opus 5" — see the Business Risk below on this correction) —
  accepted as a collapse onto the execution tier's target when, as today, no roster model separately
  clears the Opus-4.8 bar. The fast (`haiku`) tier uses the closest available model to Sonnet 5 tier
  without exceeding it (user directive, 2026-07-05) rather than being held to the Sonnet-5 bar
  itself — see the Business Risk below for the accepted capability gap.
- `docs/reference/ai-model-benchmarks.md`'s "Last updated" date reflects this plan's execution date,
  and every benchmark figure it cites for both the Claude reference point and the `opencode-go`
  roster is sourced from a currently-live model generation (not a superseded one) — **in all 3
  repos**, since `ose-primer` and `ose-infra` each carry their own copy of this doc and both are
  currently stale (`ose-primer`: "Last updated: 2026-04-19"; confirmed via direct diff, 2026-07-05).
- Pi's `.pi/settings.json` (new file, `ose-public` only per Confirmed Decision 6) pins a model that
  is at-or-above Sonnet tier by the same benchmark bar used for OpenCode's primary tier — not a
  separately-chosen, unverified model.

## Business Scope Non-Goals

- No change to which Claude models the primary binding uses, or to the planning-grade/
  execution-grade/fast tier _definitions_ themselves — only the OpenCode-side translation target.
- No commitment to re-run this refresh on any fixed cadence. The `opencode-go` roster is documented
  (both in this repo's existing benchmarks reference and in this plan's own research) as changing
  without a fixed schedule; this plan fixes the current drift, it does not eliminate the possibility
  of future drift.
- No formal cost/budget modeling of the OpenCode Go subscription's rate limits beyond noting the
  tradeoff between the two chosen tiers — a solo-maintainer repo, so this is a judgment call already
  made in the Confirmed Decisions, not a business case requiring formal modeling. (This is distinct
  from the standard per-token API pricing citations added to `tech-docs.md`'s comparison table per
  user directive 2026-07-05 — those are informational/comparative, not a budget analysis.)
- **Onboarding Pi (`pi.dev`) as an adopted harness** — explicitly out of scope (Confirmed Decision
  6). This plan pre-seeds `.pi/settings.json`'s model pin only; it does not flip Pi's catalog Status
  from `Reserved` to `Active`, and does not extend the same treatment to any other harness (Codex
  CLI, Copilot, Cursor, Windsurf, Junie, Aider) despite several of them also being genuinely BYOM
  (Confirmed Decision 5).
- **Routing OpenCode/Pi directly to an Anthropic, OpenAI, Google, or other frontier/big-brand
  provider's model is explicitly not the fix** (user directive, 2026-07-05, originally scoped to
  Anthropic and same-day extended to explicitly also cover OpenAI/Google/other frontier labs: "for
  BYOM harnesses, they should also use non 'frontier' or models from big brands like anthropic or
  openai"). "At least Sonnet/Opus tier" is a _capability bar_, not an instruction to spend
  frontier-priced tokens — the entire point of the `opencode-go` secondary binding (and Pi's model
  pin) is a cheap, non-frontier alternative that clears that bar as best it can on its own merits. If
  a future roster refresh finds nothing in `opencode-go` (or another cheap third-party provider)
  clears a given tier, the correct response is to say so explicitly and treat it as a business risk
  to accept or escalate — not to quietly substitute a frontier model ID to make the bar trivially
  pass. This plan includes a separate "frontier/big-brand model reference" table
  (`tech-docs.md`) purely for cost/capability contrast — explicitly not a candidate shortlist.

## Business Risks

- **Roster re-drift risk**: because `opencode-go` model IDs change without a fixed cadence (already
  observed twice in the ~2 months between the prior `adopt-opencode-go` plan and this one), the fix
  this plan lands could itself go stale again on a similar timescale. Mitigated by this plan citing
  a live-CLI-verified snapshot (not docs-only) and by leaving an explicit, low-friction re-check
  procedure (`opencode models`) documented in the refreshed benchmarks reference for whoever notices
  drift next.
- **Fast-tier capability gap risk**: `opencode-go/minimax-m3` (the fast/haiku target) sits ~4.2
  percentage points below Claude Sonnet 5 on SWE-bench Pro (59.0% vs 63.2%) — a real, not
  noise-level, gap. Accepted per explicit user direction (2026-07-05: "map to second closest too")
  as the tradeoff for keeping a genuinely lighter/cheaper fast tier rather than routing every agent
  invocation through `glm-5.2`'s heavier, more rate-limited slot (880 req/5h, tied for the tightest
  in the 13-model roster).
- **Thinking-tier capability gap risk**: no `opencode-go` roster model clears Claude Opus 4.8's
  SWE-bench Pro bar (69.2%) — `glm-5.2` at 62.1% (the strongest confirmed roster model) is the
  closest available, ~7.1pp below. Accepted per explicit user direction (2026-07-05: "it is okay to
  use same model on multiple tiers if no other options exist") as the thinking tier collapsing onto
  the execution tier's target, rather than this plan inventing a distinction the roster doesn't
  support.
- **"Opus 5" naming risk (corrected during plan-authoring, not left in the delivered plan)**: the
  user's initial framing for the thinking tier referenced "Opus 5" as the Claude comparison bar.
  `web-researcher` confirmed (2026-07-05) no such model exists — Opus 4.8 remains the current Opus
  generation. Anthropic did ship a higher tier (Claude Fable 5, GA 2026-06-09) above Opus, but it is
  not what Claude Code's `opus` alias resolves to and is out of scope per this plan's Non-Goals. This
  plan uses Opus 4.8 as the thinking tier's actual comparison bar throughout, and documents the
  correction explicitly (`tech-docs.md`'s "Correcting 'Opus 5'" section) rather than silently
  adjusting the number without explanation.
