---
title: "Theme A: The Wrong Answer Is Coming"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 10
---

Scenarios 1-11 build the case for why a probabilistic feature cannot simply reuse a
success-or-error interface: a confidently wrong answer has nowhere to land in that interface
(co-01), the six user-visible failure modes need distinct designs (co-02), first-contact framing
determines how errors get read later (co-03), and trust must be calibrated rather than assumed
(co-04) against a specific, well-documented failure mode of human attention -- automation bias
(co-05). The theme closes with the two moves available when none of that is enough: narrow the
feature (co-20) or decide not to ship it (co-20, co-15). Every scenario below produces a written
decision artefact; Scenario 6's artefact is a captioned, WCAG-accessible Mermaid diagram and lives
standalone under `learning/artifacts/`.

---

### Worked Scenario 1: The Deterministic Interface Fails

**Context**: Exercises co-01. Nimbus, a document-search assistant, answers "which contract expires
first?" with a single confident sentence, rendered in the exact same text block the product already
uses for a database lookup that is always correct. The lookup's UI has exactly two states: the
answer, or a red error banner if the query fails outright. There is no third state for "this is
probably right, but check it."

**Decision artifact**:

> **Interface state audit -- Nimbus contract-expiry answer**
>
> - **States the current interface supports**: (1) answer shown, styled identically regardless of
>   source; (2) error banner, shown only if the backend call itself fails.
> - **State a confidently-wrong answer lands in**: state (1) -- indistinguishable from a correct
>   database lookup. Nothing in the interface signals that this particular answer came from a model
>   reading unstructured contract text rather than a deterministic query.
> - **Conclusion**: the interface was built for a system that either returns a fact or fails
>   outright. It has no state for "returned something plausible that might be wrong," which is the
>   probabilistic feature's actual, dominant failure mode.

**Verify**: the audit names the exact rendering state a confidently wrong answer lands in, and
confirms no visual, textual, or structural difference exists between that state and a genuinely
correct answer -- satisfying co-01's rule that a deterministic interface has no state for this
failure.

**Key takeaway**: A probabilistic feature's most common failure is not an error state -- it is a
plausible answer with nowhere to land except the interface's "correct" state.

**Why It Matters**: Every later scenario in this theme and the next answers a version of the
question this audit raises: what third state should exist, and what should it show? Skipping this
diagnostic step is how teams ship a probabilistic feature inside an interface that was never
designed to hold it, discovering the gap only after a user has already acted on a wrong answer.

---

### Worked Scenario 2: Catalogue the User-Visible Failures

**Context**: Exercises co-02. Nimbus's support team is asked "how does Nimbus fail?" and initially
answers "sometimes it's wrong" -- a single undifferentiated category that gives design nothing to
work with. This scenario decomposes that into Nimbus's own six observed failure modes.

**Decision artifact**:

| Failure mode                           | User-observable description                                                       | Example                                                                |
| -------------------------------------- | --------------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| Confidently wrong                      | A fluent, specific answer that is factually incorrect, with no hedging language.  | States the wrong contract's expiry date with full sentence confidence. |
| Subtly wrong                           | Mostly correct, with one small but consequential detail altered.                  | Correct expiry date, wrong contract party name.                        |
| Refused                                | Declines to answer at all, with or without an explanation.                        | "I can't answer questions about legal documents."                      |
| Truncated                              | Answer is cut off mid-thought, appearing complete but omitting the ending.        | Lists three of four relevant contracts, stopping abruptly.             |
| Slow                                   | Correct eventually, but the wait itself changes what the user does while waiting. | 12-second response with no progress indication.                        |
| Inconsistent across identical requests | The same question, asked twice, returns two different answers.                    | Re-asking "which expires first" returns a different contract.          |

**Verify**: each of the six rows is distinguishable from the other five by a user with no visibility
into the model's internals -- satisfying co-02's rule that these are six distinct experiences, not
one undifferentiated category.

**Key takeaway**: "Sometimes it's wrong" is not a design input -- six named, user-observable failure
modes are, because each demands a different response.

