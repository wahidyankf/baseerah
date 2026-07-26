---
title: "Overview"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 1
---

This page is the spaced-repetition companion to Product Patterns for Probabilistic Systems: recall
first, then applied judgment, then hands-on decision-artifact repair, then a checklist to confirm
real automaticity, and finally why/why-not prompts that test whether you can explain the reasoning,
not just produce the artifact. Every answer is hidden in a `<details>` block; try each item yourself
before opening it.

This course has no runnable interpreter -- every "kata" below is a decision-artifact repair drill
instead of a code fix, verified the same observable-property way this course's worked scenarios were
verified (a named field present, a computed threshold met, a trigger condition stated), not by
running anything.

## Recall Q&A

Twenty-four short-answer questions, one per concept (`co-01` through `co-24`). Answer from memory,
then check.

**Q1 (co-01 -- deterministic-interface-assumption).** What model do standard product patterns
encode, and why does a probabilistic feature violate it?

<details>
<summary>Answer</summary>

A success-or-error model. A probabilistic feature succeeds partially, fails confidently, and fails
differently on identical inputs -- none of which fits a two-state (correct / error) design, so the
interface has no state for the feature's actual, dominant failure mode.

</details>

**Q2 (co-02 -- failure-modes-users-experience).** Name the six user-visible failure modes a
probabilistic feature can produce.

<details>
<summary>Answer</summary>

Confidently wrong, subtly wrong, refused, truncated, slow, and inconsistent across identical
requests -- six distinct experiences, each requiring its own design response.

</details>

**Q3 (co-03 -- setting-expectations-before-first-use).** Why does first-contact framing matter more
than it might seem?

<details>
<summary>Answer</summary>

What a feature is told to be on first contact determines how its errors are read for the rest of the
relationship -- an overpromise turns the first mistake into a broken promise; an honest framing
absorbs it.

</details>

**Q4 (co-04 -- trust-calibration).** What are the two failure quadrants trust calibration must avoid,
and who causes them?

<details>
<summary>Answer</summary>

Over-trust (accepting output without scrutiny) and blanket distrust (abandoning the feature after
one bad answer). Both are caused by the interface, not the user.

</details>

**Q5 (co-05 -- automation-bias-and-complacency).** State the counterintuitive relationship between a
system's reliability and how carefully users check its output.

<details>
<summary>Answer</summary>

The better the system gets, the worse the scrutiny becomes -- users under-scrutinize output from a
system that is usually right, and rising reliability worsens this effect rather than fixing it.

</details>

**Q6 (co-06 -- surfacing-uncertainty).** Where must an uncertainty signal be placed to actually work?

<details>
<summary>Answer</summary>

At the moment and location the user could still act on it -- a caveat the user only sees after
acting (a footer, a settings page) carries no functional value regardless of its wording.

</details>

**Q7 (co-07 -- confidence-representation-tradeoffs).** Name two confidence representations and the
trade-off each makes.

<details>
<summary>Answer</summary>

A numeric score is precise but routinely misread as a probability of correctness; a qualitative band
is read more accurately but discards information. Every representation trades precision against
being read correctly.

</details>

**Q8 (co-08 -- confidence-is-not-correctness).** Why is a model's confidence score not the same claim
as "this is probably correct"?

<details>
<summary>Answer</summary>

A confidence score typically measures how sure the model sounds to itself while generating a
sequence of tokens, not whether the underlying factual claim is true -- a model can sound very sure
while retrieving the wrong source entirely.

</details>

**Q9 (co-09 -- provenance-and-citation).** What does a citation give a user that a confidence score
does not?

<details>
<summary>Answer</summary>

A means to verify the claim themselves -- converting "should I trust this?" into "let me check
this," a strictly stronger and more reliable position than trusting an opaque number.

</details>

**Q10 (co-10 -- verifiability-by-design).** What is the single strongest lever on a probabilistic
feature's usability?

<details>
<summary>Answer</summary>

Structuring the output so checking it costs less than producing it -- verifiability is a property of
the output's shape, designed in, not an afterthought.

</details>

