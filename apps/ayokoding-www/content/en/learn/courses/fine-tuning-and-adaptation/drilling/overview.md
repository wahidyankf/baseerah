---
title: "Overview"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 1
---

This page is the spaced-repetition companion to the Fine-Tuning & Adaptation topic: recall first,
then applied judgment, then a hands-on kata, then a self-check checklist, then
elaborative-interrogation prompts that ask _why_, not just _what_. Every answer is hidden in a
`<details>` block; try each item yourself before opening it.

Every kata below is self-contained and deterministic -- hand-constructed fixtures only, no live
model call, no network, and no dependency on this topic's own worked-example or capstone code.

## Recall Q&A

Thirty-two short-answer questions, one per concept (`co-01` through `co-32`). Answer from memory,
then check.

**Q1 (co-01 -- behaviour, not knowledge).** Why is fine-tuning described as reliable for behaviour
but unreliable for knowledge?

<details>
<summary>Answer</summary>

Fine-tuning changes the model's learned weights toward a training distribution's form, style, and
task behaviour, which shows up reliably as consistent output shape. It does not append verifiable
facts the way retrieval does -- any fact baked into weights is frozen at training time and cannot
cite itself, so using it to install knowledge is both unreliable (memorized facts blur or hallucinate)
and expensive (stale the moment training ends).

</details>

**Q2 (co-02 -- the knowledge-injection mistake).** What is the single most common and costly misuse
of fine-tuning, and why does it fail?

<details>
<summary>Answer</summary>

Fine-tuning a model specifically to teach it new facts. It fails because facts baked into weights are
frozen the moment training completes -- the model is stale immediately, cannot cite a source, and any
subsequent fact change requires retraining, while retrieval solves the same problem freshly, cheaply,
and with citations.

</details>

**Q3 (co-03 -- exhaust prompting first).** Why must prompting be attempted and measured before
fine-tuning is even considered?

<details>
<summary>Answer</summary>

Because a substantial share of quality gaps close with better instructions, few-shot examples, and
enforced structure -- at zero training cost and with instant iteration. Skipping straight to
fine-tuning without measuring prompting's own ceiling risks paying training cost for a gap a prompt
change would have closed for free.

</details>

**Q4 (co-04 -- exhaust retrieval first).** If the gap is "the model does not know X," what is almost
always the right answer, and why not fine-tuning?

<details>
<summary>Answer</summary>

Retrieval. It supplies current, citable, updatable facts without ever touching the model's weights,
whereas fine-tuning on facts produces a model stale from the moment training ends and unable to cite
where an answer came from.

</details>

**Q5 (co-05 -- exhaust scoping first).** What does "scoping closes the gap" mean, and why check it
before fine-tuning?

<details>
<summary>Answer</summary>

Narrowing the task to a smaller, more specific version of what is being asked is frequently enough to
bring an unadapted base model's performance up to an acceptable level, at zero training cost. Checking
scoping first avoids paying an adaptation's cost to solve a problem that was really "the task was too
broad," not "the model needs new weights."

</details>

**Q6 (co-06 -- the decision procedure).** What five things must a decision gate confirm, in order,
before any training begins?

<details>
<summary>Answer</summary>

A measured gap; that prompting, retrieval, and scoping have each been genuinely attempted and still
fall short; that the gap is behaviour-shaped, not knowledge-shaped; that the needed data is actually
obtainable; and that evaluation is possible before any training work starts.

</details>

**Q7 (co-07 -- legitimate fine-tuning cases).** Name the five case types the decision gate is
expected to actually pass.

<details>
<summary>Answer</summary>

A consistent output format instructions cannot reliably enforce; a domain register or style with no
compact textual description; proprietary task behaviour that cannot be described in a prompt;
latency or cost reduction via a smaller adapted model; and tool-use patterns the base model handles
poorly.

</details>

**Q8 (co-08 -- the cost nobody budgets).** Beyond compute, what cost categories does a real
fine-tuning budget have to include?

<details>
<summary>Answer</summary>

Data labour (collecting, authoring, and auditing the dataset), training compute, evaluation, and --
easy to forget -- the standing maintenance obligation: the model must be re-adapted whenever its base
changes, a recurring cost, not a one-time one.

