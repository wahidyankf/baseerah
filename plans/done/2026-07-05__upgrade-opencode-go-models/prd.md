# Product Requirements Document

## Product Overview

`convert_model()` (in `apps/rhino-cli/src/application/agents/converter.rs`) is the single function
that decides which `opencode-go/` model ID every OpenCode agent frontmatter file gets, for every
agent, in all three repos. This plan changes what that function returns — and every doc, config
file, and test that encodes today's (stale) answer — so the OpenCode binding's actual capability
floor matches what this repo's own conventions already claim it to be.

## Personas

- **Solo maintainer working through OpenCode**: wants all three tiers — thinking (`opus`), execution
  (`sonnet`/omitted), and fast (`haiku`) — to each resolve to the strongest available non-frontier
  model for that tier, regardless of which binding (Claude Code or OpenCode) they happen to be using
  in a given session. Understands and accepts that the thinking tier may collapse onto the same
  model as the execution tier when no roster model separately clears Opus-tier capability. Does not
  want to have to know, from session to session, that one binding is quietly and unpredictably
  weaker than the other.
- **Solo maintainer trying Pi (`pi.dev`) for the first time**: if they pick up Pi instead of Claude
  Code or OpenCode, wants Pi's default model to already be the same cheap-but-capable
  `opencode-go/glm-5.2` rather than whatever Pi's own factory default happens to be, with the fast
  tier reachable via Pi's own model-cycling UI — without having to separately discover and configure
  any of it (user directive, 2026-07-05).
- **A future AI agent or contributor auditing model-tier claims**: reads `model-selection.md` or
  `ai-model-benchmarks.md` — in any of the 3 repos, since each carries its own copy — expecting the
  cited model IDs to actually resolve, the cited benchmark scores to reflect the models' current
  generation, and any claimed Claude reference model (e.g. "Opus 5") to actually exist.

## User Stories

1. As the maintainer, when I run a thinking-tier (`opus`) or execution-tier (`sonnet`/omitted) agent
   through OpenCode instead of Claude Code, I want the underlying model to be the strongest
   available in the `opencode-go` roster — clearing Claude Sonnet 5's tier where possible, and
   getting as close as possible to Claude Opus 4.8's tier where a roster model can't fully clear it
   — so that switching bindings never silently downgrades output quality. For the fast (`haiku`)
   tier, I want the closest available model to Sonnet 5 tier without exceeding it — not held to the
   Sonnet-5 bar itself, since deliberately staying at or below it is what makes it the fast tier.
2. As the maintainer, when I read `.opencode/opencode.json`'s `model`/`small_model` fields, I want
   both to reference model IDs that actually resolve in the live OpenCode client, so that I am never
   surprised by a silent fallback or hard failure from a retired ID.
3. As a future contributor reading `docs/reference/ai-model-benchmarks.md`, I want its OpenCode Go
   roster table and Claude reference point to reflect the currently-shipping model generations, so
   that I can trust the citation chain without independently re-verifying every number.
4. As the maintainer, when `ose-infra`'s OpenCode config diverges from the other two repos' provider
   choice, I want that divergence investigated and either reconciled or explicitly documented as
   intentional, so that cross-repo parity drift never goes unnoticed.
5. As the maintainer, when I next need to change this mapping (because the roster changed again), I
   want the `convert_model()` function and its Gherkin scenario to still describe the invariant
   precisely (which Claude tiers map to which OpenCode ID, and why), so that the next change is a
   small, well-scoped diff rather than a rediscovery exercise.
6. As the maintainer, when I run Pi against this repo for the first time, I want its project-level
   `.pi/settings.json` to already default to the same `opencode-go/glm-5.2`-tier model OpenCode
   uses, so that I don't unknowingly get a weaker or more expensive default from Pi's own factory
   settings (user directive, 2026-07-05). I do not need Pi's catalog status flipped to "Active" for
   this — the model pin stands on its own, ahead of any broader Pi adoption decision.

## Gherkin Acceptance Criteria