**Q11 (co-11 -- human-in-the-loop-as-product).** What test must a human-in-the-loop design pass to
count as "review as product" rather than a stopgap?

<details>
<summary>Answer</summary>

The review step must be measurably faster than doing the underlying work unaided -- timed, not
assumed.

</details>

**Q12 (co-12 -- review-cost-and-the-vigilance-decrement).** What is the vigilance decrement, and why
does it matter for review design?

<details>
<summary>Answer</summary>

A reviewer's accuracy decays with volume and with the system's own reliability. A review design that
ignores this decay is theatre -- it looks like safety and is not, because scrutiny predictably
collapses partway through a review session.

</details>

**Q13 (co-13 -- consequence-scaled-friction).** What two properties should determine an action's
required confirmation level?

<details>
<summary>Answer</summary>

Its reversibility and its blast radius -- not a single uniform policy applied to every action
regardless of consequence.

</details>

**Q14 (co-14 -- preview-and-diff-before-apply).** What does a preview-and-diff design catch that a
plain confirmation dialog cannot?

<details>
<summary>Answer</summary>

A field-level error (like a transposed digit) -- because it shows the exact value that will change,
side by side with its source, rather than merely asking "do you want to do this?"

</details>

**Q15 (co-15 -- undo-and-recovery).** Why is complete undo usually a better investment than a
marginal accuracy improvement?

<details>
<summary>Answer</summary>

Undo's value compounds across every future error the feature will ever produce, while an accuracy
improvement's value is capped and can erode with the next model change -- and undo is often cheaper
to build.

</details>

**Q16 (co-16 -- graceful-degradation).** What must a degradation design name explicitly?

<details>
<summary>Answer</summary>

A concrete, user-visible state for every way the model can fail to respond usefully -- unavailable,
rate-limited, too slow, or returning unusable output -- rather than leaving the case undesigned.

</details>

**Q17 (co-17 -- latency-as-a-design-material).** How can two features with identical raw latency feel
different to a user?

<details>
<summary>Answer</summary>

Through what fills the wait -- a bare spinner, streaming output, or honest staged progress each
produce a different perceived speed and a different level of user confidence that the system is
working, independent of the actual duration.

</details>

**Q18 (co-18 -- fallback-hierarchies).** What must every rung of a fallback hierarchy have?

<details>
<summary>Answer</summary>

An observable trigger condition that determines exactly when the system falls to it -- a vague or
unmeasurable trigger is a rung that will not fire reliably when it should.

</details>

**Q19 (co-19 -- silent-failure-is-the-worst-failure).** Why is a silently wrong answer worse than a
visible error?

<details>
<summary>Answer</summary>

Because the user has no way to know they need to check it -- a wrong answer rendered identically to
a right one removes the user's only chance to catch it before acting.

</details>

**Q20 (co-20 -- scoping-to-what-the-model-can-do).** Why is narrowing a feature's scope usually the
highest-leverage move, above improving the model?

<details>
<summary>Answer</summary>

Scoping is available immediately and fully within a team's own control; model improvement is not
guaranteed and depends on factors outside the product team's control.

</details>

**Q21 (co-21 -- ship-criteria-for-a-distribution).** What three things must a well-formed ship
criterion state?

<details>
<summary>Answer</summary>

A threshold, an interval, and a named acceptable worst case -- a single average pass-rate number
alone is not a launch criterion.

</details>

**Q22 (co-22 -- staged-rollout-and-guardrail-metrics).** Why does staged rollout exist even after a
feature clears its ship criteria?

<details>
<summary>Answer</summary>

Because an eval set, however good, does not contain the real, unpredictable production traffic that
can still break a feature -- staged rollout watches pre-agreed guardrail metrics that only become
observable once real users arrive.

</details>

**Q23 (co-23 -- rollback-criteria-agreed-in-advance).** Why must rollback criteria be written before
launch rather than during an incident?

<details>
<summary>Answer</summary>

Criteria decided during an incident are written by people under pressure to keep the feature live --
writing them down in advance, when nobody has a stake in the outcome yet, is what makes them
genuinely enforceable.

