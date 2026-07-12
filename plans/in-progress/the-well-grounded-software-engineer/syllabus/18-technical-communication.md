# 18 · Technical Communication (Annotated-concept, ‡ no-code)

**prd row**: Pass 1 · Core Foundations · Annotated-concept · ‡ no-code · Learn 118 / Drill 218 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: writing that moves work forward — RFCs, ADRs, and design docs; pull-request
descriptions; incident write-ups; and reader-first structure. Pulled early despite spanning the whole
curriculum because communication compounds: every later topic is easier to learn and to ship when you
can write about it clearly. `‡`: no code — the deliverables are documents, and the acceptance bar is a
reader-review pass rather than a compiler.

## Why this exists · the big idea

- **The problem before the solution**: a good decision no one can find, follow, or trust dies in a
  hallway conversation; undocumented context becomes tribal knowledge that walks out the door with the
  person who held it.
- **Keep-this-if-you-forget-everything**: write for the reader's question, not your discovery order —
  lead with the decision and the "why", put the evidence under it, and make the document skimmable in
  thirty seconds.
- **Big ideas touched**: `correctness-vs-pragmatism` (a design doc's job is a decision that ships and
  holds, not an exhaustive treatise — capture the trade-off and move), `coupling-vs-cohesion` (a
  well-scoped ADR/RFC keeps one decision and its rationale together and cross-links rather than
  inlining everything, so documents change independently).

## Prerequisites

- **Prior topics**: [topic 9 Project Management](./09-project-management.md).
- **Tools & environment**: a plain-text/Markdown workflow in version control; an ADR/RFC template; a
  diagramming tool (a C4-style context/container view) for architecture; Neovim/VSCode with Markdown
  and spellcheck (DD-17). No runtime.
- **Assumed knowledge**: how work is scoped and tracked (topic 09); enough of the domain you're
  writing about to have a defensible point of view.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the referenced conventions are stable and correctly unpinned — RFC 2119
  keyword semantics (clarified by RFC 8174), Michael Nygard's ADR format, and the C4 model are all
  current and notation-independent. There are no version numbers to pin.
- 2026-07-12 — verified (GAP for plan owner): this is a no-code topic, so the "runnable" acceptance bar
  is reframed as a reader-review/comprehension pass — there is nothing to execute, and the shipped text
  reflects that.

## Items

- Reader-first structure: BLUF ("bottom line up front"), the inverted pyramid, and the thirty-second
  skim test.
- The design doc / RFC: problem, context, options considered, decision, trade-offs, and open
  questions.
- The ADR: capturing one architecture decision, its status, and its consequences, checked in next to
  the code it governs.
- The pull-request description: what changed, why, how it was verified, and what a reviewer should look
  at first.
- The incident write-up / blameless postmortem: timeline, impact, root cause, and follow-ups without
  finger-pointing.
- Diagrams as communication: a C4-style context/container view, and when a diagram beats prose.
- Editing: cutting hedging and filler, using MUST/SHOULD/MAY precisely (RFC 2119), and matching
  register to audience.

## Tensions & trade-offs — when NOT to reach for this

- **More words is not more clear**: an exhaustive doc no one reads is worse than a one-page decision
  they act on — over-documentation carries a real maintenance and attention cost.
- **Docs drift from reality**: an ADR checked in beside the code and dated stays trustworthy; a design
  doc in a wiki nobody updates becomes a confident lie. Prefer close-to-code, dated, immutable-decision
  formats over living prose you must constantly police.
- **When NOT to write the long form**: a reversible, low-stakes decision doesn't need an RFC — a
  comment or a PR description is proportionate. Reserve heavyweight docs for decisions that are
  expensive to reverse or that many people must align on.

## Lineage — why it beat the alternative

- Engineering teams learned the hard way that architecture lived only in senior engineers' heads and
  eroded with every re-org. The IETF's RFC tradition showed that durable, referenceable design writing
  scales a distributed community; Nygard's ADRs (2011) shrank that idea to a decision-sized,
  version-controlled unit that lives with the code; the C4 model gave architecture a lightweight,
  notation-independent picture. These beat "big up-front design documents" because they are cheap to
  write, cheap to find, and scoped to change independently. The habit compounds across the curriculum —
  every judgment topic's trade-off is worth more once it's written down — and it feeds directly into
  the decision records that [`42-software-architecture`](./42-software-architecture.md) formalizes.

## Worked examples

Colocated under `technical-communication/learning/`; each is a real document you draft and revise —
the "runnable" check is a reader-review pass against a rubric, not a compiler (DD-20/DD-30).

- **beginner** — rewrite a rambling status update into a BLUF-structured, skimmable note; then write a
  tight PR description for a real change.
- **intermediate** — write an ADR and a short RFC for a genuine design choice, including options
  considered and the decisive trade-off.
- **advanced** — produce a blameless incident write-up (timeline → impact → root cause → follow-ups)
  plus a C4 context diagram for the system involved.

## Capstone spec — intra-topic (subject → full deliverable set)

- **Goal**: document one real decision and one real incident to a professional bar — an RFC/ADR a peer
  can act on without a meeting, and a blameless postmortem a stranger can follow — proving your writing
  moves work forward.
- **Concepts exercised**: [ ] BLUF/reader-first structure [ ] an ADR (decision + status +
  consequences) [ ] an RFC with options + trade-off [ ] a PR description [ ] a blameless incident
  write-up [ ] a C4-style diagram.
- **Ordered steps**:
  1. `.../learning/capstone/adr/` and `.../rfc.md` — the decision, options, trade-off, and open
     questions. Verify a peer reviewer can restate the decision and its rationale from the document
     alone.
  2. `.../pr-description.md` — a real change described (what / why / how-verified / where-to-look).
     Verify a reviewer knows where to start within thirty seconds.
  3. `.../postmortem.md` + `.../context.md` (C4) — timeline, impact, root cause, follow-ups, and a
     context diagram. Verify there is no blame language and every follow-up has an owner.
- **Acceptance criteria**: each document passes a reader-review rubric (skimmable, decision-first, no
  unexplained jargon); the postmortem is blameless and actionable; the diagram matches the prose.
- **Done bar**: reader-review-verified end-to-end + web-verified.

## Read more

**Books**

- **On Writing Well** — William Zinsser (7th ed.). Classic guide to clear, humane nonfiction prose,
  widely adapted for technical writing.
- **The Elements of Style** — Strunk, White (4th ed., 1999). Foundational terse guide to clarity and
  grammar.
- **Docs for Developers: An Engineer's Field Guide to Technical Writing** — Bhatti, Corleissen,
  Lambourne, Nunez, Waterhouse (2021). Current engineer-authored playbook for READMEs, API docs, and
  doc systems.

**Papers & articles**

- **RFC 2119: Key words for use in RFCs** — S. Bradner (1997, clarified by RFC 8174, 2017). Defines
  MUST/SHOULD/MAY used across specifications. <https://www.rfc-editor.org/rfc/rfc2119>
- **C4 model** — Simon Brown (maintained). Lean, notation-independent standard for visualizing
  architecture at context/container/component/code. <https://c4model.com/>

---

← Previous: [17 · Security Essentials](./17-security-essentials.md) · Next: [19 · Computer Science Foundations](./19-computer-science-foundations.md) →
