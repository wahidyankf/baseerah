---
title: "Theme D: Degradation, Latency, and the Launch Decision"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 40
---

Scenarios 34-44 close the loop from Theme A's opening diagnosis: what the product does when the
model itself is unavailable, slow, or unusable (co-16, co-18), why that state must be visible
rather than silent (co-19, echoing Scenario 3), how latency itself is a material the interface can
shape (co-17), and finally the launch decision -- ship, staged-rollout, and rollback criteria
written against a measured distribution rather than a hope (co-21, co-22, co-23). Scenario 36's
artefact is a captioned, WCAG-accessible Mermaid diagram and lives standalone under
`learning/artifacts/`. Scenario 44 closes the theme with a capstone-scale worked scenario spanning
every concept this course teaches, foreshadowing the full capstone dossier.

---

### Worked Scenario 34: Model-Unavailable State

**Context**: Exercises co-16. Nimbus's backend model provider had a 40-minute outage. Before this
scenario's redesign, the frontend simply showed an infinite loading spinner for the entire outage,
with no indication anything was wrong.

**Decision artifact**:

> **Model-unavailable state design**
>
> - **Detection**: the frontend treats three consecutive failed requests, or any single request
>   exceeding 15 seconds with no response, as "provider unavailable."
> - **User-visible state**: the answer area is replaced with a clearly labelled banner -- "Nimbus is
>   temporarily unavailable. Your documents are safe; try again in a few minutes." -- plus a
>   disabled (not hidden) input field, so the user can see the feature still exists and is not
>   permanently broken.
> - **What the state explicitly avoids**: an infinite spinner (implies "still working," which is
>   false), a generic error page (implies something is broken beyond recovery), and silently
>   returning a stale cached answer without labelling it as such.

**Verify**: the design states a concrete detection rule (three failures or a 15-second timeout) and
a specific, honest user-visible message -- satisfying co-16's rule that graceful degradation names a
concrete state, not merely "handle the error somehow."

**Key takeaway**: An unavailable state is not the absence of a design decision -- an undesigned
outage defaults to a spinner or a silent stale answer, both of which mislead the user about what is
actually happening.

**Why It Matters**: This single state is the first, most basic rung of Scenario 35's fallback
hierarchy, and it directly prevents co-19's silent-failure pattern from recurring in the specific
case where the model is not degraded, but entirely absent. Designing this state explicitly also gives engineering a concrete, testable specification to build against, rather than leaving the outage behaviour to whatever the frontend framework happens to do by default when a request never resolves.

---

### Worked Scenario 35: Fallback Hierarchy

**Context**: Exercises co-18 and co-16. Rather than a single binary "working / unavailable" state,
this scenario designs a full ladder of rungs Nimbus falls through as conditions degrade, each with
its own quality and cost.

**Decision artifact**:

| Rung | Condition                                     | Behaviour                                                                                                    | Quality                               |
| ---- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------- |
| 1    | Primary model responds within 5s              | Full structured answer with citations (Scenario 20).                                                         | Best available.                       |
| 2    | Primary model slow (5-15s) or rate-limited    | Falls to a smaller, faster model for the same task, same structured format.                                  | Slightly lower accuracy, still cited. |
| 3    | Both models unavailable, cached answer exists | Serves the most recent cached answer for this exact question, explicitly labelled "cached from [timestamp]." | Stale but labelled, never silent.     |
| 4    | No model available, no cache                  | Deterministic fallback -- a direct link to the raw source document, no generated answer at all.              | Lowest, but honest and never wrong.   |
| 5    | Even the raw-document lookup is unavailable   | Scenario 34's unavailable state.                                                                             | No answer offered.                    |

**Verify**: every rung has a stated, observable trigger condition (a timing threshold, a cache
existence check, a lookup failure) determining exactly when the system falls to it -- satisfying
co-18's rule.

**Key takeaway**: A fallback hierarchy replaces one binary "working or broken" state with five
named, honestly-labelled degrees of service -- each one still usable, in its own way, rather than a
single point of total failure.

**Why It Matters**: This ladder is what turns co-16's abstract commitment into something concretely
buildable one rung at a time, and rung 3's explicit "cached from [timestamp]" labelling is the
direct application of co-19's rule -- a stale answer is fine, as long as it never looks
indistinguishable from a fresh one. Naming every rung in advance also makes an incident review faster -- responders can ask which rung the system was on, and for how long, instead of reconstructing an undocumented, ad hoc degradation path after the fact.