</details>

**Q24 (co-24 -- feedback-capture-as-a-product-surface).** What two jobs does a correction affordance
do at once?

<details>
<summary>Answer</summary>

It recovers the current user immediately, and it captures a structured record that feeds the next
error-analysis pass -- a correction that is applied and discarded wastes the signal that would
prevent the same failure recurring.

</details>

## Applied scenarios

Ten self-contained scenarios. Everything needed is in the prompt; decide which concept and which
move applies, then check.

**AS1.** A team ships a document-summarization feature with a single visual treatment for every
summary, correct or not. Two weeks in, a wrong summary causes a real problem, and usage drops 80%
within a month. Diagnose the failure and name the fix.

<details>
<summary>Answer</summary>

This is co-04's blanket-distrust quadrant: with no way to distinguish a trustworthy summary from an
untrustworthy one, the rational user response to one visible mistake is to stop trusting all of
them. The fix is co-06's selective uncertainty signal or co-09's provenance -- either gives users a
case-by-case basis for trust instead of an all-or-nothing verdict.

</details>

**AS2.** A reviewer's approval time drops from 40 seconds to 18 seconds per item as a model's
accuracy improves from 85% to 94% over two quarters. A product manager calls this "obviously good
news." What is missing from that read?

<details>
<summary>Answer</summary>

co-05: rising accuracy causing falling review time is the predictable signature of automation bias,
not simply efficiency. The improving headline number may be moving the eventual failure further out
along a longer run of correct answers, eroding scrutiny in the meantime -- this needs a
vigilance-decrement mitigation (co-12), not an unqualified celebration.

</details>

**AS3.** A team debates showing "Confidence: 87%" versus "Confidence: High" on a medical-symptom
triage assistant. Which risk from co-07 is most severe in this specific context, and why?

<details>
<summary>Answer</summary>

The numeric score's risk -- being misread as a probability of correctness -- is most severe here,
because a user reading "87%" as "87% likely correct" on a health-related recommendation could make a
consequential decision based on a false precision the score never actually claimed (co-08).

</details>

**AS4.** A code-review assistant's suggested diff is shown only as a text description ("adds error
handling to the parse function") rather than an actual diff view. A reviewer approves a change that
turns out to alter unrelated logic. What pattern was missing?

<details>
<summary>Answer</summary>

co-14's preview-and-diff-before-apply: a text description does not show the exact change, so it
cannot catch an unrelated alteration the way an actual line-by-line diff would have.

</details>

**AS5.** A team builds undo for a probabilistic feature's tag-suggestion action but only reverses
the tag itself, not a triggered downstream notification. A user's colleague acts on the stale
notification after the user clicks "undo." What co-15 rule was violated, and how?

<details>
<summary>Answer</summary>

Undo must be complete, not partial. Reversing the tag while leaving the notification's effect intact
creates a false belief that the whole action was undone -- worse than no undo, because it produces
unearned confidence.

</details>

**AS6.** A feature's model provider has a partial outage: half of requests succeed normally, half
time out. The frontend currently shows a single infinite spinner for every request. What is missing,
and what should replace it?

<details>
<summary>Answer</summary>

A degradation design (co-16) with an observable trigger (co-18) -- a timeout-based detection rule
that switches the timed-out half of requests to an explicit, honest state (a fallback rung or a
labelled unavailable message), rather than leaving every request in an indistinguishable spinner
state regardless of whether it will ever resolve.

</details>

**AS7.** A launch review approves a feature because its average pass rate is 94%, above the 93%
bar. Three months later, a coherent cluster of failures affecting one specific user subgroup causes
a real incident, even though the average pass rate never dropped. What was missing from the ship
criteria?

<details>
<summary>Answer</summary>

A named acceptable worst case (co-21). An average-only threshold can hide a coherent failure cluster
small enough not to move the headline number, which is exactly the gap a stated worst-case clause is
designed to catch before launch.

</details>

