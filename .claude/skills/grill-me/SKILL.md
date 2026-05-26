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

**Rules (HARD — no exceptions):**

1. Ask questions **one at a time** — never bundle multiple questions in one message
2. **EVERY question MUST present 2-4 concrete options** with trade-off descriptions — open-ended
   questions without options are FORBIDDEN. If you cannot enumerate options, read the codebase
   first (Rule 4) and synthesize them before asking.
3. **Mark the recommended option** clearly, e.g. `**(Recommended)**`
4. **Explore the codebase first** — if a question can be answered by reading existing files,
   read them instead of asking
5. Continue until all branches are resolved

**Tool preference**: When operating in a Claude Code context, use the `AskUserQuestion` tool for
each question. The interactive multi-choice UI shows the user exactly which options are available
and lets them select with a single click. Fall back to the markdown format below only when
`AskUserQuestion` is unavailable.

## Question format (markdown fallback)

When `AskUserQuestion` is not available, structure each question like this:

> **[Question]**
>
> - **Option A**: [description] — [trade-off]
> - **Option B**: [description] — [trade-off] **(Recommended)**
> - **Option C**: [description] — [trade-off]
>
> **Recommendation**: Option B because [specific reason grounded in this context].

## After the grilling

When all decision tree branches are resolved:

1. Summarize every decision made and its rationale
2. Confirm shared understanding explicitly
3. Signal readiness to proceed to plan writing or implementation