</details>

**Q9 (co-09 -- supervised fine-tuning).** What does supervised fine-tuning actually train the model
to do?

<details>
<summary>Answer</summary>

Produce the target output given the input, by training directly on paired instruction/response
examples -- the model learns to reproduce the mapping demonstrated in the dataset.

</details>

**Q10 (co-10 -- dataset is the work).** What is the single most under-weighted fact in fine-tuning
practice, per this course?

<details>
<summary>Answer</summary>

That outcome quality is determined by the dataset far more than by any training hyperparameter --
teams that spend their effort tuning learning rate and epochs while under-investing in dataset
curation are optimizing the wrong lever.

</details>

**Q11 (co-11 -- quality over quantity).** Why can a few hundred consistent examples beat tens of
thousands of noisy ones?

<details>
<summary>Answer</summary>

Noisy examples teach the model a blurred, inconsistent version of the target behaviour, and more
noisy volume just reinforces that blur faster. A small, consistent, on-distribution set gives the
model a clean, learnable signal with nothing to average against.

</details>

**Q12 (co-12 -- dataset consistency).** Why is dataset inconsistency described as "invisible until
evaluation"?

<details>
<summary>Answer</summary>

Two conflicting examples in a dataset do not raise any error during training -- the loss just settles
on an averaged, inconsistent behaviour. The only way to catch it is to look, either with an explicit
consistency audit before training or by noticing the model's own inconsistent behaviour at eval time,
after the cost is already spent.

</details>

**Q13 (co-13 -- dataset sourcing).** Name the three dataset-sourcing strategies this course teaches,
and one distinguishing property of each.

<details>
<summary>Answer</summary>