**AS8.** During a live incident, a VP argues for keeping a probabilistic feature running "just a
little longer" to gather more data, despite pre-agreed rollback criteria being met. What should
happen, and why does this situation illustrate co-23's core argument?

<details>
<summary>Answer</summary>

The pre-agreed rollback criteria should be honored as written -- this is precisely the scenario
co-23 anticipates: rollback decisions made in the heat of an incident are systematically biased
toward staying live, which is why the criteria were written and signed off in advance, when nobody
had a stake in the outcome yet.

</details>

**AS9.** A team narrows a broad, unreliable feature to a specific sub-task and reaches 96% accuracy
with zero model changes. A stakeholder asks "why didn't we just improve the model instead?" What is
the honest answer, per co-20?

<details>
<summary>Answer</summary>

Scoping was available immediately and entirely within the team's own control; model improvement
depends on factors (a provider's roadmap, training data, research timelines) the team does not
control and cannot guarantee. Reaching for the harder, less controllable lever first would have
delayed a fix that was available on day one.

</details>

**AS10.** A correction affordance lets users fix a wrong answer in place, and the fix is applied
immediately to what the user sees. Six months later, an error-analysis review finds the exact same
failure pattern recurring at the same rate as when the feature launched. What was missing?

<details>
<summary>Answer</summary>

co-24's routing requirement: fixing the current user's view recovers that one user but, if the
correction is not logged and routed into a systematic weekly review or the eval dataset, the signal
that would have prevented recurrence is discarded every time.

</details>

## Decision-artifact repair drills

Six before/after drills. Each gives a flawed, self-contained mocked artifact -- spot and fix the
flaw yourself, then compare against the model fix and its root-cause explanation.

### Drill 1 -- uncertainty signal buried in a footer

**Before**: "AI-generated content may contain errors." -- shown once, in 10px grey text, at the very
bottom of the page, below the citation links and the feedback prompt.

<details>
<summary>Model fix and root cause</summary>

**After**: an inline badge directly beside any answer flagged low-confidence, positioned before the
user's next likely action.

**Root cause**: co-06 requires an uncertainty signal to sit between the answer and the user's next
action. A footer disclaimer fails this regardless of its wording, because most users act on the
answer before ever scrolling to see it.

</details>

### Drill 2 -- a confidence badge that implies correctness

**Before**: "94% Accurate" badge shown beside a model-generated answer, where 94 is the model's raw
token-generation confidence score.

<details>
<summary>Model fix and root cause</summary>

**After**: "Nimbus's self-assessed certainty: High" -- relabelled to avoid the word "accurate," and
calibrated against held-out eval accuracy per confidence bucket rather than displaying the raw model
score directly.

**Root cause**: co-08 -- a raw confidence score measures how sure the model sounds to itself, not
whether the claim is true. Labelling it "accurate" manufactures unearned trust in exactly the
confidently-wrong case this course's worked scenarios trace repeatedly.

</details>

### Drill 3 -- friction applied uniformly

**Before**: every action a feature can take -- viewing a summary, saving a draft, sending an
external email -- requires clicking through the identical "Are you sure?" confirmation modal.

<details>
<summary>Model fix and root cause</summary>

**After**: a friction table mapping each action's confirmation level to its own reversibility and
blast radius -- no confirmation for viewing or saving; a full preview-and-diff plus explicit
confirmation only for the irreversible, high-blast-radius external email.

**Root cause**: co-13 -- uniform friction trains users to click through the modal reflexively,
which erases its protective value for the one action that genuinely needed it.

</details>

### Drill 4 -- an undo that leaves a side effect intact

**Before**: "Undo" reverses a tag change but not the notification that change had already triggered
to a colleague.

<details>
<summary>Model fix and root cause</summary>

**After**: undo either reverses every effect (including sending a correction message to the
notified colleague) or the interface explicitly states which specific effects it could not reverse.

**Root cause**: co-15 -- a partial undo is worse than none, because it creates a false belief that
the entire action was reversed.

</details>

### Drill 5 -- a fallback hierarchy with an unlabelled rung

**Before**: a five-rung fallback hierarchy where rung 2 (a faster, less accurate backup model)
renders its answers with the exact same visual treatment as rung 1's full-accuracy answers.

<details>
<summary>Model fix and root cause</summary>

**After**: rung 2's answers carry a small "fast mode" label distinguishing them from rung 1's.

**Root cause**: co-19 -- a fallback hierarchy can still fail silently at any individual rung, even
after the hierarchy itself is well-designed; each rung needs its own explicit visibility check.

</details>

### Drill 6 -- ship criteria with no named worst case

**Before**: "Ship if pass rate ≥ 93%." No interval stated; no worst-case clause.

<details>
<summary>Model fix and root cause</summary>

**After**: "Ship if pass rate ≥ 95% with a 95% confidence interval no wider than ±3 points, AND no
failing case is a confidently-wrong answer with no citation attached."

**Root cause**: co-21 -- a single average threshold with no interval and no named worst case can be
cleared by a launch that hides a coherent, damaging failure cluster small enough not to move the
headline number.

</details>

## Self-check checklist

Confirm each item without checking the concepts page first. If you hesitate, that concept needs
another pass.

- [ ] I can name a probabilistic feature's dominant failure mode and explain why a
      success-or-error interface has no state for it. (co-01)
- [ ] I can catalogue a feature's six user-visible failure modes with a distinguishable,
      user-observable description for each. (co-02)
- [ ] I can draft first-contact copy that survives a feature's first visible mistake without
      reading as a broken promise. (co-03)
- [ ] I can name concrete interface moves that pull a feature out of both the over-trust and
      blanket-distrust quadrants. (co-04)
- [ ] I can identify the point in a review sequence where scrutiny predictably collapses, and
      explain why improving reliability worsens the effect. (co-05)
- [ ] I can place an uncertainty signal at the exact point a user could still act on it. (co-06)
- [ ] I can name, for a given confidence representation, both what it communicates and what it
      risks being misread as. (co-07)
- [ ] I can identify where a design implies confidence equals correctness and propose language that
      does not. (co-08)
- [ ] I can design a citation that a user can practically follow to independently verify a claim.
      (co-09)
- [ ] I can restructure a free-text output into a shape that is measurably cheaper to verify. (co-10)
- [ ] I can design a human-review step and confirm, with a timing measurement, that it is faster
      than the unaided task. (co-11)
- [ ] I can name a concrete vigilance-decrement mitigation and state its own cost. (co-12)
- [ ] I can build a friction table where every action's confirmation level is justified by its own
      reversibility and blast radius. (co-13)
- [ ] I can design a preview-and-diff that shows an exact field-level change, not a summary
      description. (co-14)
- [ ] I can design undo that fully reverses every effect of an action, with no partial state left
      behind. (co-15)
- [ ] I can name a concrete, user-visible state for each of unavailable, rate-limited, too slow, and
      unusable output. (co-16)
- [ ] I can compare latency treatments and identify which reduces perceived wait without implying a
      result that has not arrived. (co-17)
- [ ] I can build a fallback hierarchy where every rung has an observable trigger condition. (co-18)
- [ ] I can trace a plausible wrong answer through an interface to a user's decision and confirm
      nothing along the path would have caught it. (co-19)
- [ ] I can state a narrowed feature's measured failure rate and the specific capability excluded to
      reach it. (co-20)
- [ ] I can write a ship criterion that states a threshold, an interval, and a named acceptable
      worst case. (co-21)
- [ ] I can name guardrail metrics for a staged rollout that could not have been measured from an
      eval set alone. (co-22)
- [ ] I can write rollback criteria unambiguous enough to act on during a real incident without
      renegotiation. (co-23)
- [ ] I can design a correction affordance that both recovers the current user and routes the
      correction into later error analysis. (co-24)
- [ ] I can explain, in one sentence, why the interface is where a stochastic component meets a
      human who expects determinism. (`determinism-vs-emergence`)
- [ ] I can explain, in one sentence, why this course's product goal is a recoverable workflow
      rather than a correct one. (`correctness-vs-pragmatism`)
- [ ] I can explain, in one sentence, why a confidence indicator is a lossy summary a user reads as
      a promise. (`abstraction-and-its-cost`)

## Elaborative interrogation and self-explanation

Six why/why-not prompts, tied to this course's three cross-cutting big-idea tags. Answer each in
your own words before opening the model explanation.

**E1 (`determinism-vs-emergence`).** Why does this course insist the interface, specifically, is
"where" a stochastic component meets a human who expects determinism -- rather than saying the
model itself is the problem?

<details>
<summary>Model explanation</summary>

The model's stochasticity is a fixed property this course does not attempt to change. The interface
is the layer a product team actually controls, and it is where a user's deterministic expectations
(one question, one reliable answer) collide with the model's actual behaviour (partial success,
confident wrong answers, inconsistency across identical requests). Locating the problem at the
interface, not the model, is what makes this course's patterns actionable without requiring a
better model first.

