# 53 · Creating AI-Powered Apps (By Example, Python)

**prd row**: Pass 3 · Build for the Real World · By Example · Python · Learn 153 / Drill 253 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: building applications on top of LLMs as an engineer, not a researcher — prompting,
structured output, retrieval-augmented generation (RAG), tool/function calling, the Model Context Protocol
(MCP), agentic loops, and — first-class — evaluation, cost, latency, and safety. Runnable in Python against
a local or mockable model so no paid key is required to learn the shapes (DD-20). Data plumbing builds on
[`37-data-engineering`](./37-data-engineering.md); it is served over a backend from
[`39-backend-at-scale`](./39-backend-at-scale.md).

## Why this exists · the big idea

- **The problem before the solution**: building on an LLM means building on a component that is
  non-deterministic, occasionally wrong with confidence, and attackable through its own input — the usual
  "it returns the right answer" contract no longer holds.
- **Keep-this-if-you-forget-everything**: treat the model as an unreliable subsystem you engineer around —
  ground it with retrieval, constrain it with structured output and tool schemas, and hold it accountable
  with evals, budgets, and injection guards.
- **Big ideas touched**: `determinism-vs-emergence` (probabilistic output is the defining constraint),
  `correctness-vs-pragmatism` (you manage "good enough" with evals and guardrails, not proofs).

## Prerequisites

- **Prior topics**: [topic 11 Backend Essentials](./11-backend-essentials.md) (serving the app + calling
  an API), [topic 4 Just Enough Python](./04-just-enough-python.md), and
  [topic 15 Software Testing](./15-software-testing.md) (the eval/testing mindset for non-deterministic
  output).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with a pinned CVE-clean model/SDK client;
  a **local or mockable model** + a local vector store for RAG so the examples run without a paid key
  (DD-20); no real API keys committed (secrets rule).
- **Assumed knowledge**: calling an HTTP/API from Python (topic 11); functions + JSON (topic 04); writing a
  test/assertion (topic 15).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (TIME-SENSITIVE, re-check at authoring): the current **ratified** Model Context
  Protocol version is **2025-11-25**. A **release candidate for 2026-07-28** is public (stateless core,
  Extensions framework, Tasks, MCP Apps, auth hardening) targeting final publication ~2026-07-28 — inside
  the likely authoring window, so re-verify the MCP spec version immediately before authoring this topic.
  (modelcontextprotocol.io/specification/versioning)
- 2026-07-12 — verified: RAG (chunk/embed/retrieve), tool/function calling, structured output, agentic
  loops, eval harnesses, and prompt-injection guardrails are standard/stable framing as of 2026. Keep
  model-SDK API shapes version-unpinned; re-pull the current SDK surface at authoring time.

## Items

- Prompting as engineering: instructions, few-shot, structured/JSON output, determinism knobs.
- Retrieval-augmented generation: chunking, embeddings, a vector store, retrieve → augment → generate.
- Hybrid retrieval: combining dense (vector) and sparse (BM25/keyword) search with re-ranking, and
  evaluating retrieval quality — beyond naive top-k embeddings.
- Tool / function calling: letting the model call typed functions; validating arguments.
- The Model Context Protocol (MCP): standardized tool/context servers; where it fits.
- Agentic loops: plan → act → observe; guardrails and stopping conditions.
- Engineering concerns (first-class): evaluation harnesses, cost/latency budgets, caching, and safety
  (prompt injection, PII, output validation).

## Worked examples

Colocated under `creating-ai-powered-apps/learning/code/`; each runnable against a local/mockable model
(DD-20/DD-30).

- **beginner** — a structured-output call (JSON schema-validated) + a tiny eval asserting the schema + a
  golden case.
- **intermediate** — a minimal RAG pipeline: chunk + embed a corpus, retrieve top-k, generate a grounded
  answer with citations.
- **advanced** — tool/function calling with argument validation + a bounded agentic loop with a stopping
  condition and an injection guard.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a grounded question-answering app over a local corpus — RAG (chunk/embed/retrieve),
  structured/cited output, a tool-calling step for a live lookup, a bounded agentic loop with guardrails —
  and wrap it in an evaluation harness that scores answer quality, plus explicit cost/latency budgeting and
  a prompt-injection guard, all runnable against a local/mockable model.
- **Concepts exercised**: [ ] structured/validated output [ ] a RAG pipeline with citations [ ] tool/
  function calling with argument validation [ ] a bounded agentic loop with a stop condition [ ] an eval
  harness scoring quality [ ] a cost/latency budget + an injection guard.
- **Ordered steps**:
  1. `.../learning/capstone/code/rag.py` — chunk + embed a local corpus + retrieve top-k + generate a cited
     answer. Verify answers cite retrieved chunks and a schema-validated shape.
  2. Add a tool-calling step (a typed lookup) with argument validation. Verify invalid arguments are
     rejected and the tool result is incorporated.
  3. Wrap in a bounded agentic loop with a stop condition + a prompt-injection guard. Verify the loop
     terminates and an injected instruction in the corpus is not obeyed.
  4. `eval.py` — a golden-set eval scoring answer quality + a cost/latency budget assertion. Verify the eval
     runs, reports scores, and flags a budget breach.
- **Acceptance criteria**: RAG answers are grounded + cited; tool arguments are validated; the agent loop is
  bounded and injection-resistant; the eval harness produces reproducible scores within the stated
  cost/latency budget; no API key is committed.
- **Done bar**: runnable end-to-end (local/mockable model) + web-verified.

## Read more

**Books**

- **Designing Machine Learning Systems** — Chip Huyen (2022). Widely adopted, practitioner-oriented reference for building production ML/AI systems end to end.

**Papers & articles**

- **Attention Is All You Need** — Ashish Vaswani et al. (2017). The paper introducing the Transformer architecture underlying virtually all modern LLMs. <https://arxiv.org/abs/1706.03762>
- **Language Models are Few-Shot Learners** — Tom B. Brown et al. (2020). The GPT-3 paper that established the modern prompting/few-shot paradigm for LLM applications. <https://arxiv.org/abs/2005.14165>
- **Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks** — Patrick Lewis et al. (2020). The paper that named and formalized RAG, now the standard pattern for grounding LLM apps in external data. <https://arxiv.org/abs/2005.11401>
- **OpenAI Prompt Engineering Guide** — OpenAI (ongoing). Widely used, vendor-authoritative practical reference for prompt design and evaluation. <https://platform.openai.com/docs/guides/prompt-engineering>

---

← Previous: [52 · CI/CD & Release Engineering](./52-cicd-and-release-engineering.md) · Next: [54 · Agentic AI](./54-agentic-ai.md) →