**Why It Matters**: This catalogue is the reference table the rest of this course keeps returning
to: Scenario 3 traces the silent-failure path for "confidently wrong," Theme B answers "subtly
wrong" and "confidently wrong" with uncertainty and provenance signals, and Theme D answers "slow"
and "inconsistent" with latency treatments and fallback hierarchies. A catalogue this specific is
what turns "make it more reliable" into a set of separately addressable design problems.

---

### Worked Scenario 3: Silent-Failure Walkthrough

**Context**: Exercises co-19 and co-02. Continuing Scenario 2's "confidently wrong" row: this
scenario traces one specific wrong answer -- Nimbus stating the wrong contract's expiry date -- all
the way from the model's output to the point where a user, Priya from Legal Ops, acts on it.

**Decision artifact**:

> **Silent-failure trace -- wrong contract-expiry answer**
>
> 1. Model generates: "The Meridian Supply contract expires first, on March 3." (Actually the
>    Anchor Logistics contract expires first, on February 14 -- a contract the model's retrieval
>    step missed.)
> 2. Interface renders this in the standard answer block -- same font, same colour, same layout as
>    every correct answer Priya has seen before.
> 3. Priya reads the answer, sees no caveat, citation, or confidence indicator anywhere near it.
> 4. Priya schedules the Meridian renewal review for next month and does not schedule Anchor
>    Logistics -- which lapses, unrenewed, on February 14.
> 5. Nothing in steps 1-4 would have caught the error before it caused a real, costly consequence.

**Verify**: the trace reaches Priya's actual decision (scheduling the wrong renewal) and confirms,
step by step, that no point along the path -- rendering, reading, or deciding -- contained anything
that could have caught the error, satisfying co-19's rule.

**Key takeaway**: A silent failure is not defined by the model being wrong -- it is defined by every
downstream step also failing to catch it, because nothing was designed to.

**Why It Matters**: This is the single scenario every other pattern in this course exists to
prevent. Theme B's uncertainty and provenance signals, Theme C's review design, and Theme D's
degradation states are all, in different ways, attempts to insert a catch point somewhere along
this exact five-step path before it reaches a costly, silent conclusion.

---

### Worked Scenario 4: First-Run Expectation-Setting

**Context**: Exercises co-03. Nimbus's onboarding screen currently says "Nimbus finds answers in
your documents instantly." This scenario drafts a replacement that survives the user's first wrong
answer instead of being contradicted by it.

**Decision artifact**:

> **First-contact copy -- before and after**
>
> - **Before**: "Nimbus finds answers in your documents instantly." (Implies certainty and speed as
>   guarantees; a slow or wrong first answer directly contradicts this framing.)
> - **After**: "Nimbus reads your documents and drafts an answer in seconds -- always check the
>   source link before you act on anything it tells you." (States what it does -- drafts, not
>   guarantees -- and names the specific behaviour, checking the source, that survives a wrong
>   answer without feeling like a broken promise.)

**Verify**: re-reading the "after" copy immediately after a hypothetical wrong first answer, the
copy still reads as accurate -- it never claimed certainty, so a wrong answer does not contradict it
-- satisfying co-03's rule.

**Key takeaway**: Honest first-contact copy does not need to undersell the feature -- it needs to
name what the feature actually guarantees, so a later miss does not read as a broken promise.

**Why It Matters**: The gap between what onboarding copy promises and what the feature actually
delivers is repaid, with interest, at the moment of the first visible mistake. Setting the
expectation honestly the first time is far cheaper than repairing trust after an overpromise has
already been contradicted. This same discipline applies beyond onboarding screens -- any first-touch moment (an empty state, a tooltip, a sales conversation) sets the same kind of expectation, and each deserves the same honesty check.

---

### Worked Scenario 5: Overpromise Postmortem

**Context**: Exercises co-03 and co-04. Nimbus's marketing site originally read "Never miss a
contract deadline again -- Nimbus guarantees accurate answers." This scenario critiques that copy
against its first real failure.

**Decision artifact**:

> **Overpromise postmortem -- "Nimbus guarantees accurate answers"**
>
> - **The promise made**: certainty ("guarantees"), framed as a permanent outcome ("never miss ...
>   again").
> - **The first failure**: the Scenario 3 incident -- the wrong expiry date that caused a real
>   contract lapse.
> - **The specific trust damage**: Priya does not conclude "Nimbus made one mistake" -- she
>   concludes "the guarantee was false," because the copy left no room for a single miss to be
>   anything other than a broken promise. She now manually double-checks every Nimbus answer,
>   erasing the time savings the feature exists to provide.
> - **The fix**: replace "guarantees" with a stated verification habit (Scenario 4's "after" copy),
>   which absorbs a miss without contradicting itself.

**Verify**: the postmortem names the specific word or phrase in the original copy that becomes false
the instant a single wrong answer occurs, and traces the concrete behavioural consequence (manual
double-checking of everything) that follows -- satisfying co-03 and co-04's rule that first-contact
framing determines how the first failure is read.

**Key takeaway**: A guarantee is a promise with no room for a single miss -- and a probabilistic
feature will always eventually miss, so the guarantee is the thing that breaks, not the feature.

**Why It Matters**: Overpromising is a common, well-intentioned marketing instinct, and it is
precisely the instinct this course exists to correct. An honest framing (Scenario 4) does not sell
less well because it is less impressive -- it sells sustainably because it survives contact with the
feature's real, ongoing failure rate. Marketing and product teams should treat this postmortem as a template: any existing claim using words like “guarantee,” “always,” or “never” is worth auditing the same way before the feature's next visible miss forces the audit anyway.

---

### Worked Scenario 6: Trust-Calibration Diagram (diagram)

**Context**: Exercises co-04. Trust calibration has two failure quadrants, not one -- over-trust and
blanket distrust -- and both are caused by the interface. This diagram-medium scenario maps actual
reliability against user trust and names concrete interface moves that pull a feature out of each
failure quadrant.

> A captioned diagram mapping actual reliability against user trust, naming the over-trust and
> blanket-distrust failure quadrants and the interface moves that correct each -- exercises co-04.
> The full diagram, with its own Verify / Key takeaway / Why It Matters, lives at
> [Artifact: Trust-Calibration Diagram](/en/learn/courses/product-patterns-for-probabilistic-systems/learning/artifacts/ex-06-trust-calibration-diagram).

**Key takeaway**: Trust calibration is a two-sided problem -- pulling a feature out of blanket
distrust (Scenario 9) uses a different set of interface moves than pulling it out of over-trust
(Scenario 8), and a design that only addresses one leaves the other quadrant unaddressed.

**Why It Matters**: This diagram is the map the rest of Theme A's remaining scenarios (7-9) each
place themselves on -- automation bias (Scenario 7) is the mechanism behind the over-trust
quadrant, and Scenario 9's abandoned feature is a concrete instance of the blanket-distrust
quadrant. Naming both quadrants up front is what keeps later scenarios from reading as unrelated
problems.

---

### Worked Scenario 7: Automation Bias in a Review Flow

**Context**: Exercises co-05 and co-12. A Nimbus reviewer, Marcus, checks fifty auto-drafted
contract summaries in a row. Nimbus's summaries are correct roughly 96% of the time. This scenario
walks Marcus's fifty-item session and identifies where his scrutiny predictably collapses.

**Decision artifact**:

> **Review-session trace -- fifty summaries, 96% baseline accuracy**
>
> - **Items 1-8**: Marcus reads each summary against the source contract carefully; catches one of
>   the ~0.3 expected errors in this range on item 6.
> - **Items 9-25**: Marcus's per-item read time drops by roughly half; he is now skimming for
>   obvious mismatches rather than reading the source contract in full.
> - **Items 26-50**: Marcus is approving summaries in a rhythm, largely on the strength of "the last
>   twenty were all fine" -- the exact reasoning that fails the moment item 41 contains the
>   session's second error, which he approves without catching.
> - **Point of collapse**: roughly item 15-20, where reading behaviour visibly shifts from
>   verification to pattern-matched approval.

**Verify**: the trace names a specific item range (not "eventually" or "at some point") where
Marcus's behaviour measurably changes from full verification to pattern-matched approval --
satisfying co-05 and co-12's rule that the vigilance decrement has an identifiable onset, not just
an eventual, vague presence.

**Key takeaway**: Scrutiny does not decay evenly across a review session -- it drops sharply after
roughly the first dozen items, which is exactly where a review design needs to intervene, not at
item fifty.

**Why It Matters**: A review design that assumes uniform attention across a fifty-item batch is
designing against a false model of human behaviour. Scenario 25 in Theme C returns to this exact
trace to design concrete mitigations (batch size, forced checkpoints) targeted at this specific
collapse point. Knowing the approximate collapse point in advance also lets a team decide, deliberately, whether to shrink batch sizes, insert a break, or accept the risk for lower-stakes review work -- a choice that is only available once the collapse point is actually measured.

---

### Worked Scenario 8: Reliability Makes Scrutiny Worse

**Context**: Exercises co-05. Nimbus's summary accuracy improved from 88% to 96% over two product
quarters. Reviewer approval time per item dropped by 40% over the same period. This scenario
annotates why the counterintuitive direction -- better model, worse scrutiny -- is exactly what
automation-bias research predicts.

**Decision artifact**:

> **Annotated trend -- accuracy up, scrutiny down**
>
> - **Quarter 1 (88% accuracy)**: average review time per item, 45 seconds. Reviewers report
>   "checking every claim against the source."
> - **Quarter 3 (96% accuracy)**: average review time per item, 27 seconds. Reviewers report
>   "mostly scanning for anything that looks obviously off."
> - **Annotation**: this is not reviewers becoming lazier -- it is the predictable consequence of a
>   system becoming more trustworthy on average. The improvement in the headline accuracy number
>   directly causes the erosion in per-item scrutiny, because a system that is right 96% of the time
>   gives a reviewer 24 consecutive correct experiences, on average, between errors -- more than
>   enough to establish a pattern of trust the reviewer then applies uniformly, including to the
>   25th item.

**Verify**: the annotation states the specific causal direction (rising accuracy causes falling
scrutiny, not the reverse) and names the mechanism (a longer average run of correct outputs between
errors) -- satisfying co-05's rule that this direction is counterintuitive but well-documented.

**Key takeaway**: An improving accuracy number is not, by itself, evidence the feature is getting
safer to use unreviewed -- it may simply be moving the failure further out along a run of correct
answers that erodes scrutiny in the meantime.

**Why It Matters**: Teams that treat a rising pass rate as purely good news miss this trade-off
entirely. A product team should expect -- and design against -- review quality degrading as model
quality improves, which is precisely why co-12's vigilance-decrement mitigations (Theme C) are not
optional polish but a load-bearing part of any human-in-the-loop design.

---

### Worked Scenario 9: Blanket-Distrust Case

**Context**: Exercises co-04 and co-06. A different Nimbus rollout, in the Facilities team,
collapsed: two wrong answers in the first week and usage dropped to near zero within a month. This
scenario diagnoses why the interface gave users no way to distinguish a trustworthy answer from an
untrustworthy one.

**Decision artifact**:

> **Blanket-distrust diagnosis -- Facilities rollout**
>
> - **Timeline**: week 1, two wrong answers (a vendor-contact detail and a maintenance-schedule
>   date), both delivered with the same visual treatment as every correct answer before them; week
>   4, daily active users down 94% from week 1.
> - **Root cause**: the interface offered exactly one visual treatment for every answer, correct or
>   not. Once two answers were discovered wrong, users had no basis to distinguish future correct
>   answers from future wrong ones -- so they stopped trusting all of them, which is a rational
>   response to an interface providing no distinguishing signal.
> - **What was missing**: any of co-06's uncertainty signals, co-09's provenance, or co-10's
>   cheap-verification structuring -- any one of which would have given users a basis for
>   case-by-case trust instead of an all-or-nothing verdict.

**Verify**: the diagnosis identifies the specific interface property (a single undifferentiated
visual treatment) responsible for the collapse, and names at least one concrete fix from a later
theme -- satisfying co-04 and co-06's rule that blanket distrust is a design failure, not simply
"users being cautious."

**Key takeaway**: When an interface gives users no way to tell a good answer from a bad one, the
rational user response to a single visible mistake is to stop trusting everything -- not just the
mistake itself.

**Why It Matters**: This is the blanket-distrust quadrant from Scenario 6's diagram made concrete,
and it demonstrates why Theme B's entire uncertainty-and-provenance toolkit exists: not to make the
feature look less confident, but to give users the case-by-case discrimination ability this rollout
never had. The lesson generalizes beyond this one rollout: any feature launched with a single, undifferentiated presentation of every answer is one bad answer away from the same collapse, regardless of how strong its average accuracy actually is.

---

### Worked Scenario 10: Narrow the Feature Instead

**Context**: Exercises co-20. Nimbus's original scope -- "answer any question about any document" --
produces a 78% accuracy rate, too low for Legal Ops to trust unaided. This scenario redesigns the
feature's scope rather than attempting to improve the underlying model.

**Decision artifact**:

> **Scoping proposal -- Nimbus for Legal Ops**
>
> - **Original scope**: free-form Q&A over every document type Legal Ops uses (contracts, policy
>   memos, meeting notes, external correspondence). Measured accuracy: 78%.
> - **Narrowed scope**: structured extraction over one document type only -- contract metadata
>   (parties, dates, renewal terms) from contracts specifically, always paired with a citation to
>   the source clause. Measured accuracy on this narrower task: 97%.
> - **What was excluded**: free-form questions, non-contract document types, and any question
>   requiring synthesis across multiple documents -- all routed to a "not supported yet" state
>   instead of a low-confidence guess.
> - **Why this is the higher-leverage move**: reaching 97% on the narrow task required no model
>   change at all -- only removing every input the model was unreliable on.

**Verify**: the proposal states both the narrowed task's measured accuracy and the specific
capability excluded to reach it -- satisfying co-20's rule that scoping is a concrete trade, not a
vague "let's do less."

**Key takeaway**: A large jump in reliability was available without touching the model at all --
simply by refusing to answer the questions the model was actually bad at.

**Why It Matters**: Scoping is available to a product team on day one, does not require a model
provider's roadmap to cooperate, and is fully within a team's own control -- which is exactly why
co-20 calls it the highest-leverage move, not merely one option among several equally good ones. It is also the move a team can revisit and expand later, once the model itself improves, whereas an interface built around an unreliable broad scope has to be re-architected rather than simply widened when that day comes.

---

### Worked Scenario 11: Decide Not to Ship

**Context**: Exercises co-20 and co-15. A proposed Nimbus feature -- auto-drafting outbound legal
correspondence -- cannot be narrowed to an acceptable failure rate (the task is inherently
open-ended) and any wrong draft that gets sent is effectively irreversible (co-15's undo does not
apply once an email has left the building). This scenario documents the decision not to ship it.

**Decision artifact**:

> **No-ship decision record -- Nimbus auto-drafted outbound correspondence**
>
> - **Scoping attempted**: restricting to routine renewal-reminder emails only. Measured accuracy on
>   even this narrowed task: 81% -- still below the bar Legal accepted for anything leaving the
>   building unreviewed.
> - **Undo attempted**: none exists once an email is sent to an external recipient; there is no
>   equivalent of Scenario 32's correction affordance for a message already delivered.
> - **Decision**: do not ship auto-send. A drastically reduced version -- auto-draft only, with a
>   mandatory human send action -- remains viable and is scoped separately.
> - **Owner and revisit condition**: Product Lead, revisit if narrowed-task accuracy clears 95% in a
>   future model evaluation.

**Verify**: the record states both why scoping failed to reach an acceptable rate and why undo does
not apply to this specific action, and names a concrete revisit condition -- satisfying co-20 and
co-15's rule that a no-ship decision is a reasoned, revisitable conclusion, not an unexplained
refusal.

**Key takeaway**: When neither scoping nor undo can bring a probabilistic feature's risk down to an
acceptable level, the correct product decision is not to ship it -- and writing that decision down,
with a revisit condition, is itself a deliverable, not a failure to deliver.

**Why It Matters**: Recognising this case is as much a part of professional product judgment on
probabilistic features as knowing how to ship one. A team that always finds a way to ship, even when
neither of this course's two strongest levers (scoping, undo) apply, is optimizing for shipping
over the user's actual safety -- and Theme D's launch and rollback criteria (Scenarios 40-43) exist
specifically to make that trade-off visible before it happens by accident.

---

← Previous: [Overview](./overview.md) &middot; Next: [Theme B: Uncertainty, Confidence, and Provenance](./theme-b-uncertainty-confidence-and-provenance.md) →