</details>

**E2 (`determinism-vs-emergence`).** co-19 calls silent failure "the worst failure." Why is this
framed as a determinism-vs-emergence problem rather than simply an accuracy problem?

<details>
<summary>Model explanation</summary>

A silent failure is not primarily about how often the model is wrong -- it is about the interface
rendering an emergent, uncertain output using the visual language of a deterministic, certain one.
Even a highly accurate model produces silent failures if its rare wrong answers look identical to
its common right ones; the fix (uncertainty signals, provenance, visible degradation states) is
squarely an interface response to the determinism/emergence mismatch, not a model-accuracy fix.

</details>

**E3 (`correctness-vs-pragmatism`).** Why does this course describe undo (co-15) as usually a
"better investment" than a marginal accuracy improvement, rather than treating accuracy as always
the priority?

<details>
<summary>Model explanation</summary>

Pursuing correctness (higher accuracy) has diminishing, uncertain, and non-compounding returns --
each percentage point is harder to gain than the last, and gains can erode with the next model
change. Pragmatism (investing in undo) accepts that errors will keep happening and instead makes
every future error cheap, a return that compounds across the feature's entire lifetime. The course
consistently prefers the pragmatic, compounding investment over the harder, capped one.

</details>

**E4 (`correctness-vs-pragmatism`).** Scenario 11's "decide not to ship" case chooses not to release
a feature even after real engineering effort was invested. Why is this a pragmatic decision rather
than a failure to find a correct solution?

