---
title: "Overview"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 1
---

This page is the spaced-repetition companion to the Evaluating AI Systems In Depth topic: recall
first, then applied judgment, then a hands-on kata, then a self-check checklist, then
elaborative-interrogation prompts that ask _why_, not just _what_. Every answer is hidden in a
`<details>` block; try each item yourself before opening it.

Every kata below is self-contained and deterministic -- hand-constructed fixtures only, no live
model call, no network, and no dependency on this topic's own worked-example or capstone code.

## Recall Q&A

Twenty-eight short-answer questions, one per concept (`co-01` through `co-28`). Answer from
memory, then check.

**Q1 (co-01 -- error analysis first).** Why must you read failures before choosing a metric,
rather than choosing a metric first and then checking it against the data?

<details>
<summary>Answer</summary>

A metric chosen before reading failures optimizes for whatever it happens to measure, not for what
actually broke. Reading the failures first is what lets the metric target the failure modes that
are actually happening, instead of a generic guess.

</details>

**Q2 (co-02 -- open-coding failures).** Why should the first labels applied to failures be
invented while reading, not drawn from a pre-existing taxonomy?

<details>
<summary>Answer</summary>

Reaching for a pre-built category list quietly imports assumptions about what could go wrong
before the real failures have even been read fully; open-coding lets each failure describe itself
in the analyst's own words first.

</details>

**Q3 (co-03 -- failure taxonomy).** What turns a pile of open codes into a usable taxonomy?

<details>
<summary>Answer</summary>

Clustering the open codes into a small set of named, mutually intelligible failure modes, each
traceable back to the specific open codes that produced it, with an observed frequency attached.

</details>

**Q4 (co-04 -- frequency-weighted prioritization).** Why is frequency alone not enough to decide
which failure mode to fix first?

<details>
<summary>Answer</summary>

A rare-but-catastrophic failure can matter more than a common-but-harmless one; ranking by
frequency times cost, not frequency alone, is what surfaces the failures worth fixing first even
when they are individually rare.

</details>

**Q5 (co-05 -- derived task-specific criteria).** What must be true of a criterion for it to count
as evidence-based rather than speculative?

<details>
<summary>Answer</summary>

It must trace to a failure mode you actually observed in the error-analysis pass; a criterion
invented from a general sense of "good" with no failure behind it is speculation, not evidence.

</details>

**Q6 (co-06 -- criterion operationalization).** What is the test for whether a criterion has been
operationalized enough?

<details>
<summary>Answer</summary>

Two independent human labelers, given the same criterion and the same output, reach the identical
verdict without needing to ask a clarifying question.

</details>

**Q7 (co-07 -- human labeling protocol).** What three things make human labels usable as ground
truth?

<details>
<summary>Answer</summary>

A written labeling guide, independent labelers working without seeing each other's answers, and a
documented disagreement-resolution rule.

</details>

**Q8 (co-08 -- ground-truth set).** Why is a human-labeled ground-truth set the foundation
everything else in this course rests on?

<details>
<summary>Answer</summary>

Every automated scorer and judge is only trustworthy to the extent it agrees with this reference;
without a ground-truth set there is nothing to validate a judge against, so nothing downstream is
checkable.

</details>

**Q9 (co-09 -- LLM-as-judge).** What is an LLM-as-judge, structurally?

<details>
<summary>Answer</summary>

A model prompted to score another model's output against one operationalized criterion, returning
a structured, parseable verdict.

</details>

**Q10 (co-10 -- judge-human agreement is mandatory).** What is a judge's value equal to, and what
happens if that value is never measured?

<details>
<summary>Answer</summary>

A judge's value is exactly its measured agreement with human labels on the same items; unmeasured,
a judge is decoration -- it can still produce confident-sounding verdicts that are never checked
against reality.

</details>

**Q11 (co-11 -- judge scope reliability).** Why can a judge be trustworthy on one criterion and
worthless on another, and why does that matter for how agreement is reported?

<details>
<summary>Answer</summary>