---

### Worked Scenario 36: Degradation Diagram (diagram)

**Context**: Exercises co-18 and co-16. This diagram-medium scenario renders Scenario 35's fallback
hierarchy as a flow with every rung's observable trigger condition made explicit.

> A captioned diagram of the five-rung fallback hierarchy with each rung's observable trigger
> condition -- exercises co-18 and co-16. The full diagram, with its own Verify / Key takeaway / Why
> It Matters, lives at
> [Artifact: Degradation Diagram](/en/learn/courses/product-patterns-for-probabilistic-systems/learning/artifacts/ex-36-degradation-diagram).

**Key takeaway**: A fallback hierarchy is only as trustworthy as its trigger conditions being
genuinely observable -- a rung with a vague or unmeasurable trigger is a rung that will not actually
fire when it should.

**Why It Matters**: This diagram is the reference Scenario 37 uses directly to confirm every rung
surfaces its own identity to the user, and it is the artefact a real on-call engineer would consult
during an actual incident to understand which rung the system is currently operating on. Publishing this diagram alongside the feature's own documentation also lets a new team member understand the full degradation surface in one glance, rather than piecing it together from scattered code comments.

---

### Worked Scenario 37: Fail Visibly, Not Silently

**Context**: Exercises co-19 and co-16. This scenario audits Scenario 35's fallback hierarchy for the
one property co-19 demands above all others: does the user know which rung they are currently on?

**Decision artifact**:

> **Visibility audit -- fallback hierarchy, rung by rung**
>
> - **Rung 1 (full answer)**: no label needed -- this is the expected, default state.
> - **Rung 2 (faster fallback model)**: currently unlabelled. **Gap found**: a user cannot tell rung
>   2's answer apart from rung 1's, even though rung 2's accuracy is measurably lower. **Fix
>   applied**: a small "fast mode" label added to rung 2 answers.
> - **Rung 3 (cached)**: already labelled "cached from [timestamp]" per Scenario 35's design --
>   passes.
> - **Rung 4 (raw document link)**: already visibly distinct -- no generated answer text appears at
>   all, only a link -- passes.
> - **Rung 5 (unavailable)**: already an explicit banner per Scenario 34 -- passes.

