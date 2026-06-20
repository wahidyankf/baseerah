# Business Requirements — Web Design Tester Agent

## Business Goal

Complete the live-site **advocate triad** by adding a **design-team advocate** that judges whether a
running website matches its design (mockups + design system + tokens + external design source) and
follows good general design practice — a perspective neither existing tester owns, and neither
automated gates nor the static UI checker assert at runtime.

## Why This Exists (Business Rationale)

The repository already runs two live-site testers, each a deliberate professional lens
[Repo-grounded — `.claude/agents/web-exploratory-tester.md`, `.claude/agents/web-usability-tester.md`]:

- `web-exploratory-tester` (QA/correctness, spec-aware) — _"is it correct?"_
- `web-usability-tester` (end-user comprehension, spec-blind) — _"is it usable?"_

A site can be **correct** and **usable** and still be **off-design**: drifted from its mockups,
ignoring the design tokens at runtime, reinventing components the shared library already provides, or
simply cramped and visually inconsistent. The
[User-Facing Delivery Hardening Convention](../../../repo-governance/development/quality/user-facing-delivery-hardening.md)
exists precisely because a feature once shipped to production bland and off-design while every gate
was green [Repo-grounded]. The two testers do not close this gap:

- exploratory cites _specs_, not the _design system at runtime_;
- usability is _spec-blind and mockup-blind by design_ — it must not read the design intent.

The **static** counterpart, `swe-ui-checker`, reads component _source_ for token/a11y/pattern
compliance and writes audit reports — it never drives a browser, so it cannot catch divergence that
only appears in the **rendered** page (a token overridden by inline style, a mockup not matched after
build, a primitive reinvented in a route the source check did not reach)
[Repo-grounded — `.claude/agents/swe-ui-checker.md` `tools: Read, Glob, Grep, Write, Bash`].

`web-design-tester` is the **runtime design advocate** that closes this gap on demand and makes the
triad complete and symmetric.

## Business Impact

**Pain points addressed**:

- Off-design, token-divergent, or reinvented-component UI reaching production undetected because no
  agent checks _design fidelity on the running page_. [Judgment call — grounded in the
  hardening-convention's documented production incident, not a measured rate.]
- Design-system erosion: bespoke components quietly re-implementing `libs/web-ui` primitives, which
  fragments the design language over time. [Judgment call]
- Cramped / low-density-discipline layouts and visual inconsistency that no correctness or
  comprehension test flags. [Judgment call]

**Expected benefits**:

- On-demand runtime design-fidelity verification, source-attributed (`DWT-###`) and fed into the same
  fix-planning pipeline the other two testers already feed.
- A symmetric, teachable triad: correctness / usability / design — each a separate, nameable lens.
- Reinforcement of the design-system-primitive-reuse and mockup-parity rules from the hardening
  convention, now checkable against the live page.

## Affected Roles

Solo-maintainer repository — no sign-off ceremonies. The roles below are hats the maintainer wears
and the agents/workflows that consume this capability:

- **Maintainer-as-orchestrator** — invokes the triad (or the combined web workflow) and consumes the
  combined fix plan.
- **The combined web workflow** (renamed to `web-ux-test-fixing-planning`) — gains a third delegated
  tester and becomes the near-end round that web-UI feature-change plans must run (expanded Rule 15).
- **`plan-maker`** — promotes the filed `DWT-###` backlog plan into an executable delivery plan, as
  it already does for `EWT-###`/`UWT-###` plans.
- **`swe-ui-*` / `swe-*-dev` families** — consume `findings.md` to drive design fixes.
- **Sibling repos `ose-primer` and `ose-infra`** — receive the topic-identical capability.

## Business-Level Success Metrics

- **Triad completeness** (observable fact): three live-site `tester` agents exist and are registered
  in every catalog surface — `web-exploratory-tester`, `web-usability-tester`, `web-design-tester`.
- **Boundary clarity** (observable fact): the agent definition pins, in prose, the
  `web-design-tester` (runtime) vs `swe-ui-checker` (static-source) line, with no duplicated
  responsibility.
- **Multi-harness parity** (observable check): `npm run validate:sync` and
  `npm run harness:bindings-validation` both pass after the new agent is synced
  [Repo-grounded — `package.json` scripts `validate:sync`, `harness:bindings-validation`].
- **Three-repo parity** (observable fact): the agent + registrations land topic-identically in
  `ose-public`, `ose-primer`, `ose-infra`, with repo-specifics localized — verified per repo's own
  gate.
- **Quality discipline** (qualitative reasoning): the new lens demonstrably reports at least the
  class of defect (mockup divergence, token override at runtime, reinvented primitive, cramped/
  inconsistent layout) that neither sibling tester nor `swe-ui-checker` reports — verified by the
  charter's worked dimensions, not by a fabricated numeric target.
- **Reciprocal triad complementarity** (observable fact): each of the three tester definitions names
  the other two and pins its non-overlapping lens — a checkable property of the agent files.
- **Web-UI-feature-change rule consistency** (observable check): the expanded Rule 15 (three-tester
  near-end round) reads consistently across the hardening convention, `AGENTS.md`, `plan-execution`,
  `plan-maker`, `plan-checker`, and `plan-execution-checker`.

## Business-Scope Non-Goals

- Not a static-source checker (that is `swe-ui-checker`) — no overlap, no duplication.
- Not a correctness tester (that is `web-exploratory-tester`) and not a usability tester (that is
  `web-usability-tester`).
- Not a fixer/deployer — it discovers and files; it never changes the site or the design system.
- No new maker-checker-fixer trio — the triad is three peer _testers_.

## Business Risks & Mitigations

| Risk                                                           | Likelihood | Mitigation                                                                                                                                 |
| -------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Scope creep into `swe-ui-checker`'s static-source territory    | Medium     | Charter pins the runtime-vs-static boundary explicitly; `prd.md` carries a Gherkin scenario asserting the agent does **not** audit source. |
| Three-repo drift (capability lands differently per repo)       | Medium     | Surgical-topic propagation with a localization map (`tech-docs.md`); each repo gated to its own pre-push + CI before the next.             |
| Binding desync (`.opencode`/`.amazonq`/`.codex` mirrors stale) | Low        | Mandatory `npm run generate:bindings` + `validate:sync` + `harness:bindings-validation` step per repo, with a Phase Gate.                  |
| Triad asymmetry (filing format diverges from the two siblings) | Low        | Filing format (README+brd+prd+findings+spec-gaps, `DWT-###`, severity, steps-to-reproduce) modelled verbatim on the sibling testers.       |

## Related Documents

- [`prd.md`](./prd.md) — product requirements and Gherkin acceptance criteria
- [`tech-docs.md`](./tech-docs.md) — architecture, ground-truth sources, registration surfaces
- [User-Facing Delivery Hardening Convention](../../../repo-governance/development/quality/user-facing-delivery-hardening.md)
- [Governance Vendor-Independence Convention](../../../repo-governance/conventions/structure/governance-vendor-independence.md)