Agreement is a property of a specific criterion, not the judge in the abstract, so agreement must
be measured and reported separately per criterion -- blending it into one number can hide a
criterion where the judge is barely better than chance.

</details>

**Q12 (co-12 -- judge model separation).** Why should the judge never be the same model that
generated the output under evaluation?

<details>
<summary>Answer</summary>

A judge sharing a family resemblance with the model it grades can develop a subtle, correlated
preference for that model's own phrasing or reasoning style, independent of actual correctness --
separation is about correlated blind spots, not fairness.

</details>

**Q13 (co-13 -- judge bias modes).** Name the judge bias modes this course tests for, and why does
each need its own dedicated probe?

<details>
<summary>Answer</summary>

Position bias, verbosity bias, self-preference, and score compression. Each is systematic, not
random noise, so each needs its own targeted probe (for example, swap slot order, or pad a reply)
rather than a single generic bias check.

</details>

**Q14 (co-14 -- pairwise vs. pointwise judging).** What is the genuinely contested trade-off
between pairwise and pointwise judging?

<details>
<summary>Answer</summary>

Pairwise ("which is better") tends to track human preference more closely, but pointwise ("score
this 1-5") is more robust to adversarial manipulation -- pairwise preferences flip in roughly 35%
of adversarial cases versus only 9% for absolute scores.

</details>

**Q15 (co-15 -- rubric design for judges).** What kind of rubric agrees with human judgment best,
and why?

<details>
<summary>Answer</summary>

Short, binary, single-question rubrics -- every additional clause in a long, multi-dimensional
rubric is another place for a judge or human labeler to diverge from another reader's
interpretation.

</details>

**Q16 (co-16 -- judge recalibration).** Why is judge-human agreement re-measured on a schedule
instead of once?

<details>
<summary>Answer</summary>

Agreement decays when the model, prompt, or data distribution changes -- often gradually, through
drift rather than one dramatic event -- so a judge validated once and never re-checked can quietly
become unreliable over time.

</details>

**Q17 (co-17 -- reference-free vs. reference-based).** What's the difference between
reference-based and reference-free scoring, and how does each fail?

<details>
<summary>Answer</summary>

Reference-based scoring compares against a gold answer and fails on a valid alternative phrasing
it wasn't told about; reference-free scoring checks the output against required facts or
properties on its own terms and can miss subtler correctness issues a reference would have caught.

</details>

**Q18 (co-18 -- trajectory evaluation).** For an agent, what is a trajectory, and why is it a
distinct object of evaluation from the final answer?

<details>
<summary>Answer</summary>

The full sequence of tool calls the agent made, in order. It is distinct because a final answer
alone cannot reveal whether the process that produced it was sound -- the same correct answer can
result from a correct process or a lucky guess.

</details>

**Q19 (co-19 -- outcome vs. process scoring).** Why does scoring both outcome and process catch
something neither catches alone?

<details>
<summary>Answer</summary>

A right answer reached by a wrong path is a latent failure -- outcome scoring alone rates it
identically to a properly-verified answer, hiding a failure that has not yet surfaced on a
different input where the shortcut doesn't happen to work.

</details>

**Q20 (co-20 -- multi-step failure attribution).** Why does locating WHICH step failed matter more
than just knowing the trajectory failed?

<details>
<summary>Answer</summary>

Knowing only that a trajectory failed gives an engineer nothing to act on; knowing which specific
step (and, in a multi-agent system, which specific subagent) caused it turns a failure into a
fixable bug report.

</details>

**Q21 (co-21 -- eval dataset construction).** Where should eval cases come from, and why does that
sourcing matter?

<details>
<summary>Answer</summary>

Production traffic, red-team probes, and regression reports, with deliberate coverage of the
failure taxonomy -- sourcing from real, flagged behavior keeps the dataset grounded in what
actually happens rather than what an engineer imagined might happen.

</details>

**Q22 (co-22 -- dataset contamination and leakage).** What is dataset leakage, and why does it
produce misleading scores?

<details>
<summary>Answer</summary>

Cases that leaked into a prompt, a cache, or a model's fine-tuning data produce an artificially
inflated pass on that specific case -- a score that will not transfer to genuinely novel inputs,
however reassuring it looks.

</details>

**Q23 (co-23 -- eval in CI).** What does it mean for an eval suite's result to be "a merge
decision, not a report"?

<details>
<summary>Answer</summary>

The suite doesn't just describe quality after the fact -- its outcome directly blocks or allows a
specific merge, with a reasoned, logged verdict an engineer can act on immediately rather than a
passive number nobody consults.

</details>

**Q24 (co-24 -- regression bar and noise floor).** Why must a regression bar be derived from a
measured noise floor rather than picked as a round number?

<details>
<summary>Answer</summary>

A bar set without reference to the suite's own run-to-run noise produces false positives on
ordinary wobble or false negatives on real regressions; deriving the bar from measured noise (for
example, baseline minus twice the standard deviation) is what makes it defensible.

</details>

**Q25 (co-25 -- cost of evaluation).** Why does an eval suite need its own cost budget, the same
way a test suite needs a runtime budget?

<details>
<summary>Answer</summary>

Judge calls and repeat runs cost real money and wall time, and that cost can creep upward
unnoticed through retry storms or expanding scope -- budgeting and reporting it in the same run as
the pass rate is what keeps it visible before it becomes a surprise bill.

</details>

**Q26 (co-26 -- tiered eval suites).** Why run a fast deterministic tier on every commit and a
full judged tier only at merge, rather than running everything on every commit?

<details>
<summary>Answer</summary>

A judge-using tier on every commit would be prohibitively slow and expensive, while a
deterministic-only tier would miss criteria that genuinely need judge-based checking -- tiering,
mirroring the unit/integration/e2e split, gets both fast feedback and full coverage.

</details>

**Q27 (co-27 -- evals drive improvement).** When does the evaluation loop actually close?

<details>
<summary>Answer</summary>

Only when a failing case routes back into error analysis and produces the next criterion, dataset
case, or fix -- an eval suite that never grows past its original taxonomy will eventually miss the
failure modes that emerge after it was built.

</details>

**Q28 (co-28 -- what evals cannot catch).** What does a well-built eval suite structurally fail to
catch, by construction?

<details>
<summary>Answer</summary>

Novel harms, distribution shift, and anything absent from the dataset -- every criterion in a
well-built suite traces back to an observed failure, which means, by construction, it cannot catch
a pattern nobody has seen yet. An eval suite bounds risk; it does not eliminate it.

</details>

## Applied problems

Eight scenarios spanning the whole topic. Each describes a realistic situation without naming the
specific concept -- decide what applies and why, then check your reasoning against the worked
solution. Every scenario below is invented for this drill and does not reuse any case, fixture, or
domain from the worked examples or the capstone.

**AP1.** A team building an AI meeting-notes assistant notices summaries "feel off" sometimes, so
an engineer immediately writes a ROUGE-style overlap scorer against a handful of reference
summaries and starts tracking it weekly. Three months later the score has climbed steadily, but
users still complain the assistant regularly omits action items. What went wrong, and what should
have happened first?

<details>
<summary>Worked solution</summary>

This is error analysis skipped in favor of a convenient metric (co-01) -- word-overlap similarity
to a reference summary has nothing to do with whether action items are captured, so a team can
optimize it for months while the real, user-reported failure (missed action items) never improves.
The team should have read a batch of real user complaints and summaries first, named "omits action
items" as its own failure mode, and only then chosen or built a metric that actually measures it.

</details>

**AP2.** An engineer writes a criterion for a coding assistant, "the code should be idiomatic and
well-structured," and asks two colleagues to score the same five code samples against it. They
agree on two of five. What's wrong with the criterion, and what would fix it?

<details>
<summary>Worked solution</summary>

The criterion fails operationalization (co-06) -- "idiomatic and well-structured" is a judgment
call with no mechanical check, so two readers can reasonably disagree. An operationalized version
names the specific, checkable properties that decide the verdict (for example, "uses the
standard library's context manager for file handles, and no function exceeds 40 lines"), turning a
subjective impression into something two people would score identically.

</details>

**AP3.** A team builds an LLM-as-judge to grade "does this legal-summary draft correctly state the
contract's termination clause," ships it straight to production, and reports "the judge caught
three bad drafts this week" as evidence it works. Is that evidence sufficient?

<details>
<summary>Worked solution</summary>

No -- an anecdote of catches is not the same as measured agreement (co-10). The judge might just as
easily be missing many real failures or flagging correct drafts as wrong, and nobody would know,
because its verdicts were never compared against human-labeled ground truth on a held-out sample.
The team needs to measure agreement between the judge's verdicts and human labels on a sample of
cases, with a confidence interval, before trusting any of the judge's individual catches.

</details>

**AP4.** A judge is validated once, with one blended agreement number (91%) reported across five
different criteria it grades. Six months later a subtle regression appears on exactly one of the
five criteria, and nobody notices for weeks because the suite's overall dashboard still shows
"judge agreement: 91%." What went wrong in how agreement was reported?

<details>
<summary>Worked solution</summary>

Blending agreement across unrelated criteria into one number hides exactly this kind of
criterion-specific failure (co-11) -- a judge that is 99% reliable on four criteria and 60%
reliable on the fifth can still average to 91%, and the 91% headline gives no way to see which
criterion is the weak one. Agreement should be measured and reported per criterion, so a single
criterion's decay is visible on its own, not averaged away by four healthy ones.

</details>

**AP5.** A team fine-tunes a model, then uses that SAME fine-tuned model, prompted differently, as
the judge for its own outputs. The judge consistently scores the fine-tuned model's answers higher
than a competing model's genuinely better answers on the same questions. What's the likely cause?

<details>
<summary>Worked solution</summary>

This is the self-preference effect from insufficient model separation (co-12) -- a judge sharing
weights and training history with the model it grades can develop a correlated preference for that
model's own phrasing or reasoning patterns, independent of actual correctness. The fix is using a
genuinely different judge model (different family, different training), and then re-measuring
agreement to confirm the self-preference effect is gone, not merely assumed gone.

</details>

**AP6.** An agent completes a multi-step research task and produces a correct final citation. The
team's eval suite only checks the final citation's correctness and reports 100% pass. A later
audit of the full trace shows the agent skipped verifying the source's publication date in three of
ten passing cases, and on a different, unluckier input this exact shortcut later produces a wrong
citation. What did the eval suite miss, and how should it have been built to catch this earlier?

<details>
<summary>Worked solution</summary>

The suite scored outcome only, missing a latent process failure (co-19) -- a correct final citation
can still result from a skipped verification step that happened not to matter on this particular
input. Scoring the trajectory itself (did the agent verify the publication date, not just did it
cite something correct) alongside the outcome, and attributing any process failure to its specific
step (co-20), would have surfaced the skipped-verification pattern before it produced a visibly
wrong citation on a different case.

</details>

**AP7.** A team sets its CI regression bar at a flat "must stay above 90% pass rate," ships a
genuinely neutral prompt tweak, and the suite blocks the merge because that run scored 89%. The
engineer re-runs the identical, unchanged code and gets 91% the second time. What should the team
do differently about how the bar is set?

<details>
<summary>Worked solution</summary>

The bar was set without reference to the suite's own measured noise floor (co-24) -- if repeated,
unchanged runs naturally vary between 89% and 93%, a flat 90% bar blocks roughly half of all
genuinely neutral changes purely on noise. The team should run the unchanged suite several times,
measure its mean and standard deviation, and set the bar below the baseline by a defensible
multiple of that measured standard deviation, so the bar only trips on a change large enough to
exceed ordinary run-to-run noise.

</details>

**AP8.** A team builds its eval dataset entirely from cases its own engineers wrote by hand while
designing the feature, achieves a 98% pass rate, and ships. In production, real users trigger a
failure mode -- an unusual phrasing pattern -- that the suite never once tests, because no engineer
happened to think to write a case like it. Was the dataset badly built?

<details>
<summary>Worked solution</summary>

Partly -- sourcing entirely from engineer-invented cases, rather than real production traffic and
red-team probes (co-21), means the dataset reflects what the team imagined might go wrong, not
what users actually do. But even a well-sourced dataset built this way would still have a
structural limit (co-28): every criterion traces back to an observed failure, so a genuinely novel
failure mode nobody has seen yet -- from either engineers or early production traffic -- remains
invisible by construction, no matter how the dataset was sourced. The fix is both: source cases
from real traffic going forward, and treat every eval suite's silence on novel patterns as an
acknowledged limit, not a guarantee of safety.

</details>

## Code katas

Five self-contained exercises. Every kata runs on hand-constructed, in-memory data using only the
Python 3.13 standard library -- no live model call, no network, and no dependency on this topic's
own worked-example or capstone files, so every kata is runnable anywhere Python 3.13 is installed.

### Kata 1 -- Open-code and cluster a failure batch of your own

Invent eight short "request/reply" pairs of your own (any domain -- a recipe assistant, a travel
planner, whatever you like), where at least three replies are genuinely wrong in some way. For each
wrong reply, write a short open code in your own words describing exactly what went wrong. Then
write `build_taxonomy(open_codes: list[str]) -> dict[str, int]` that clusters your own open codes
into a small number of named modes (you decide the mode names and the matching rule) and counts how
many of your invented failures fall into each. Confirm the mode with the highest count matches what
you expected before running it (co-02, co-03, co-04).

### Kata 2 -- A per-criterion agreement calculator with a confidence interval, from scratch

Write `wilson_interval(successes: int, total: int, z: float = 1.96) -> tuple[float, float, float]`
that returns `(point_estimate, lower_bound, upper_bound)` using only `math.sqrt` from the standard
library -- no external stats package. Construct three of your own criterion-level result sets (each
a `(successes, total)` pair, at least one below 85% agreement), run each through your function, and
confirm the interval narrows as `total` grows for the same point estimate, and that a criterion
scoring below your own chosen threshold is one you would correctly flag for retirement (co-10,
co-11).

### Kata 3 -- A position-bias probe harness, hand-built

Write a mock judge function `judge(reply_a: str, reply_b: str) -> str` that returns `"a"` or
`"b"` for which reply it prefers, using any rule you like EXCEPT one that only depends on content --
deliberately make it favor whichever reply is passed in the `reply_a` slot, to simulate position
bias. Then write `detect_position_bias(judge_fn, reply_x: str, reply_y: str) -> bool` that calls
your judge function once with `(reply_x, reply_y)` and once with the slots swapped
`(reply_y, reply_x)`, and returns `True` if the verdict flips purely from slot order. Confirm your
deliberately-biased judge is correctly detected, then fix the judge function to depend only on
content and confirm the probe now returns `False` (co-13).

### Kata 4 -- A trajectory step-attribution function, from scratch

Invent your own three-step agent trajectory as a `list[tuple[str, bool]]` of `(tool_name,
step_is_correct)` pairs, where exactly one step is deliberately incorrect. Write
`find_first_failing_step(steps: list[tuple[str, bool]]) -> int | None` that returns the 0-based
index of the first incorrect step, or `None` if every step is correct. Then write
`process_score(tool_names: list[str], reference: list[str]) -> bool` comparing your trajectory's
tool-name sequence against a reference sequence you also invent. Construct one trajectory that
passes process scoring and fails outcome scoring (fabricate a simple "outcome" check of your own),
and one that does the reverse -- confirm both combinations are distinguishable through your two
functions together (co-18, co-19, co-20).

### Kata 5 -- Derive a regression bar from a measured noise floor, then gate on it

Invent five of your own "repeated, unchanged run" pass rates (floats between 0 and 1, all close
together, simulating natural run-to-run noise). Write `measure_noise_floor(rates: list[float]) ->
tuple[float, float]` returning `(mean, standard_deviation)` using only `statistics.mean` and
`statistics.stdev`. Write `set_regression_bar(baseline: float, noise_floor: float, multiple: float
= 2.0) -> float` and `run_eval_gate(candidate_rate: float, bar: float) -> bool`. Construct one
candidate rate that should clear your derived bar (a small, noise-sized dip) and one that should
not (a large, clearly-real drop), and confirm your gate function decides each one correctly before
you run it (co-23, co-24).

## Self-check checklist

Confirm each item without checking the learning track first. If you hesitate, that concept needs
another pass.

- [ ] I can explain why a metric chosen before reading failures optimizes for the wrong thing.
      (co-01)
- [ ] I can explain why open codes are invented while reading, not drawn from a pre-existing list.
      (co-02)
- [ ] I can state what turns a pile of open codes into a usable taxonomy. (co-03)
- [ ] I can explain why frequency times cost, not frequency alone, drives prioritization. (co-04)
- [ ] I can state what makes a criterion evidence-based rather than speculative. (co-05)
- [ ] I can state the two-person test for whether a criterion is operationalized enough. (co-06)
- [ ] I can name the three things that make human labels usable as ground truth. (co-07)
- [ ] I can explain why a ground-truth set is the foundation everything else validates against.
      (co-08)
- [ ] I can describe an LLM-as-judge structurally, in one sentence. (co-09)
- [ ] I can state what a judge's value is exactly equal to, and what an unmeasured judge is.
      (co-10)
- [ ] I can explain why agreement is measured and reported per criterion, not blended. (co-11)
- [ ] I can explain why a judge should never be the same model that generated the output. (co-12)
- [ ] I can name all four judge bias modes this course tests for. (co-13)
- [ ] I can state the pairwise-vs-pointwise trade-off, including which side each finding supports.
      (co-14)
- [ ] I can explain why short, binary rubrics agree with humans better than long ones. (co-15)
- [ ] I can explain why judge-human agreement is re-measured on a schedule, not once. (co-16)
- [ ] I can explain how reference-based and reference-free scoring each fail differently. (co-17)
- [ ] I can explain why a trajectory is a distinct object of evaluation from the final answer.
      (co-18)
- [ ] I can explain what a right-answer-wrong-path case is, and why it matters. (co-19)
- [ ] I can explain why knowing the failing STEP matters more than knowing only that it failed.
      (co-20)
- [ ] I can name the three sources this course draws eval cases from, and why sourcing matters.
      (co-21)
- [ ] I can explain what dataset leakage is and why it produces misleading scores. (co-22)
- [ ] I can explain what it means for an eval suite's result to be a merge decision, not a report.
      (co-23)
- [ ] I can explain why a regression bar must be derived from a measured noise floor. (co-24)
- [ ] I can explain why an eval suite needs its own cost budget. (co-25)
- [ ] I can explain why a fast tier and a judged tier are run at different points in CI. (co-26)
- [ ] I can state when the evaluation loop actually closes. (co-27)
- [ ] I can name at least one class of thing a well-built eval suite structurally cannot catch.
      (co-28)

## Elaborative interrogation & self-explanation

Six prompts that ask you to explain WHY, connecting two or more concepts, rather than recall a
single fact. Write your own answer before checking the discussion.

**E1.** Why can't a team skip straight from "we have a rough sense something is wrong" (co-01) to
writing derived criteria (co-05), bypassing the taxonomy step (co-03) in between?

<details>
<summary>Discussion</summary>

A criterion derived directly from a vague sense of wrongness, without first clustering real
failures into named, frequency-counted modes, risks encoding the analyst's first guess rather than
the failure that actually dominates. The taxonomy step is what turns "something feels off" into
"this specific named pattern happens three times as often as any other, so it is worth a
criterion" -- skipping it collapses co-01's discipline of reading first back into exactly the
guess-a-metric-first trap co-01 exists to prevent, just one layer deeper.

</details>

**E2.** Why is a human-labeled ground-truth set (co-08) a strict prerequisite for judge-human
agreement measurement (co-10), rather than something you could substitute with a second judge's
opinion?

<details>
<summary>Discussion</summary>

Measuring one judge's agreement against a second judge tells you only whether two automated
systems agree with each other, which could mean they are both right or both share the same
systematic blind spot -- it says nothing about correctness. A human-labeled ground-truth set is
what anchors "agreement" to an actual, independently-reasoned judgment, which is the only thing
that makes an agreement percentage meaningful evidence rather than a circular comparison between
two unvalidated systems.

</details>

**E3.** Judge model separation (co-12) and judge bias-mode probing (co-13) are both mitigations
against a judge scoring unfairly. Is using a genuinely separate model (co-12) alone sufficient to
rule out every bias mode this course names?

<details>
<summary>Discussion</summary>

No -- model separation specifically addresses self-preference bias, the correlated blind spot a
judge develops toward outputs resembling its own family's style. Position bias (which slot a reply
sits in) and verbosity bias (favoring longer replies) are properties of how the judge processes any
two inputs, regardless of which model produced them, so a genuinely separate judge model can still
exhibit both. Each bias mode needs its own dedicated probe (co-13) precisely because fixing one
does not imply the others are fixed.

</details>

**E4.** Pairwise vs. pointwise judging (co-14) and outcome vs. process scoring (co-19) are two
separate design choices in this course. How does the choice between pairwise and pointwise judging
interact with scoring an agent's trajectory specifically?

<details>
<summary>Discussion</summary>

Trajectory scoring is naturally more pointwise-shaped at the step level -- each step either
matches the sanctioned sequence or does not, which is closer to a binary structural check than a
"which trajectory is better" comparison. But at the OUTCOME level, comparing two candidate
end-to-end runs of the same agent on the same task (for example, after a prompt change) is exactly
where pairwise judging's advantages apply -- "which of these two final answers is better" often
tracks human preference more reliably than two separate pointwise scores. The two concepts combine
rather than compete: process scoring at the step level tends toward deterministic or pointwise
checks, while outcome comparison across candidates is a natural fit for pairwise judging.

</details>

**E5.** A regression bar (co-24) and tiered eval suites (co-26) are both about making a merge
decision practical. Why does the choice to tier a suite change how the regression bar for each
tier should be set?

<details>
<summary>Discussion</summary>

A fast deterministic tier and a judged tier have different noise floors -- a deterministic scorer
produces the identical verdict on the identical input every time, so its noise floor approaches
zero and its bar can sit very close to the baseline. A judged tier's noise floor is nonzero because
the judge itself introduces variance (a different sample, or minor prompt sensitivity, can shift
its verdict slightly run to run), so its regression bar must be set further below its own baseline
to avoid false positives from that judge-specific noise. Treating both tiers with one shared bar
would either make the deterministic tier too lax or the judged tier too trigger-happy.

</details>

**E6.** Eval dataset construction (co-21) sources cases from real production traffic and red-team
probes specifically to ground the dataset in reality. Why does even a dataset built this carefully
still run into the limit co-28 names?

<details>
<summary>Discussion</summary>

Sourcing from real traffic and red-team probes grounds every EXISTING case in something that
genuinely happened or was deliberately, adversarially anticipated -- but it cannot include a case
for a failure mode that has not yet occurred and that no red-team exercise happened to imagine
either. Every criterion in the suite still traces back to an observed failure (co-05's own rule,
applied at the dataset level), which means the suite's coverage is bounded by the union of what has
already been seen and what red-teaming successfully anticipated -- a structurally smaller set than
"everything that could ever go wrong." Good sourcing narrows this gap; it cannot close it entirely,
which is exactly why co-28 names the limit explicitly rather than treating good sourcing as a
complete guarantee.

</details>

---

← Previous: [Capstone](../learning/capstone/overview.md)
