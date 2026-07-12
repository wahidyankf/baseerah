# 31 · Agentic Coding (Annotated-concept, ‡ polyglot)

**prd row**: Pass 2 · Depth, Design & Craft · Annotated-concept · ‡ polyglot · Learn 131 / Drill 231 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: using AI coding agents as a disciplined development-workflow skill — prompting for
code, the review/verification loop, when to trust versus verify, context management, and guardrails.
`‡ polyglot`: the skill is workflow, not syntax, so the target language varies while the loop stays the
same. This is the _user's_ side of agents; the _builder's_ side — how agents are constructed — is
revisited in the AI band at [`54-agentic-ai`](./54-agentic-ai.md).

## Why this exists · the big idea

- **The problem before the solution**: an agent will produce plausible, confident, wrong code fast;
  used naively it accelerates the creation of bugs and quietly erodes the author's understanding of
  their own codebase.
- **Keep-this-if-you-forget-everything**: the agent drafts, you verify — treat generated code as an
  untrusted contribution that must pass the same tests, review, and reasoning as any other, and keep a
  tight loop where you check before you build on it.
- **Big ideas touched**: `correctness-vs-pragmatism` (agents are a pragmatism engine — enormous
  leverage on the routine, but correctness stays your job via tests and review, not the model's),
  `determinism-vs-emergence` (the same prompt yields different output — you manage a non-deterministic
  collaborator with context, constraints, and verification rather than expecting a repeatable
  function).

## Prerequisites

- **Prior topics**: [topic 15 Software Testing](./15-software-testing.md) and
  [topic 30 Software Engineering Practices](./30-software-engineering-practices.md).
- **Tools & environment**: an AI coding agent/assistant (editor-integrated and/or CLI); a
  version-controlled repo for safe, reversible iteration; a fast test suite as the verification
  harness; Neovim/VSCode with the agent integration (DD-17).
- **Assumed knowledge**: writing and running tests to verify a change (topic 15); code review, small
  commits, and working in trunk (topic 30); reading code in more than one language (the earlier
  primers).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the workflow patterns here (draft-then-verify, context management, tight
  review loops, guardrails) are model- and vendor-independent and deliberately not pinned to a specific
  tool or model version, which change rapidly. The named research foundations (the ReAct
  reasoning-and-acting loop, chain-of-thought prompting) are stable published concepts.
- 2026-07-12 — verified (GAP for plan owner): specific agent products, model names, and their
  capabilities move fast — keep the shipped text tool-agnostic and re-verify any named tool at
  authoring time.

## Items

- Prompting for code: giving the agent the goal, constraints, examples, and the acceptance criteria up
  front.
- The verification loop: read → run the tests → review the diff before accepting; never build on
  unverified output.
- Trust vs verify: which tasks are safe to delegate (boilerplate, mechanical refactors, tests-first
  scaffolds) and which demand close review (security, concurrency, novel logic).
- Context management: what to put in the agent's context window, what to leave out, and why more
  context is not always better.
- Guardrails: sandboxing, small reversible steps, tests as a tripwire, and keeping a human decision at
  every risky boundary.
- Failure modes: hallucinated APIs, confidently wrong refactors, silent scope creep, and the erosion
  of your own mental model.

## Tensions & trade-offs — when NOT to reach for this

- **Speed vs understanding**: delegating the code you most need to understand — the tricky core —
  trades short-term velocity for long-term ignorance of your own system. Verify most where it matters
  most; delegate most where it matters least.
- **Automation bias is the real risk**: a fluent, confident answer invites you to skip the review it
  most needs — the tool's persuasiveness is inversely correlated with your scrutiny unless you force
  the loop.
- **When NOT**: high-stakes, novel, or security-critical logic, and any situation where you can't
  cheaply verify the output — if you can't test or review it fast, the agent's speed is a liability,
  not an asset.

## Lineage — why it beat the alternative

- Assisted coding evolved from autocomplete to snippet generators to agents that read a repo, plan,
  edit, run tests, and iterate — the ReAct pattern (interleaved reasoning and acting) and
  chain-of-thought prompting are the research lineage that made the tool loop practical. It won over
  pure hand-coding for routine work because the leverage on boilerplate, mechanical refactors, and
  first drafts is large — but only under the discipline this topic teaches, which is exactly why
  testing ([topic 15](./15-software-testing.md)) and engineering practice
  ([topic 30](./30-software-engineering-practices.md)) are hard prerequisites. It hands off to
  [`54-agentic-ai`](./54-agentic-ai.md), which flips the perspective from _using_ agents to _building_
  them — tool calling, the agentic loop, and evals as the test suite for non-deterministic systems.

## Worked examples

Colocated under `agentic-coding/learning/`; each is a recorded agent session — the prompt, the
generated diff, and the verification that accepted or rejected it (DD-20/DD-30). Polyglot: the target
language varies; the workflow does not.

- **beginner** — delegate a well-specified pure function tests-first; verify by running the suite
  before accepting the diff.
- **intermediate** — drive a mechanical refactor across several files, reviewing each diff; catch and
  reject one confidently-wrong change with a reason.
- **advanced** — a context-managed feature loop: scope a change, constrain the agent, iterate against a
  failing test to green, and document what you chose to verify by hand versus trust.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: complete one real feature with an agent under a verify-first discipline — a specified
  prompt, tests as the tripwire, reviewed diffs, and a written record of what you trusted versus
  verified — landing a change you fully understand.
- **Concepts exercised**: [ ] a specified prompt with constraints + acceptance criteria [ ] a
  tests-as-tripwire verification loop [ ] a trust/verify decision log [ ] context management [ ]
  catching + rejecting a wrong generation [ ] small reversible steps.
- **Ordered steps**:
  1. `.../learning/capstone/prompt.md` — the goal, constraints, examples, acceptance criteria, and a
     failing test. Verify the test fails and the prompt states the acceptance bar.
  2. `.../session/` — drive the agent to green in small steps, reviewing each diff. Verify every
     accepted diff was run and reviewed, and at least one bad generation was caught and rejected with a
     reason.
  3. `.../trust-verify-log.md` — record what was delegated vs hand-verified and why. Verify each risky
     change maps to a human verification step.
- **Acceptance criteria**: the feature passes its tests; every accepted change was verified before
  being built on; the log justifies each trust/verify call; no unreviewed agent output reached the
  final diff.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **AI Engineering: Building Applications with Foundation Models** — Chip Huyen (2025). Current standard
  reference for building production applications, including agents, on top of foundation models.

**Papers & articles**

- **ReAct: Synergizing Reasoning and Acting in Language Models** — Shunyu Yao, Jeffrey Zhao, Dian Yu,
  Nan Du, Izhak Shafran, Karthik Narasimhan, Yuan Cao (2022). Foundational paper defining the
  interleaved reasoning-and-acting loop underlying most modern coding agents.
  <https://arxiv.org/abs/2210.03629>
- **Chain-of-Thought Prompting Elicits Reasoning in Large Language Models** — Jason Wei et al. (2022).
  Foundational paper showing intermediate reasoning steps improve LLM performance, underpinning prompt
  and context engineering practice. <https://arxiv.org/abs/2201.11903>
- **Building Effective Agents** — Anthropic (2024). Widely cited engineering guide distinguishing
  workflows from agents and giving practical patterns for agentic systems.
  <https://www.anthropic.com/engineering/building-effective-agents>
- **Best Practices for Claude Code** — Anthropic (documentation, continually updated). Official
  guidance on agentic coding workflows, context management, and tool use for coding agents.
  <https://code.claude.com/docs/en/best-practices>

---

← Previous: [30 · Software Engineering Practices](./30-software-engineering-practices.md) · Next: [32 · Software Product Engineering](./32-software-product-engineering.md) →
