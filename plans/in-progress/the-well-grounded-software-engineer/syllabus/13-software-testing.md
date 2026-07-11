# 13 · Software Testing (By Example, Python + TS)

**prd row**: Pass 1 · First Working Software · By Example · Python + TS · Learn 113 / Drill 213 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: testing as a discipline across both stacks — unit through integration, test doubles, TDD,
and property-based testing folded in. Deep CI wiring cross-refs
[`22-software-engineering-practices`](./22-software-engineering-practices.md). This topic underpins the
Regression Test Mandate the whole repo enforces.

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md) and
  [topic 11 Just Enough TypeScript](./11-just-enough-typescript.md) (examples span both);
  [topic 09 Backend Essentials](./09-backend-essentials.md) provides the app the integration test targets.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with **pytest** + **Hypothesis**;
  **Node.js** with **Vitest**/Jest + **fast-check**; optional mutation tools (mutmut/Stryker).
- **Assumed knowledge**: reading/writing basic Python and TypeScript; running a program from the CLI.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). Re-confirm version pins at authoring.

- 2026-07-12 — verified: **pytest 9.1.1**, **Hypothesis 6.156.6**, **Vitest 4.1.10**, **fast-check 4.9.0**
  — all current/CVE-clean. Pact contract testing is current (`pact-python` latest 2026-05-04; PactV3/V4 +
  Matchers V3). Mutation testing: **`mutmut` 3.6.0** (Python). (pypi.org / npmjs.com)
- 2026-07-12 — verified (CORRECTION): the JS/TS mutation tool is **`@stryker-mutator/core` 9.6.1** — the
  bare `stryker` npm package is abandoned (last publish 7 years ago); reference the scoped package only.
  (npmjs.com)

## Items

- **Why test; test design**; the **test pyramid vs the testing trophy** (both taught, with the
  trade-off).
- **Unit tests**: `pytest` (Python) + Vitest/Jest (TS) from the CLI; arrange–act–assert.
- **Test doubles** (Fowler taxonomy): dummy / stub / spy / mock / fake; when each fits.
- **TDD** (folded in here): red → green → refactor driven end to end on a real example.
- **Property-based testing** (folded in here): **Hypothesis** (Python) + **fast-check** (TS); generative
  thinking, shrinking, invariants over examples.
- **Coverage & its limits**; mutation testing intro (mutmut / Stryker).
- **Integration & e2e intro**; contract testing intro (**Pact**); test-containers intuition.
- **Running the whole suite** and reading reports from the terminal; CI intro (cross-ref practices).

## Worked examples

Colocated under `software-testing/learning/code/`; runnable in both stacks (DD-20/DD-30).

- **beginner** — TDD a small pure function in `pytest`; the same in Vitest.
- **intermediate** — stub/mock a dependency; a Hypothesis property test that surfaces an edge case.
- **advanced** — an integration test against the Backend-Essentials app; reading a coverage + mutation
  score.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take one small feature and build its full test suite across the pyramid — TDD'd unit tests,
  a mocked-dependency test, a property-based test (Hypothesis + fast-check), and an integration test
  against the Backend-Essentials app — with a coverage report read from the CLI.
- **Concepts exercised**: [ ] arrange–act–assert unit tests (both stacks) [ ] a test double (stub/mock)
  [ ] TDD red→green→refactor [ ] a property-based test with shrinking [ ] an integration test [ ] reading
  coverage.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — TDD a pure function: write the failing `pytest`/Vitest test first,
     then implement. Verify the test goes red→green.
  2. Add a stub/mock isolating a dependency. Verify the unit test runs without the real dependency.
  3. Add a Hypothesis (+ fast-check) property test asserting an invariant. Verify it passes and would
     shrink a counterexample (demonstrate on a seeded bug).
  4. Add an integration test hitting the Backend-Essentials endpoints. Verify it passes against the
     running app; read the coverage report.
- **Acceptance criteria**: all tiers green; the property test demonstrably catches a seeded regression;
  coverage report generated and interpreted; the red→green history is shown.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [12 · Frontend Essentials](./12-frontend-essentials.md) · Next: [14 · Security Essentials](./14-security-essentials.md) →
