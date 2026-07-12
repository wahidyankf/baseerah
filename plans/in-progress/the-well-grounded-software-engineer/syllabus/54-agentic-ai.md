# 54 · Agentic AI (By Example, Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python † · Learn 154 / Drill 254 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: building agents, not just calling models — tool/function calling, the agentic loop,
the Model Context Protocol (MCP), memory and context management, and evals as the test suite for
non-deterministic systems. It follows [`53-creating-ai-powered-apps`](./53-creating-ai-powered-apps.md)
and turns a single model call into a system that reasons, acts, observes, and iterates toward a goal.
`†`: Python, fully type-annotated (DD-34) — every snippet carries type hints in the mypy-clean spirit.

## Why this exists · the big idea

- **The problem before the solution**: a single prompt-and-response can't do multi-step work — it
  can't look something up, run a tool, check the result, and adjust. Wiring that by hand as brittle
  string-parsing gives you a fragile chatbot, not an agent; and because the model is
  non-deterministic, the usual "run the test, it's green" safety net doesn't apply.
- **Keep-this-if-you-forget-everything**: an agent is a loop — the model decides an action, a tool
  executes it, the result feeds back as an observation, and it repeats until a goal or a stop
  condition. Give it tools, a memory, and a way to know it's done; then _evaluate_ it, because you
  can't unit-test a probabilistic thing the old way.
- **Big ideas touched**: `determinism-vs-emergence` (useful behaviour _emerges_ from a model looping
  over tools rather than from a coded-out control flow — powerful, but you trade predictability),
  `correctness-vs-pragmatism` ("provably correct" is off the table for a stochastic system, so you
  ship with evals, guardrails, and human checkpoints — disciplined compromise, not proof).

## Prerequisites