Production traffic (reflects real usage but is biased toward whatever traffic already happens most),
expert authoring (high quality but expensive per example), and synthetic generation (fast and cheap
but bounded by the generating model's own errors).

</details>

**Q14 (co-14 -- synthetic data and its limits).** What is the fundamental limit of synthetic training
data?

<details>
<summary>Answer</summary>

It is bounded by the teacher model's own correctness -- any error the teacher makes propagates
silently into the generated dataset, and the student trained on that data inherits the same error
with no warning sign.

</details>

**Q15 (co-15 -- train/validation/test discipline).** What does a held-out split actually protect
against?

<details>
<summary>Answer</summary>

Memorization masquerading as improvement. Without a genuinely held-out split, a reported quality gain
could just mean the model memorized its own training data rather than generalized -- the split is
what makes "did this help" a falsifiable question.

</details>

**Q16 (co-16 -- data leakage).** Why does one leaked evaluation case invalidate the whole eval, not
just that one score?

<details>
<summary>Answer</summary>

A leaked case means the model may have memorized the exact answer rather than generalized to it --
that inflated single case's pass makes the aggregate pass rate untrustworthy as a signal, since you
can no longer tell how much of the improvement is real generalization versus memorized
answer-matching.

</details>

**Q17 (co-17 -- full fine-tuning).** What is full fine-tuning, and what is its central risk?

<details>
<summary>Answer</summary>

Updating every parameter in the model. It is the most powerful and most expensive option, and --
because nothing is held frozen -- the most prone to degrading capability the fine-tune was never
meant to touch.

</details>

**Q18 (co-18 -- parameter-efficient fine-tuning).** What trade does parameter-efficient fine-tuning
make, and why does it usually win?

<details>
<summary>Answer</summary>

It trains a small set of added parameters while freezing the base model entirely, capturing most of a
full fine-tune's benefit at a fraction of the compute and artefact size, with far less collateral
damage to capability outside the training distribution -- for nearly every application case, this
trade is the right one.

</details>

**Q19 (co-19 -- low-rank adaptation).** What does LoRA actually inject into the model, and what stays
untouched?

<details>
<summary>Answer</summary>

Trainable low-rank matrices injected into the model's layers; the base model's own weights are left
completely untouched -- only the small added matrices are ever updated during training.

</details>

**Q20 (co-20 -- rank and capacity).** What does an adapter's rank actually control, and what is the
trade-off in raising it?

<details>
<summary>Answer</summary>

Rank bounds how much behaviour change the adapter can express -- a capacity knob. Raising it can
capture more complex behaviour, but it also grows the adapter's size and its overfitting risk on a
small dataset, so rank should be chosen from a measured sweep, never defaulted.

</details>

**Q21 (co-21 -- adapters are composable artefacts).** What operational property makes adapters an
advantage over full fine-tuning, independent of training cost?

<details>
<summary>Answer</summary>

Adapters are small, versionable, swappable, and independently serveable against a shared base --
several task-specific behaviours can be hot-swapped in and out of one loaded base model, something a
full fine-tune's own separate whole-model checkpoints cannot do this cheaply.

</details>

**Q22 (co-22 -- catastrophic forgetting).** Why is catastrophic forgetting described as invisible
unless deliberately measured?

<details>
<summary>Answer</summary>

A target-task eval alone only checks the capability the fine-tune was meant to change -- it says
nothing about capability outside that scope. Forgetting only becomes visible once a dedicated
regression suite, covering untouched capability, is run and its score is compared against the base.

</details>

**Q23 (co-23 -- overfitting in fine-tuning).** Why does training loss fail to reveal overfitting on a
small fine-tuning dataset?

<details>
<summary>Answer</summary>

Training loss is computed only on training data, so it keeps improving as the model increasingly
memorizes that exact data -- it has no visibility into held-out performance, which can peak early and
then decline while loss keeps looking better and better.

</details>

**Q24 (co-24 -- hyperparameters that matter).** Which hyperparameters dominate fine-tuning outcomes,
and what is true of the rest?

<details>
<summary>Answer</summary>

Learning rate, epoch count, and adapter rank dominate; most other configuration knobs are noise
relative to dataset quality -- sweeping the wrong knobs while the dataset itself is noisy will not
recover the result a clean dataset would have given for free.

</details>

**Q25 (co-25 -- evaluate against the base).** What is the only meaningful measure of whether a
fine-tune helped?

<details>
<summary>Answer</summary>

A measured, paired comparison against the unadapted base model -- on the target task AND on a
regression suite covering untouched capability -- never a single retrospective claim or two isolated
pass-rate numbers reported in isolation.

</details>

**Q26 (co-26 -- regression suite for forgetting).** What must a regression suite's cases specifically
NOT include, and why?

<details>
<summary>Answer</summary>

The fine-tune's own target task. A regression suite is built entirely from capabilities the fine-tune
was never meant to change -- that is what makes catastrophic forgetting (co-22) detectable rather
than merely assumed absent.

</details>

**Q27 (co-27 -- distillation).** What does distillation train a smaller model to do, and for what
reason?

<details>
<summary>Answer</summary>

Reproduce a larger teacher model's behaviour, pursued for latency and cost reduction -- never for a
capability gain the base model itself could not already provide.

</details>

**Q28 (co-28 -- distillation's limits).** Why can a distilled student never exceed its teacher?

<details>
<summary>Answer</summary>

The student is trained to reproduce the teacher's own outputs, so it inherits the teacher's errors
along with its correct answers, and its ceiling is bounded by however good the teacher actually is on
its own measured, held-out eval -- not by an assumed or quoted-from-elsewhere figure.

</details>

**Q29 (co-29 -- serving an adapted model).** What operational concerns does serving an adapter
introduce that a base-only deployment does not have?

<details>
<summary>Answer</summary>

Loading, swapping, and routing between adapters against the serving stack, with real memory and
routing implications -- several adapters sharing one loaded base changes the deployment's own memory
footprint and requires a mechanism for finding and attaching the right adapter to the right request.

</details>

**Q30 (co-30 -- the maintenance obligation).** What is pinned about a fine-tuned adapter, and what
happens when that pin is broken?

<details>
<summary>Answer</summary>

The adapter is pinned to the exact base-model version it was trained against. When the deployed base
is upgraded past that pin, the SAME adapter becomes incompatible and must be re-adapted -- this is a
recurring, owned cost, not a one-time training expense.

</details>

**Q31 (co-31 -- licensing and data rights).** When must a base model's licence and training data's
rights be verified, and why does the timing matter?

<details>
<summary>Answer</summary>

Before training begins, not after. Verifying afterward means a licence or rights problem is
discovered only once the cost of curation and training has already been sunk -- and per this course's
own capstone, the same check applies again to any REPLACEMENT candidate considered at retirement time,
not only at the original training decision.

</details>

**Q32 (co-32 -- when to undo a fine-tune).** What makes retiring an adapter a normal, healthy outcome
rather than a failure?

<details>
<summary>Answer</summary>

Adapters age against better alternatives the same way any production artefact does -- a better base
model or a retrieval solution can eventually beat a fine-tuned adapter on both quality and cost.
Planning for that outcome, and retiring the adapter once evidenced, is the discipline this course asks
for, not an admission that the original adaptation was a mistake.

</details>

## Applied problems

Eight scenarios spanning the whole topic. Each describes a realistic situation without naming the
specific concept -- decide what applies and why, then check your reasoning against the worked
solution. Every scenario below is invented for this drill and does not reuse any function, fixture,
or domain from the 75 worked examples or the capstone.

**AP1.** A fintech team spends six weeks fine-tuning a model to answer questions about their current
fee schedule, which changes every quarter. Three months after shipping, support tickets show the bot
confidently quoting last quarter's now-outdated fees. What was the underlying mistake, and what is
the fix?

<details>
<summary>Worked solution</summary>

This is the knowledge-injection mistake (co-02) -- fees are facts, not behaviour, and baking them
into weights freezes them the moment training ends. The fix is retrieval (co-04): serve the fee
schedule from a live, updatable source instead of training it into the model, so a quarterly fee
change never requires a retrain.

</details>

**AP2.** A team wants their support bot's replies to always end with a one-line "next steps" summary.
An engineer jumps straight to collecting a training dataset and fine-tuning for this, without trying
anything else first. A reviewer flags the plan before training starts. What did the reviewer catch?

<details>
<summary>Worked solution</summary>

The decision procedure (co-06) was skipped -- specifically, prompting (co-03) was never genuinely
attempted and measured. A "always end with a one-line summary" instruction is exactly the kind of
format requirement enforced instructions and structured output handle at zero training cost; jumping
to fine-tuning without measuring the prompting alternative first violates the gate's own ordering.

</details>

**AP3.** A team assembles a fine-tuning dataset by scraping five years of internal support macros,
written by dozens of different agents with no shared style guide, some instructing "always apologize
first" and others "never apologize, just solve it." The dataset is 4,000 examples. What is wrong with
this dataset before any training begins?

<details>
<summary>Worked solution</summary>

The dataset likely fails a consistency audit (co-12) -- conflicting macros teach the model
contradictory versions of the same behaviour, and the model will learn an averaged, inconsistent
result that no single macro author intended. Size (4,000 examples) does not fix this; the fix is an
explicit consistency audit BEFORE training, not more volume (co-10, co-11).

</details>

**AP4.** A team reports a fine-tuned model scoring 99% on their eval and celebrates. A later audit
finds that twelve of the fifteen eval cases were accidentally copy-pasted into the training file
during dataset assembly. What is the actual state of this eval, and what should the team do?

<details>
<summary>Worked solution</summary>

This is data leakage (co-16) -- twelve of fifteen eval cases were memorized during training rather
than genuinely generalized to, so the 99% score is not trustworthy evidence of anything. The fix is
to rebuild a genuinely disjoint eval set (co-15), re-run the comparison against the base, and treat
the original 99% figure as void, not as a starting point to adjust from.

</details>

**AP5.** An engineer, unsure what adapter rank to use, picks the largest rank the training library
allows "to be safe." Training loss looks excellent throughout the run. On a held-out set the model
performs worse than a much smaller adapter would have. What went wrong, and what's the general fix?

<details>
<summary>Worked solution</summary>

Two compounding issues: an oversized rank on a small dataset overfits (co-20), and training loss --
computed only on training data -- never revealed it (co-23). The fix is to run a rank sweep against
held-out validation (not training loss) and choose the rank where the marginal validation gain
flattens, rather than defaulting to the largest available option.

</details>

**AP6.** A team fine-tunes an adapter that raises a sentiment-classification pass rate by 15 points.
Two weeks after shipping, a separate feature that shares the same served model starts giving garbled
arithmetic answers it used to get right. The team's only eval was the sentiment classification task
itself. What should have caught this before shipping?

<details>
<summary>Worked solution</summary>

A regression suite covering capabilities the fine-tune was never meant to touch (co-26), run
alongside the target-task eval, would have caught this as catastrophic forgetting (co-22) before
shipping. A single target-task eval, however good its own result, cannot see damage outside its own
scope -- co-25's mandate is both signals together, never one alone.

</details>

**AP7.** A team distills their large production model into a smaller one for latency reasons, and is
surprised and disappointed when the distilled student does not beat the large teacher on any quality
metric. They conclude distillation "doesn't work." What is the actual misunderstanding?

<details>
<summary>Worked solution</summary>

Distillation is a cost and latency optimization, never a quality technique (co-27) -- the student is
trained to reproduce the teacher's own behaviour and inherits its errors, so it can approach but never
exceed the teacher's measured ceiling (co-28). The team's disappointment reflects a wrong expectation,
not a failed technique; the correct comparison is latency and cost saved against a small, accepted
quality cost, not a quality win.

</details>

**AP8.** A platform team needs a single enterprise customer's support bot to use a slightly more
formal tone than every other customer. An engineer proposes standing up an entirely separate,
fully fine-tuned model deployment just for that one customer. A colleague objects. What's the
colleague's likely alternative, and why is it better here?

<details>
<summary>Worked solution</summary>

Train a small adapter for the tone difference and hot-swap it in against the SAME shared base
(co-21, co-29) -- adapters are small, composable, and independently serveable, so this one customer's
tone need not require a whole separate model deployment. A full standalone deployment is far more
expensive to run and maintain for a behaviour difference this narrow.

</details>

## Code katas

Five self-contained exercises. Every kata runs on hand-constructed, in-memory data using only the
Python 3.13 standard library -- no live model call, no network, and no dependency on this topic's own
worked-example or capstone files, so every kata is runnable anywhere Python 3.13 is installed.

### Kata 1 -- Write the five-condition decision gate from scratch

Write `decision_gate(gap_measured: bool, prompting_closed_it: bool, retrieval_closed_it: bool,
scoping_closed_it: bool, behaviour_shaped: bool, data_obtainable: bool, eval_possible: bool) -> tuple[str,
str]` returning `("go", "reason")` only when the gap is measured, none of prompting/retrieval/scoping
closed it, the gap is behaviour-shaped, data is obtainable, and evaluation is possible -- otherwise
`("no-go", "reason")` naming the FIRST condition that failed. Construct three scenarios of your own --
one that reaches go, one that fails on "behaviour-shaped," one that fails because prompting alone
already closed the gap -- and confirm each returns the verdict you predicted (co-06).

### Kata 2 -- A dataset consistency-conflict finder, hand-built

Write `find_conflicts(cases: list[tuple[str, str, str]]) -> list[tuple[str, str, str]]` where each
case is `(case_id, instruction, response)`, returning every case whose instruction matches an earlier
case's instruction but with a DIFFERENT response. Construct a six-case dataset of your own with
exactly two planted conflicts, and confirm your function returns exactly those two conflicting cases,
not the ones they conflict with (co-12).

### Kata 3 -- Rank selection from a sweep, from scratch

Write `select_rank(sweep: dict[int, float], plateau_threshold: float) -> int` that returns the
smallest rank in `sweep` beyond which no later rank improves the score by more than
`plateau_threshold`. Construct your own rank sweep of at least five ranks with a clear plateau point,
and confirm your function selects the rank you predicted before running it, then confirm it would
select a DIFFERENT (larger) rank if you lower `plateau_threshold` to a stricter value (co-20).

### Kata 4 -- The exact binomial sign test, from scratch

Implement `sign_test_p_value(b: int, c: int) -> float` using `math.comb`, matching this topic's own
Example 68 and capstone Step 4. Confirm it returns `1.0` for `b=c=0`, confirm `sign_test_p_value(6, 1)`
does NOT clear a `0.05` threshold while `sign_test_p_value(24, 4)` -- the same ratio, four times the
sample -- does, and explain in one sentence why sample size changed the verdict even though the ratio
did not (co-25).

### Kata 5 -- A minimal adapter-swap server, from scratch

Write a small class with `load_adapter(name: str, base_id: str) -> None` (raising if `base_id`
doesn't match the server's own loaded base) and `switch_to(name: str) -> None` (raising if the
adapter isn't loaded), plus an `active` property. Load two adapters pinned to the SAME base, switch
between them, and confirm `active` updates correctly; then attempt to load a THIRD adapter pinned to
a DIFFERENT base and confirm it raises rather than silently attaching (co-21, co-30).

## Self-check checklist

Confirm each item without checking the learning track first. If you hesitate, that concept needs
another pass.

- [ ] I can explain why fine-tuning is reliable for behaviour but unreliable for knowledge. (co-01)
- [ ] I can explain why fine-tuning to inject facts produces a model stale the moment training ends.
      (co-02)
- [ ] I can explain why prompting must be genuinely attempted and measured before fine-tuning is
      considered. (co-03)
- [ ] I can explain why retrieval, not fine-tuning, is almost always the right answer to "the model
      does not know X." (co-04)
- [ ] I can explain what "scoping closes the gap" means and why it is checked before fine-tuning.
      (co-05)
- [ ] I can name the five things the decision gate must confirm, in order, before training begins.
      (co-06)
- [ ] I can name the five legitimate fine-tuning case types the gate is expected to pass. (co-07)
- [ ] I can name the four cost categories a real fine-tuning budget must include. (co-08)
- [ ] I can explain what supervised fine-tuning actually trains the model to do. (co-09)
- [ ] I can explain why the dataset, not the hyperparameters, dominates outcome quality. (co-10)
- [ ] I can explain why a few hundred consistent examples can beat tens of thousands of noisy ones.
      (co-11)
- [ ] I can explain why dataset inconsistency is invisible until evaluation. (co-12)
- [ ] I can name the three dataset-sourcing strategies and one distinguishing property of each.
      (co-13)
- [ ] I can explain why synthetic data is bounded by the generating model's own errors. (co-14)
- [ ] I can explain what a held-out split actually protects against. (co-15)
- [ ] I can explain why one leaked evaluation case invalidates the whole eval. (co-16)
- [ ] I can explain what full fine-tuning is and its central risk. (co-17)
- [ ] I can explain the trade parameter-efficient fine-tuning makes and why it usually wins. (co-18)
- [ ] I can explain what LoRA injects into the model and what stays untouched. (co-19)
- [ ] I can explain what an adapter's rank controls and the trade-off in raising it. (co-20)
- [ ] I can explain the operational property that makes adapters an advantage over full fine-tuning.
      (co-21)
- [ ] I can explain why catastrophic forgetting is invisible unless deliberately measured. (co-22)
- [ ] I can explain why training loss fails to reveal overfitting on a small dataset. (co-23)
- [ ] I can name the hyperparameters that dominate fine-tuning outcomes. (co-24)
- [ ] I can state the only meaningful measure of whether a fine-tune helped. (co-25)
- [ ] I can explain what a regression suite's cases must NOT include, and why. (co-26)
- [ ] I can explain what distillation trains a smaller model to do, and for what reason. (co-27)
- [ ] I can explain why a distilled student can never exceed its teacher. (co-28)
- [ ] I can name the operational concerns serving an adapter introduces. (co-29)
- [ ] I can explain what is pinned about a fine-tuned adapter and what breaks that pin. (co-30)
- [ ] I can explain when a base model's licence and data rights must be verified, and why the timing
      matters. (co-31)
- [ ] I can explain why retiring an adapter is a normal, healthy outcome rather than a failure.
      (co-32)

## Elaborative interrogation & self-explanation

Six prompts that ask you to explain WHY, connecting two or more concepts, rather than recall a single
fact. Write your own answer before checking the discussion.

**E1.** Co-01 (behaviour, not knowledge) and co-04 (exhaust retrieval first) both point at the same
underlying distinction. Why does this course treat "is the gap behaviour-shaped or knowledge-shaped"
as the single most load-bearing question in the whole decision procedure?

<details>
<summary>Discussion</summary>

Because the answer to that one question determines which of two entirely different, non-overlapping
toolkits applies -- retrieval for knowledge gaps, adaptation for behaviour gaps -- and getting it
wrong wastes real cost in the wrong direction. Fine-tuning a knowledge gap produces a stale model that
cannot cite sources; trying to "retrieve" a behaviour gap (a tone, a format) does not even make sense,
since there is no document to retrieve. Every other step in the decision gate assumes this
classification was made correctly first.

</details>

**E2.** Co-17 (full fine-tuning) and co-18 (parameter-efficient fine-tuning) are presented as a
trade-off, not a strict ranking. Why does this course still say the adapter should be the DEFAULT
choice, with a full fine-tune required to "argue for itself"?

<details>
<summary>Discussion</summary>

Because for nearly every application-engineering case, the adapter captures most of the benefit at a
fraction of the cost, with far less collateral damage to untouched capability -- the asymmetry in risk
and cost is large enough that starting from "adapter, unless proven insufficient" catches the common
case correctly, while starting from "full fine-tune, unless proven unnecessary" pays the larger cost
and risk by default on cases that never needed it.

</details>

**E3.** Co-23 (overfitting invisible in training loss) and co-15 (train/validation/test discipline)
are two concepts about the same underlying failure mode. What's the precise difference between what
each one is warning about?

<details>
<summary>Discussion</summary>

Co-15 is the structural discipline -- splits must be genuinely disjoint, or nothing downstream is
trustworthy. Co-23 is what happens even WITH a correct split, if you only watch the wrong signal:
training loss keeps improving by construction as the model memorizes its own training data, so a
correct split is necessary but not sufficient -- you also have to actually WATCH the held-out split's
own score, not just trust that loss going down means things are getting better.

</details>

**E4.** Co-25 (evaluate against the base) requires BOTH a target-task comparison and a regression
suite (co-26). Why would a target-task comparison alone, even if done perfectly and with a real
statistical test, still be an incomplete answer to "did this fine-tune help"?

<details>
<summary>Discussion</summary>

Because a target-task comparison, however statistically rigorous, only ever measures the capability
the fine-tune was meant to change -- it has no visibility into capability outside that scope by
construction. A statistically significant target-task win can coexist with severe catastrophic
forgetting (co-22) that the target-task eval was never designed to catch; only a dedicated regression
suite, covering deliberately untouched capability, closes that blind spot.

</details>

**E5.** Co-27 (distillation) and co-32 (when to undo a fine-tune) both involve replacing one model
with another. What's the key difference in what each decision is optimizing for?

<details>
<summary>Discussion</summary>

Distillation replaces a large model with a smaller one trained to reproduce it, optimizing
specifically for latency and cost at an accepted, bounded quality cost -- the student's ceiling is set
by the teacher. Retiring an adapter (co-32) replaces an adapted model with something that beats it on
BOTH quality and cost -- a genuinely better base model or a retrieval solution -- there is no accepted
quality trade in that direction; retirement only happens once a strictly better alternative is
measured, not merely available.

</details>

**E6.** Co-08 (the cost nobody budgets) and co-30 (the maintenance obligation) overlap but are not
identical. Why does this course treat the maintenance obligation as deserving its own separate
concept, rather than folding it entirely into "cost"?

<details>
<summary>Discussion</summary>

Co-08 is about budgeting a fine-tune's TOTAL cost correctly at decision time -- data, compute, eval,
and maintenance, all named up front. Co-30 is about what happens AFTER shipping: the adapter is pinned
to a specific base version, and every future base upgrade re-opens a compatibility question that has
to be checked, not assumed. Separating them matters because co-08's mistake is a one-time budgeting
error made at the start, while co-30's mistake is an ongoing operational blind spot that resurfaces
every time the platform team ships a new base model, long after the original budget was approved.

</details>

---

← Previous: [Capstone](../learning/capstone/overview.md)
