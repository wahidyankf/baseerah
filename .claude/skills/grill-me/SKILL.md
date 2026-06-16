---
name: grill-me
description: >
  Interview the user relentlessly about a plan or design, presenting choices one at a time
  until shared understanding is reached. Resolves every branch of the decision tree. Use
  when the user wants to stress-test a plan, get grilled on their design, or mentions
  "grill me".
---

# Grill Me

Stress-test plans and designs through relentless, structured questioning before implementation
begins.

## When to activate

Activate when:

- User says "grill me", "challenge my plan", "stress-test this", "interrogate my design",
  or any close variant
- A new plan is being created and design decisions remain open
- A design review is requested before committing to implementation

## Process

Interview the user about every aspect of the plan until shared understanding is reached. Walk
down each branch of the decision tree, resolving dependencies one-by-one.

This skill is the canonical implementation of the
[Grilling-With-Options Convention](../../../repo-governance/development/workflow/grilling-with-options.md) —
that convention is the normative source for the format, mechanism, and scope below. Keep them in
sync.

**Rules (HARD — no exceptions):**

1. **Explore the codebase first** — if a question can be answered by reading existing files,
   read them instead of asking. Never ask what a file read can answer.
2. Present **2-4 concrete, mutually-exclusive options** per question, each with a one-sentence
   trade-off specific to this decision (no generic "this is simpler" filler) — open-ended
   questions without options are FORBIDDEN. If you cannot enumerate options, read the codebase
   first (Rule 1) and synthesize them before asking.
3. **Mark exactly one option Recommended** with a one-line rationale grounded in the repo state
   and the user's stated constraints. More than one Recommended is forbidden.
4. **One decision per question.** Tightly-coupled decisions (where one answer constrains the
   other) MAY be batched in a single multi-question prompt; unrelated decisions MUST NOT be
   bundled.
5. The user can always supply an **unlisted write-in answer** — options are a starting point, not
   a cage. Treat a write-in with the same weight as a listed option; if it opens a new branch,
   grill on that branch.
6. **Two standing options on EVERY question** — beyond the 2-4 substantive options, ALWAYS
   surface (a) a free-form **type-your-own (blank state)** path whose answer is whatever the user
   types — explicit, never merely implicit (this is the most common omission) — and (b) a
   **"chat about this"** option that lets the user discuss the branch in prose before deciding.
   With `AskUserQuestion`, the auto-provided free-text "Other" entry is the blank-state type; add
   "Let's chat about this" as an explicit option (keep substantive options ≤3 so it fits the
   4-option cap). When the user picks "chat about this", drop the structured options, talk the
   branch through, then return to a structured question once they are ready to decide.
7. Continue until all branches are resolved — do not stop early.

**Violation of Rule 2 (asking without options) is the most common failure mode.** If you catch
yourself writing a question without listing concrete options, rewrite it with options before
sending. **Dropping the blank-state type option (Rule 6) is the second most common failure** —
every question MUST let the user type their own answer.

## Mechanism — use the AskUserQuestion tool

Grilling MUST use the **`AskUserQuestion` tool** (the harness's native interactive
multiple-choice mechanism), not free-text prose questions. It renders options as selectable
choices and returns a structured answer — eliminating parse ambiguity — and always offers a
free-form "Other" path.

- One `AskUserQuestion` call carries 1–4 questions; use multiple questions only for
  tightly-coupled decision clusters (Rule 4).
- Each question carries 2–4 options (Rule 2); put the Recommended one first and append
  `(Recommended)` to its label with the rationale in its description (Rule 3).
- Every question keeps a standing **"Let's chat about this"** option and relies on the
  auto-provided free-text **"Other"** entry for the blank-state type (Rule 6); use ≤3
  substantive options so the chat option fits within the 4-option cap.

**Fallback only when `AskUserQuestion` is unavailable** (non-interactive harness): use inline
markdown options instead, still satisfying Rules 2–5:

> **[Question]**
>
> - **Option A**: [description] — [trade-off] **(Recommended — [rationale])**
> - **Option B**: [description] — [trade-off]
> - **Other — type your own answer**: free-form write-in; the answer is whatever you type (blank
>   state). Always present.
> - **Chat about this**: talk the decision through before deciding. Always present.

No bare "What do you think about X?" questions. No yes/no questions without an options list.
Present the choices; let the user pick or override.

## After the grilling

When all decision tree branches are resolved:

1. Summarize every decision made and its rationale
2. Confirm shared understanding explicitly
3. Signal readiness to proceed to plan writing or implementation