```gherkin
Feature: OpenCode Go model mapping uses the closest available model per tier

  Scenario: Converting a thinking-tier Claude model alias yields the closest available OpenCode Go model to Opus tier
    Given a Claude Code agent frontmatter with model "opus"
    When rhino-cli converts it to OpenCode agent frontmatter
    Then the corresponding .opencode/ agent uses the "opencode-go/glm-5.2" model identifier
    And this is documented as a collapse onto the execution tier's target, since no opencode-go
      roster model separately clears Claude Opus 4.8's benchmark tier

  Scenario: Converting an execution-tier Claude model alias yields the Sonnet-tier-or-above OpenCode Go model
    Given a Claude Code agent frontmatter with model "sonnet" or omitted
    When rhino-cli converts it to OpenCode agent frontmatter
    Then the corresponding .opencode/ agent uses the "opencode-go/glm-5.2" model identifier

  Scenario: Converting a fast-tier Claude model alias yields the closest OpenCode Go model to Sonnet tier without exceeding it
    Given a Claude Code agent frontmatter with model "haiku"
    When rhino-cli converts it to OpenCode agent frontmatter
    Then the corresponding .opencode/ agent uses the "opencode-go/minimax-m3" model identifier

  Scenario: The sync validator confirms every OpenCode agent matches the current mapping
    Given a Claude Code agent and its already-synced OpenCode counterpart
    When rhino-cli validates Claude-to-OpenCode sync correctness
    Then the validator reports the OpenCode agent's model field as in sync
    And no agent references a retired model ID ("opencode-go/glm-5" or "opencode-go/minimax-m2.7")

  Scenario: Top-level OpenCode config uses the current models for the primary and small-model slots
    Given the repo's ".opencode/opencode.json"
    When its "model" and "small_model" fields are read
    Then "model" resolves to "opencode-go/glm-5.2"
    And "small_model" resolves to "opencode-go/minimax-m3"

  Scenario: The engine change is byte-identical across all three repos
    Given "apps/rhino-cli/src/application/agents/converter.rs" in ose-public, ose-primer, and ose-infra
    When the files are diffed pairwise
    Then all three are byte-identical

  Scenario: ose-infra's OpenCode config no longer silently diverges on provider choice
    Given ose-infra's ".opencode/opencode.json" before this plan pointed at "zai-coding-plan/glm-5.1"
    When Phase 0's investigation completes
    Then the plan's delivery log records either a reconciliation to "opencode-go/glm-5.2" with the
      old provider's fields removed, or an explicit documented reason for the divergence remaining
    And no repo is left in a state where the divergence is simply unexplained

  Scenario: Pi defaults to the same model tier as OpenCode's thinking/execution tiers, with the fast tier reachable via cycling
    Given ose-public's ".pi/settings.json" does not exist before this plan
    When this plan creates it
    Then it sets "defaultProvider" to "opencode-go"
    And it sets "defaultModel" to Pi's catalog ID for glm-5.2, tagged [Needs Verification] per
      Decision 5/6 in tech-docs.md
    And it sets "enabledModels" to include both Pi's catalog IDs for glm-5.2 and minimax-m3
    And "docs/reference/platform-bindings.md"'s Pi row Status remains "Reserved" (not "Active")

  Scenario: Every repo's own governance/reference docs cite the current model mapping
    Given "ose-public", "ose-primer", and "ose-infra" each carry their own copy of
      "repo-governance/development/agents/model-selection.md" and
      "docs/reference/ai-model-benchmarks.md"
    When this plan's docs-refresh steps complete in each repo
    Then none of the 3 repos' copies contain "opencode-go/minimax-m2.7" or unsuffixed
      "opencode-go/glm-5"
    And each repo's "ai-model-benchmarks.md" "Last updated" date reflects this plan's execution date
```

## Product Scope

**In scope (product-facing behavior):**

- The exact OpenCode model ID every synced agent frontmatter file ends up with.
- The exact `model`/`small_model` values in each repo's `.opencode/opencode.json`.
- The exact `defaultProvider`/`defaultModel` values in `ose-public`'s new `.pi/settings.json`.
- The accuracy of every doc that documents this mapping for a human or agent reader, in all 3 repos.

**Out of scope (product-facing behavior):**

- Any change to Claude Code agent behavior, tool permissions, or frontmatter beyond the `model:`
  field's downstream OpenCode translation.
- Any UI, app, or end-user-facing product behavior — this plan is entirely internal tooling/config.
- Routing the OpenCode/Pi target to an Anthropic, OpenAI, Google, or other frontier/big-brand
  provider's model ID (e.g. `anthropic/claude-sonnet-5`) — explicitly rejected; see `brd.md`
  Business Scope Non-Goals and `tech-docs.md` Decision 0.
- Any harness other than Pi (Codex CLI, Copilot, Cursor, Windsurf, Junie, Aider) — see `tech-docs.md`
  Decision 4.
- Pi's catalog `Status` field, or any other Pi onboarding behavior beyond the model pin — see
  Decision 5.