<details>
<summary>Model explanation</summary>

The pragmatic frame asks "is this workflow recoverable," not "is this feature perfect." When neither
scoping nor undo can bring a feature's risk down to something a workflow can absorb, shipping it
anyway optimizes for having shipped something over the user's actual safety -- exactly the trade
this course warns against. Recognizing and documenting that case is itself the pragmatic, correct
professional move, not evidence the team failed to find a solution.

</details>

**E5 (`abstraction-and-its-cost`).** Why does this course call a confidence indicator "a lossy
summary of a distribution" rather than simply "a number the user can trust or not"?

<details>
<summary>Model explanation</summary>

A confidence score is an abstraction over a much richer underlying reality -- the full distribution
of possible outputs the model could have produced for that input, and the specific reasons any one
of them might be wrong. Collapsing that distribution into a single number necessarily discards
information, and the user has no way to see what was discarded -- they read the number as if it
were the whole picture, which is exactly the abstraction's cost co-08 names directly.

</details>

**E6 (`abstraction-and-its-cost`).** co-10's verifiability-by-design pattern restructures free text
into per-field, cited output. How does this connect to abstraction-and-its-cost, and what "cost" does
it reduce?

<details>
<summary>Model explanation</summary>

A free-text paragraph is itself a lossy abstraction over the underlying evidence the model drew
from -- checking it requires reconstructing which claims came from where. A structured, per-field,
per-citation format reduces that abstraction's cost by making the mapping from claim to evidence
explicit and cheap to traverse, rather than asking the user to reverse-engineer it from prose. The
abstraction is not eliminated -- the output is still a summary -- but its verification cost is
sharply reduced.

</details>

---

← Previous: [Capstone](../learning/capstone/overview.md)