- **Prior topics**: [topic 52 CI/CD & Release Engineering](./52-cicd-and-release-engineering.md)
  (the pipeline that runs your eval suite as a gate) and [topic 15 Software Testing](./15-software-testing.md)
  (the testing mindset you'll adapt for non-deterministic systems).
- **Tools & environment**: a macOS/Linux terminal; **Python** at a recent stable release with type
  hints and `mypy`; access to an LLM with tool/function-calling (via an API or a local model); an MCP
  client/server library; an eval harness; Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: making a model API call and shaping a prompt (topic 53); writing and running
  a test suite and reasoning about coverage (topic 15); running a job in a CI pipeline (topic 52).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the **agentic loop** (reason → act → observe, the ReAct pattern),
  **tool/function calling**, and **evals as the test methodology** for non-deterministic systems are
  the stable, mainstream concepts; left correctly version-unpinned since specific model and SDK
  versions move fast.
- 2026-07-12 — verified (GAP for plan owner): the **Model Context Protocol (MCP)** is an open standard
  for connecting agents to tools/data and is broadly adopted, but its spec and SDKs are actively
  evolving — pin the exact MCP library version and any model/provider SDK at drafting time, and keep
  the teaching centered on the protocol's _role_ (a standard tool/context interface) rather than a
  frozen API. Do not hard-code any specific model's capabilities.

## Items

- Tool/function calling: exposing typed functions to the model, letting it choose and invoke them, and
  parsing structured tool calls back into real execution.
- The agentic loop: reason → act → observe → repeat, with a clear stop condition and a step budget to
  bound cost and runaway behaviour.
- MCP: connecting an agent to tools and data sources through a standard protocol instead of bespoke
  glue per integration.
- Memory and context management: short-term (the working transcript), longer-term retrieval, and
  keeping the context window relevant rather than full.
- Evals as the test suite: task datasets, scoring (exact-match, rubric, LLM-as-judge), and running
  evals in CI to catch regressions in a non-deterministic system.
- Guardrails and control: input/output validation, permission boundaries on tools, and human-in-the-
  loop checkpoints for consequential actions.

## Tensions & trade-offs — when NOT to reach for this

- **An agent is often the wrong tool**: if the task is a fixed, known sequence, a plain workflow (a
  deterministic pipeline that calls a model at one step) is cheaper, faster, and far more predictable
  than an autonomous loop. Reach for agency only when the path genuinely can't be enumerated in
  advance.
- **Autonomy multiplies cost and blast radius**: every loop iteration is tokens, latency, and a chance
  to take a wrong, possibly irreversible action. Unbounded tool access plus a loop is how an agent
  runs up a bill or deletes the wrong thing — bound the steps, scope the tools, and gate the dangerous
  ones behind a human.
- **No evals means no safety net**: without an eval suite, you can't tell whether a prompt tweak or a
  model upgrade improved or silently broke the agent. Shipping an agent you can't measure is shipping a
  system you can't maintain.

## Lineage — why it beat the alternative

- Agentic AI emerged once models got good enough at tool-use that the bottleneck moved from "can it
  answer" to "can it act". The ReAct pattern (2022) formalized interleaving reasoning with actions;
  Toolformer showed models could learn to call tools; Reflexion added self-critique loops. The
  practical winner over hand-coded orchestration is the constrained loop-with-tools-and-evals: it
  captures the flexibility that makes agents useful while the eval suite and guardrails supply the
  discipline that stochastic systems otherwise lack — and MCP standardized the tool interface so
  integrations stopped being bespoke. This builds directly on the model-application foundations of
  [topic 53 Creating AI-Powered Apps](./53-creating-ai-powered-apps.md) and uses the pipeline of
  [topic 52 CI/CD & Release Engineering](./52-cicd-and-release-engineering.md) to run evals as a gate.

## Worked examples

Colocated under `agentic-ai/learning/code/`; each runnable from the CLI, every Python snippet fully
type-annotated and `mypy`-clean (DD-20/DD-30/DD-34).

- **beginner** — expose two typed tools to a model and let it choose one via function calling; parse
  the tool call, execute it, and return the result.
- **intermediate** — a bounded agentic loop (reason → act → observe with a step budget and stop
  condition) that solves a small multi-step task using the tools, plus basic memory of prior steps.
- **advanced** — connect a tool via MCP and add an eval suite (task dataset + scoring) run in CI, so a
  prompt or model change is measured against a regression bar.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small but real agent that completes a multi-step task using typed tools over MCP,
  with a bounded agentic loop, memory, guardrails on consequential tools, and an eval suite that runs
  in CI — proving you can build, constrain, and _measure_ a non-deterministic system.
- **Concepts exercised**: [ ] typed tool/function calling [ ] bounded reason-act-observe loop [ ] MCP
  tool connection [ ] memory/context management [ ] guardrails + human checkpoint [ ] an eval suite in
  CI.
- **Ordered steps**:
  1. `.../learning/capstone/code/tools.py` — typed tools plus function-call parsing/dispatch. Verify
     the model selects and the runtime executes the correct tool; `mypy` clean.
  2. `.../learning/capstone/code/agent.py` — a bounded reason-act-observe loop with a step budget,
     stop condition, and step memory. Verify it completes a multi-step task and halts within the
     budget.
  3. `.../learning/capstone/code/mcp/` + guardrails — connect a tool via MCP and gate a consequential
     action behind validation/a human checkpoint. Verify the dangerous tool cannot fire without the
     checkpoint.
  4. `.../learning/capstone/evals/` — a task dataset + scoring, wired into CI. Verify the eval reports
     a score and fails the pipeline when the agent regresses below the bar.
- **Acceptance criteria**: tool calling and the bounded loop work; MCP connects a tool; guardrails
  block ungated consequential actions; the eval suite scores the agent and gates CI on regression; all
  Python is type-annotated and `mypy`-clean.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Papers & articles**

- **ReAct: Synergizing Reasoning and Acting in Language Models** — Shunyu Yao et al. (2022). The
  foundational paper defining the reason-plus-act loop that underlies most modern agent frameworks.
  <https://arxiv.org/abs/2210.03629>
- **Toolformer: Language Models Can Teach Themselves to Use Tools** — Timo Schick et al. (2023).
  Canonical early paper on LLM tool-use/function-calling. <https://arxiv.org/abs/2302.04761>
- **Reflexion: Language Agents with Verbal Reinforcement Learning** — Noah Shinn et al. (2023).
  Influential paper on self-reflective agent loops that improve via linguistic feedback rather than
  weight updates. <https://arxiv.org/abs/2303.11366>
- **Generative Agents: Interactive Simulacra of Human Behavior** — Joon Sung Park et al. (2023), UIST.
  Widely cited multi-agent simulation paper demonstrating believable agent memory, planning, and
  social behavior. <https://arxiv.org/abs/2304.03442>
- **A Survey on Large Language Model based Autonomous Agents** — Lei Wang et al. (2023). Widely cited
  survey providing a unifying framework across the fast-moving agentic AI literature.
  <https://arxiv.org/abs/2308.11432>

---

← Previous: [53 · Creating AI-Powered Apps](./53-creating-ai-powered-apps.md) · Next: [55 · IT / Application Security](./55-it-and-application-security.md) →