**Verify**: the audit checks every one of the five rungs individually against the same rule ("can
the user tell which rung they are on?") and finds and fixes exactly one gap (rung 2) -- satisfying
co-19's rule that a degraded state must never be indistinguishable from a fully healthy one.

**Key takeaway**: A fallback hierarchy can still fail silently at any individual rung even after the
hierarchy itself is designed -- each rung needs its own explicit visibility check, not just the
hierarchy's overall existence.

**Why It Matters**: Rung 2's gap is exactly the kind of silent failure Scenario 3 traced all the way
to a costly user decision -- this audit is the concrete discipline that prevents Scenario 35's good
design from quietly reintroducing the same failure mode at one specific rung. This per-rung discipline should be repeated any time a new rung is added to a fallback hierarchy -- a hierarchy that passed this check once does not stay compliant automatically as the system evolves.

---

### Worked Scenario 38: Latency as Material

**Context**: Exercises co-17. The primary model has already run slow enough to trip Scenario 35's
rung-2 fallback trigger, so Nimbus is running on the fast-mode fallback model (labelled per
Scenario 37) -- and that fallback model's own structured-answer generation genuinely takes 8-12
seconds. This scenario compares three treatments of that same wait on the same slow response.

**Decision artifact**:

| Treatment              | What the user sees during the wait                                                                                            | Perceived-quality result                                                         |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| Bare spinner           | A generic spinning icon, no additional information.                                                                           | Users report the wait as "long" and frequently re-click, assuming a failure.     |
| Streaming              | Answer text appears progressively, field by field, as each is generated.                                                      | Users report the wait as "fast," even though total completion time is unchanged. |
| Honest staged progress | Explicit stage labels -- "Reading documents... Extracting terms... Verifying citations..." -- update as each stage completes. | Users report confidence the system is working, and re-click rate drops sharply.  |

**Verify**: the comparison is drawn from the same underlying 8-12 second fallback-model response
time across all three treatments, isolating the treatment itself as the variable, and reports a
distinct perceived result for each -- satisfying co-17's rule that latency is shaped by treatment,
not only by raw duration.

**Key takeaway**: The actual wait time did not change across any of the three treatments -- only
what filled it did, and that alone was enough to change both perceived speed and user confidence
that the system was working at all.

**Why It Matters**: This is the concrete demonstration of co-17's claim that latency is a design
material, not merely an engineering number to minimize -- streaming and honest staged progress are
both available to Nimbus's product team immediately, independent of any model-speed improvement. Because this treatment is independent of model speed, it is also one of the few latency-related improvements a product team can ship without any coordination from the model provider at all.

---

### Worked Scenario 39: Optimistic State That Lies

**Context**: Exercises co-17 and co-19. An earlier Nimbus revision showed "Reminder scheduled ✓"
immediately upon a user clicking "schedule," before actually confirming the backend action
succeeded -- an optimistic UI pattern applied without accounting for a probabilistic feature's
higher failure rate.

**Decision artifact**:

> **Optimistic-UI failure trace**
>
> - **Design**: "Reminder scheduled ✓" appears the instant the button is clicked, assuming success,
>   with the actual backend confirmation arriving (or failing) up to 6 seconds later.
> - **Failure case**: the backend call failed (a transient error) roughly 2% of the time, but the
>   "scheduled ✓" message never changed to reflect that -- it had already told the user the action
>   succeeded and nothing corrected the record afterward.
> - **Consequence**: 2% of scheduled reminders silently never existed, and users had no reason to
>   suspect it -- they had been told, explicitly, that the action succeeded.
> - **Fix**: the optimistic state now reads "Scheduling..." until backend confirmation actually
>   arrives, then updates to "Scheduled ✓" or a visible failure state -- never claiming success
>   before it is confirmed.

**Verify**: the trace identifies the specific gap (a claimed success shown before actual backend
confirmation) and quantifies the resulting silent-failure rate (2%), satisfying co-17 and co-19's
rule that an optimistic state must not imply a completed action that has not actually completed.

**Key takeaway**: Optimistic UI is a legitimate latency-hiding pattern for actions that succeed
almost always -- but a probabilistic feature's higher failure rate means the pattern must be paired
with an honest correction the moment a claimed success turns out to be false, never applied as if
failure could not happen.

**Why It Matters**: This is co-19's silent-failure pattern recurring in a new place -- not in the
answer's content this time, but in the interface's claim about its own state. The same discipline
(never claim more certainty than actually exists) that governs confidence displays in Theme B
governs status messages here too. Any feature using an optimistic UI pattern should ask specifically what its true failure rate is before adopting it unmodified -- a pattern safe at a 0.1% failure rate is not automatically safe at 2%.

---

### Worked Scenario 40: Ship Criteria From a Distribution

**Context**: Exercises co-21. Nimbus's contract-metadata extraction feature is ready for its first
launch review. This scenario writes ship criteria against the feature's measured eval distribution,
not a single headline number.

**Decision artifact**:

> **Ship-criteria sheet -- Nimbus contract-metadata extraction**
>
> - **Primary threshold**: pass rate ≥ 95% on the 120-case eval set, with a 95% confidence interval
>   no wider than ±3 percentage points (extending `evaluating-ai-output-essentials`'s pass-rate
>   framing with the confidence interval this course's co-21 teaches).
> - **Named acceptable worst case**: of the failing cases, none may be a confidently-wrong answer
>   with no citation attached -- every failing case in the eval set must at minimum have surfaced an
>   uncertainty signal (co-06) or a citation the user could have used to catch it (co-09).
> - **Measured result at this review**: pass rate 96.7%, interval ±2.1 points; zero
>   failing cases lacked a citation.
> - **Verdict**: criteria met, cleared for staged rollout (Scenario 42).

**Verify**: every criterion states a threshold, an interval, and a named acceptable worst case, and
each is checked against the actual measured eval report rather than an assumption -- satisfying
co-21's rule.

**Key takeaway**: A ship decision is a threshold on a distribution plus an explicit, named worst
case -- a single average pass-rate number, without an interval and without a stated worst case, is
not a launch criterion, it is a hope.

**Why It Matters**: This sheet directly reuses `evaluating-ai-output-essentials`'s measurement
discipline and turns it into a product decision -- the exact seam this course's overview names as
its relationship to the evals courses. Writing criteria this way also gives a launch review a concrete, falsifiable basis for its decision -- reviewers can check the sheet against the actual eval report directly, rather than relying on a verbal assurance that it looks good.

---

### Worked Scenario 41: The Worst Case Nobody Named

**Context**: Exercises co-21. This scenario contrasts Scenario 40's ship criteria against an earlier
Nimbus launch that used only an average pass-rate threshold, and shows the failure that threshold
permitted.

**Decision artifact**:

> **Postmortem -- ship criteria that covered the average, not the tail**
>
> - **Original criterion**: "ship if pass rate ≥ 93%." Measured at launch: 94.2% -- criterion met.
> - **What the criterion missed**: of the failing 5.8% of cases, a cluster of them were all the
>   same failure type -- confidently wrong, no citation, on contracts written in a non-English
>   language the eval set under-represented. This cluster was invisible to the single average
>   number, because it was a small enough fraction not to move the headline pass rate below
>   threshold.
> - **Consequence**: a real user relying on a non-English contract received a confidently wrong,
>   uncited answer -- exactly the failure Scenario 40's revised criteria now explicitly forbid in
>   its "named acceptable worst case" clause.
> - **Fix carried into Scenario 40**: the named-worst-case clause exists specifically because of
>   this postmortem.

**Verify**: the postmortem identifies a specific failure cluster the average-only threshold missed
and traces it to a real, named consequence, satisfying co-21's rule that ship criteria must name an
acceptable worst case, not only an average threshold.

**Key takeaway**: A pass-rate average can clear a threshold while hiding a coherent cluster of
failures affecting one specific, real subgroup of users -- naming the acceptable worst case
explicitly is what catches this before launch instead of after.

**Why It Matters**: This postmortem is the direct origin story for the "named acceptable worst case"
clause in Scenario 40's criteria -- ship criteria improve because of exactly this kind of retrospective,
which is why writing a postmortem this specific matters even when, as here, the launch technically
met its stated bar. Any team reviewing a past launch's criteria should ask the same question this postmortem asks -- not just whether the average cleared the bar, but what the failing cases had in common.

---

### Worked Scenario 42: Staged Rollout With Guardrails

**Context**: Exercises co-22. Having cleared Scenario 40's ship criteria, this scenario designs the
exposure ramp and the guardrail metrics that would halt it.

**Decision artifact**:

> **Staged-rollout plan -- Nimbus contract-metadata extraction**
>
> - **Ramp**: 5% of eligible users (day 1-3) → 25% (day 4-7) → 100% (day 8+), each stage gated on
>   the guardrail metrics below staying within threshold for the full prior stage.
> - **Guardrail metrics and halt thresholds**:
>   - Correction-affordance usage rate (Scenario 32) exceeding 8% of answers (baseline in eval:
>     under 3%) halts the ramp immediately.
>   - Any single day's manually-reported incident count exceeding 2 halts the ramp immediately.
>   - Median review time (Scenario 23's timing discipline) exceeding 90 seconds for three
>     consecutive days halts the ramp and triggers a redesign review.
> - **Rationale**: none of these three metrics exist in the eval set -- they are only observable
>   once real, unpredictable production traffic starts flowing through the feature.

**Verify**: every guardrail metric names a specific, real-time-checkable threshold that would halt
the ramp, and none of the three metrics is one the eval set could have measured in advance --
satisfying co-22's rule that staged rollout exists precisely because eval sets do not contain real
traffic.

**Key takeaway**: A staged rollout is not merely "release slowly" -- it is release slowly while
watching specific, pre-agreed metrics that could not have been measured before real users arrived,
each with a concrete threshold that halts the ramp automatically.

**Why It Matters**: The correction-affordance usage-rate guardrail directly reuses Scenario 32's
mechanism -- a spike in corrections during rollout is exactly the kind of signal a pre-launch eval
set cannot produce but real usage immediately can. A staged rollout with well-chosen guardrails also limits the blast radius of any single undetected failure mode -- a problem caught at 5% exposure affects far fewer users than the same problem caught only after a full launch.

---

### Worked Scenario 43: Rollback Criteria Written First

**Context**: Exercises co-23. Before Scenario 42's rollout begins, this scenario writes the
conditions for turning the feature off entirely -- agreed and signed off before any incident
pressure exists.

**Decision artifact**:

> **Rollback-criteria sheet -- agreed before rollout begins**
>
> - **Automatic rollback conditions** (no discussion required, any single condition triggers
>   immediate rollback): any guardrail metric from Scenario 42 breaching its threshold for two
>   consecutive days without a shipped fix; any single incident causing a confirmed, irreversible
>   external action based on a wrong answer (an email sent, per Scenario 27's highest-friction row).
> - **Discussion-required conditions** (rollback considered, decision made within 4 hours by the
>   named on-call product lead): correction-affordance usage rate elevated but not yet breaching
>   Scenario 42's hard threshold; user-reported confusion trending upward without a specific
>   incident.
> - **Sign-off**: Product Lead, Engineering Lead, Legal Ops stakeholder -- all three signed before
>   Scenario 42's rollout began.

**Verify**: the sheet distinguishes automatic-rollback conditions from discussion-required ones,
each stated unambiguously enough to act on without new negotiation, and is signed off before rollout
begins, not during an incident -- satisfying co-23's rule.

**Key takeaway**: Rollback criteria decided during an actual incident are written by people under
pressure to keep the feature live -- writing them down in advance, when nobody has a stake in the
outcome yet, is what makes them genuinely enforceable later.

**Why It Matters**: This sheet is what makes Scenario 11's earlier no-ship decision and this
scenario's rollback decision the same kind of professional judgment, applied at two different
points in a feature's lifecycle -- both require naming, in advance, the conditions under which
shipping is not the right choice. Any team preparing to launch a new probabilistic feature should treat writing this sheet as a prerequisite step, not an optional one -- a launch review that has not produced rollback criteria has not actually finished its own job.

---

### Worked Scenario 44: Capstone-Probabilistic-Feature-Design Dossier

**Context**: Exercises co-01 through co-24. This capstone-scale worked scenario assembles every
pattern this course teaches into one design dossier for a single feature -- Nimbus's
contract-metadata extraction, the running example across Themes A through D -- at the same scale
the full [Capstone](./capstone/overview.md) applies to a different, independently chosen feature.

**Decision artifact**:

> **Design-dossier index -- Nimbus contract-metadata extraction**
>
> 1. **Failure catalogue** (co-01, co-02, co-19): Scenario 1's interface-state audit establishing why
>    a failure catalogue is needed at all, the six failure modes from Scenario 2, and Scenario 3's
>    silent-failure trace as the confidently-wrong path walked to its consequence.
> 2. **Expectation-setting and trust-calibration plan** (co-03, co-04, co-05): Scenario 4's
>    first-contact copy, Scenario 6's trust-calibration diagram, and Scenario 7-8's automation-bias
>    findings.
> 3. **Selective uncertainty and provenance policy** (co-06 through co-10): Scenario 15's
>    numeric-vs-band reading test, Scenario 17's policy, Scenario 18's citation design, Scenario 20's
>    cheap-verification structure, and Scenario 22's accessible treatment, all meeting WCAG AA.
> 4. **Human-review design** (co-11, co-12): Scenario 23's timed review flow and Scenario 25's
>    vigilance-decrement mitigations.
> 5. **Friction, preview, and undo** (co-13 through co-15): Scenario 27's friction table, Scenario
>    29's preview-and-diff, and Scenario 30's complete-undo investment.
> 6. **Fallback hierarchy** (co-16 through co-18): Scenario 35's ladder, Scenario 37's per-rung
>    visibility audit, and Scenario 38's latency-as-material treatment.
> 7. **Scope, ship, guardrail, and rollback criteria** (co-20 through co-23): Scenario 10's
>    narrowed-scope proposal and Scenario 11's no-ship decision, Scenario 40's ship criteria, Scenario
>    42's staged-rollout guardrails, and Scenario 43's rollback sheet.
> 8. **Correction affordance and feedback routing** (co-24): Scenario 32's design and Scenario 33's
>    routing plan into eval regression cases.

**Verify**: every one of the eight dossier sections cites the specific earlier worked scenario it
draws from, and every `co-NN` this course teaches (co-01 through co-24) is traceable to at least one
section -- satisfying this scenario's role as a complete, cross-referenced assembly rather than a
new, unrelated example.

**Key takeaway**: Nothing in this dossier is new -- every section is an assembled reference back to
a specific, already-verified worked scenario, which is exactly what makes a design dossier
trustworthy: a reviewer can trace any single decision back to the concrete evidence that produced
it.

**Why It Matters**: This scenario demonstrates, on the course's own running example, the exact shape
the [Capstone](./capstone/overview.md) asks a learner to produce independently for a feature of
their own choosing -- a complete, traceable design dossier a real launch review could act on. Working through this index before the standalone capstone begins also gives a learner a concrete template to follow -- the same eight-section shape, applied to a new feature of their own choosing.

---

← Previous: [Theme C: Humans in the Loop, Friction, and Recovery](./theme-c-humans-in-the-loop-friction-and-recovery.md) &middot; Next: [Capstone](./capstone/overview.md) →
