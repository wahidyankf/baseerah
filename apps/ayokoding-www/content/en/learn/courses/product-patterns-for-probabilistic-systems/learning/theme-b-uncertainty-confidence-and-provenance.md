---
title: "Theme B: Uncertainty, Confidence, and Provenance"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 20
---

Scenarios 12-22 build the toolkit that closes the gap Theme A diagnosed: where an uncertainty
signal belongs (co-06), what trade-off each representation of confidence makes (co-07), why
confidence is not the same claim as correctness (co-08), and why a citation a user can independently
check (co-09) is often a stronger signal than any confidence number, especially once the output
itself is restructured to be cheap to verify (co-10). The theme closes with an uncertainty
treatment that clears WCAG AA without depending on colour alone. Scenario 21's artefact is a
captioned, WCAG-accessible Mermaid diagram and lives standalone under `learning/artifacts/`.

---

### Worked Scenario 12: Where Uncertainty Belongs

**Context**: Exercises co-06. Nimbus originally placed a single disclaimer -- "AI-generated answers
may contain errors" -- in the page footer, visible only if a user scrolls past the answer entirely.
This scenario relocates the signal to the point it can actually be acted on.

**Decision artifact**:

> **Placement redesign -- uncertainty signal relocation**
>
> - **Before**: static disclaimer in the page footer, below the answer, below the citation links,
>   below the "was this helpful?" prompt. Measured: fewer than 3% of users scroll far enough to see
>   it before acting on the answer above.
> - **After**: an inline badge directly adjacent to any answer flagged low-confidence by Nimbus's own
>   scorer, positioned before the user's next likely action (in this case, before the "schedule
>   renewal" button).
> - **Decision rule**: an uncertainty signal's placement is correct only if it sits between the
>   answer and the user's next action -- never after it.

**Verify**: the "after" placement sits before the specific action (scheduling a renewal) the user
would take based on the answer, satisfying co-06's rule that a signal must appear while the user
could still act on it.

**Key takeaway**: The same words, moved from the footer to directly beside the answer, change from
decorative to functional -- placement is not a styling detail, it is the entire mechanism by which
an uncertainty signal works or fails to work.

**Why It Matters**: A correct uncertainty signal in the wrong place is functionally equivalent to no
signal at all, because a user who has already acted cannot retroactively benefit from it. Every
remaining scenario in this theme assumes the placement problem is solved and focuses on what the
signal itself should say. Getting placement right first is also what makes the rest of this theme's content trade-offs meaningful -- comparing representations or writing a selective policy is wasted effort if the resulting signal never reaches the user in time to matter.

---

### Worked Scenario 13: Four Confidence Representations

**Context**: Exercises co-07. The same low-confidence Nimbus answer is shown four different ways, to
compare what each communicates and what each risks being misread as.

**Decision artifact**:

| Representation   | Rendered as                                                    | Communicates                                 | Risk                                                                                                                       |
| ---------------- | -------------------------------------------------------------- | -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| Numeric score    | "Confidence: 0.72"                                             | A precise-looking quantity.                  | Read as "72% chance this is correct" -- a probability claim the score does not actually make (co-08).                      |
| Qualitative band | "Confidence: Moderate"                                         | A coarse, ordinal signal.                    | Discards the precision a user might genuinely want when comparing two answers.                                             |
| Hedged language  | "This is likely the Meridian contract, but verify."            | A natural-language caveat embedded in prose. | Easy to skim past if the sentence is long; strength of the hedge varies by wording, not by an underlying number.           |
| Visual treatment | Answer text rendered in a distinct, muted colour with an icon. | An at-a-glance category (verify vs. trust).  | Fails for colour-blind or screen-reader users unless paired with text and shape (co-06's accessibility rule, Scenario 22). |

**Verify**: each row states both what its representation communicates and a distinct, named risk --
satisfying co-07's rule that every representation trades precision against being read correctly, in
its own specific way.

**Key takeaway**: There is no confidence representation without a trade-off -- the choice is about
which risk (false precision, discarded information, skippable prose, inaccessible visuals) a given
feature can least afford.

**Why It Matters**: Scenario 15 returns to the numeric-versus-band choice specifically, because it
is the most contested of the four in real product practice; this comparison table is the reference
every later choice in this theme draws from. No single row in this table is disqualified outright -- a product team should expect to mix representations across different parts of the same feature, choosing per situation rather than picking one company-wide default and applying it everywhere.

---

### Worked Scenario 14: Confidence Is Not Correctness

**Context**: Exercises co-08. Nimbus's own confidence scorer rated a wrong answer 0.94 -- high
confidence -- because the model's internal token probabilities were high for a fluently phrased,
factually incorrect sentence. This scenario walks through why displaying that number as-is would
have manufactured false trust.

**Decision artifact**:

> **Design-error walkthrough -- high confidence, wrong answer**
>
> - **What happened**: the model generated "The Meridian Supply contract expires March 3" with an
>   internal score of 0.94, and it was factually wrong (the correct date was February 14, for a
>   different contract).
> - **Why the score was high**: the scorer measures how confidently the model generated this
>   specific sequence of tokens -- essentially "how sure the model sounds to itself" -- not whether
>   the underlying claim about the contract is true. A model can sound very sure while retrieving
>   the wrong source document entirely.
> - **The design error being avoided**: displaying "94% confident" beside this answer would have
>   told the user the opposite of the truth -- it would have made a wrong answer look more
>   trustworthy than a correctly-hedged one.
> - **The fix applied**: Nimbus's confidence badge is calibrated against held-out eval accuracy per
>   confidence bucket, not raw model score, and is labelled "Nimbus's self-assessed certainty" rather
>   than "accuracy," so the badge never implies a probability of being right.

**Verify**: the walkthrough names the specific mechanism (token-generation confidence, not claim
verification) that produced a high score on a wrong answer, satisfying co-08's rule that confidence
and correctness are measuring different things.

**Key takeaway**: A model's confidence score measures how sure the model sounds to itself, not how
likely the underlying claim is true -- treating the two as interchangeable is the single most common
confidence-display error.

**Why It Matters**: This is the concrete failure mode co-08 warns about, and it is the reason
Scenario 18's citation-based signal is often preferred over any confidence score at all -- a
citation lets the user verify the claim directly, sidestepping the confidence-versus-correctness gap
entirely. Any team building its own confidence indicator should ask, explicitly, what the underlying number actually measures before deciding how -- or whether -- to show it to a user at all.

---

### Worked Scenario 15: Numeric vs. Band, User Reading

**Context**: Exercises co-07 and co-08. Continuing Scenario 13's comparison, this scenario documents
Nimbus's own contested internal debate over "0.82" versus "likely" without resolving it, per this
course's own accuracy-note discipline.

**Decision artifact**:

> **Contested-decision memo -- numeric score vs. qualitative band**
>
> - **Position for numeric ("0.82")**: precise, comparable across answers, and useful for a power
>   user who wants to rank several candidate answers against each other.
> - **Position for band ("Likely")**: harder to misread as a probability-of-correctness claim,
>   because it makes no numeric claim at all; better matches what the underlying score can actually
>   support.
> - **Observed user behaviour (informal internal testing, both variants shown to the same ten
>   reviewers)**: reviewers shown "0.82" more often described the answer as "82% likely to be
>   right" in their own words; reviewers shown "Likely" more often described it as "probably right,
>   but I'd double-check," closer to the intended meaning.
> - **Decision**: unresolved. Nimbus ships the qualitative band for now, revisits if a future,
>   larger user study contradicts the informal result above.

**Verify**: the memo presents both positions with a stated basis for each and does not declare one
correct -- satisfying this course's own discipline of teaching a genuinely contested trade-off as
contested, per co-07 and co-08.

**Key takeaway**: The numeric-versus-band choice is not a solved problem in this field -- a written
decision that documents the trade-off and the evidence considered is more honest than pretending one
answer is universally correct.

**Why It Matters**: Presenting this as settled, in either direction, would misrepresent the current
state of product practice. A team encountering this exact choice should expect to run its own user
research rather than copy either side of this memo as a default. Documenting the trade-off this openly also protects future teams from re-litigating the same debate from scratch -- the next revision can start from this memo's evidence instead of a fresh opinion-only argument.

---

### Worked Scenario 16: Caveat Fatigue

**Context**: Exercises co-06. An earlier Nimbus revision attached the same hedge -- "this may not
be fully accurate" -- to every single answer, regardless of the model's actual confidence. This
scenario documents what happened to user attention as a result.

**Decision artifact**:

> **Caveat-fatigue observation log**
>
> - **Week 1**: users report reading the hedge and, in several cases, independently verifying the
>   answer against the source document before acting.
> - **Week 6**: support tickets and session recordings show users scrolling past the identical hedge
>   without pausing -- it now appears on literally every answer, including ones later confirmed
>   correct, so it has stopped correlating with anything actionable.
> - **Root cause**: a hedge attached uniformly carries zero information, because it does not
>   distinguish the answers that actually need extra scrutiny from the ones that do not.

**Verify**: the log identifies the specific point (uniform application regardless of actual
confidence) at which the hedge's information content dropped to zero, satisfying co-06's rule that
a signal must be selective to remain meaningful.

**Key takeaway**: A caveat shown on every single output is mathematically indistinguishable from no
caveat at all, because it carries no information about which specific answer needs extra scrutiny.

**Why It Matters**: This is the direct motivation for Scenario 17's selective-uncertainty policy --
caveat fatigue is not a hypothetical risk, it is the observed, predictable consequence of applying
an uncertainty signal without a rule for when it fires. It is also a cautionary example for any team tempted to add a caveat just to be safe -- safety in appearance is not the same as safety in practice, and an uninformative caveat actively erodes the signal value of every future one.

---

### Worked Scenario 17: Selective-Uncertainty Policy

**Context**: Exercises co-06 and co-07. Following Scenario 16's diagnosis, this scenario writes the
rule that decides which Nimbus answers carry an uncertainty signal and which do not, applicable
without a human making a case-by-case judgment call each time.

**Decision artifact**:

> **Selective-uncertainty policy -- Nimbus contract-metadata answers**
>
> - **Fires the signal when**: (a) the answer's calibrated confidence bucket (Scenario 14) falls
>   below the 90th-percentile accuracy threshold measured against the eval set, OR (b) the source
>   citation spans more than one document, OR (c) the question was previously logged as a case where
>   two identical requests returned different answers (co-19's inconsistency failure mode).
> - **Does not fire when**: none of the above conditions hold -- the answer renders with no
>   additional signal, exactly as before.
> - **Rationale**: each condition names a concrete, measurable property of the answer, not a vague
>   "seems uncertain" judgment -- so the policy applies identically regardless of who is reviewing
>   it.

**Verify**: every trigger condition in the policy is a concrete, checkable property (a confidence
bucket threshold, a citation-span count, a logged inconsistency) rather than a subjective judgment
call -- satisfying co-06 and co-07's rule that a selective policy must be applicable without
case-by-case discretion.

**Key takeaway**: A selective-uncertainty policy is only as good as its trigger conditions -- vague
triggers reproduce the caveat-fatigue problem one condition at a time, while concrete, measurable
triggers keep the signal meaningful.

**Why It Matters**: This policy is the concrete artefact that resolves the tension named in this
course's own overview: uncertainty signals must be selective enough to carry information. Writing
the rule down, rather than leaving it to reviewer judgment, is what makes the policy consistent
across every answer Nimbus produces. A policy this concrete is also auditable -- a reviewer or a future engineer can check any specific answer against the three conditions directly, rather than having to guess whether a given signal should have fired.

---

### Worked Scenario 18: Citation as the Better Signal

**Context**: Exercises co-09. Rather than adding another confidence indicator, this scenario
replaces the badge entirely with a direct citation to the source clause, for the subset of answers
where a citation is practical.

**Decision artifact**:

> **Redesign -- confidence badge replaced by citation**
>
> - **Before**: "Confidence: Moderate" badge next to the answer, with no way for the user to check
>   the claim except trusting the badge.
> - **After**: the answer text includes an inline link -- "Anchor Logistics contract, Section 4.2" --
>   that opens the exact source clause the answer was drawn from, highlighted.
> - **User behaviour change**: Priya no longer has to decide whether to trust a badge -- she can
>   open Section 4.2 and read the actual clause herself in under ten seconds, converting "should I
>   trust this?" into "let me check this," a strictly easier and more reliable question to answer.

**Verify**: the citation opens to the exact source clause (not merely the source document) and does
so in a single click, satisfying co-09's rule that provenance must be practically checkable, not
merely present.

**Key takeaway**: A citation does not ask the user to trust a number -- it hands them the means to
verify the claim themselves, which is a fundamentally stronger position than any confidence signal
can offer.

**Why It Matters**: This is the concrete resolution of Scenario 9's blanket-distrust case -- a
citation gives users exactly the case-by-case discrimination ability that a uniform visual treatment
could not, without requiring the user to trust an opaque confidence number at all. It also reduces the interface's own burden -- a well-placed citation replaces an entire category of confidence-display design decisions (which number, which band, which colour) with a single, simpler mechanism the user already understands: a link.

---

### Worked Scenario 19: Provenance That Does Not Help

**Context**: Exercises co-09 and co-10. Not every citation is equally useful. This scenario contrasts
Scenario 18's clickable clause citation against a Nimbus citation that names a 40-page PDF with no
page number or highlight -- provenance in name only.

**Decision artifact**:

> **Provenance-quality comparison**
>
> - **Practical citation (Scenario 18)**: "Anchor Logistics contract, Section 4.2" -- opens directly
>   to the highlighted clause. Verification cost: under ten seconds.
> - **Provenance theatre**: "Source: Vendor_Contracts_Master.pdf" -- a 40-page file with no page
>   number, section reference, or highlight. Verification cost: several minutes of manual searching,
>   which most users will not spend, making the citation functionally decorative.
> - **Conclusion**: a citation's value is entirely a function of how cheaply a user can actually use
>   it to verify the claim -- naming a source without making it practically checkable produces the
>   appearance of verifiability with none of the substance.

**Verify**: the comparison states a concrete verification-cost estimate for each citation and
identifies which one crosses a threshold most users will not spend, satisfying co-09 and co-10's
rule that provenance must be practically, not merely nominally, checkable.

**Key takeaway**: A citation that names a source without pointing at the specific claim is
provenance theatre -- it looks like verifiability in a design review and functions like nothing at
all in actual use.

**Why It Matters**: This is the specific trap Scenario 20's cheap-verification redesign exists to
avoid -- the fix for weak provenance is not removing citations, it is investing in the retrieval and
rendering work that makes a citation point at an exact, checkable location. Any team auditing its own feature's citations should apply this same verification-cost test -- a source name alone is not evidence of provenance; only a source a user can and will actually check counts.

---

### Worked Scenario 20: Design for Cheap Verification

**Context**: Exercises co-10. Building on Scenario 19's diagnosis, this scenario restructures
Nimbus's output format itself so that checking any answer costs meaningfully less than the model
took to produce it.

**Decision artifact**:

> **Output restructuring -- from free-text answer to structured, checkable fields**
>
> - **Before**: a single free-text paragraph answering "which contract expires first, and what are
>   its terms?" -- verifying it requires re-reading the whole paragraph against the whole contract.
> - **After**: a structured card -- `contract_name`, `expiry_date`, `renewal_terms` -- each field
>   individually linked to its own exact source clause, so a user can check any one field
>   independently without re-reading the others.
> - **Verification-cost result**: checking one field (say, just the expiry date) now takes roughly
>   five seconds against roughly forty-five seconds to verify the equivalent claim buried in the
>   original free-text paragraph.

**Verify**: the restructuring names the specific verification-cost reduction (roughly 45 seconds to
roughly 5 seconds for a single field) and attributes it to the output's shape, not to any change in
the underlying model -- satisfying co-10's rule.

**Key takeaway**: Verifiability is a property you design into the output's shape -- a structured,
field-by-field format with per-field citations is dramatically cheaper to check than an equally
accurate free-text paragraph, at no cost to the underlying model.

**Why It Matters**: This is the single highest-leverage pattern in this theme, and it composes with
everything before it: Scenario 17's selective-uncertainty policy can now fire per field instead of
per answer, and Scenario 18's citations become per-field instead of per-paragraph, each making the
whole answer cheaper and more precisely verifiable. Because this change lives entirely in the output's shape, it is also durable across future model swaps -- a better or worse underlying model still benefits from the same structured, per-field citation format.

---

### Worked Scenario 21: Verifiability Diagram (diagram)

**Context**: Exercises co-10 and co-09. This diagram-medium scenario ranks the output structures
seen across Theme B by their verification cost, from the cheapest (a structured, per-field citation)
to the most expensive (an unstructured paragraph with a document-only citation).

> A captioned diagram ranking output structures by verification cost, from cheapest to most
> expensive -- exercises co-10 and co-09. The full diagram, with its own Verify / Key takeaway / Why
> It Matters, lives at
> [Artifact: Verifiability Diagram](/en/learn/courses/product-patterns-for-probabilistic-systems/learning/artifacts/ex-21-verifiability-diagram).

**Key takeaway**: Verification cost is a ladder, not a binary -- every rung up from "unstructured,
no citation" toward "structured, per-field citation" buys a concrete, measurable reduction in how
long a user needs to check the answer.

**Why It Matters**: This ranking is the reference Theme C's review-cost scenarios (23-26) build on
directly -- a review step's cost is bounded from below by how verifiable its input already is, so
investing in Theme B's rungs pays off again, concretely, in Theme C's human-review design. A team auditing its own feature can place its current output format on this same ladder to see, concretely, how much investment separates it from the cheapest achievable rung.

---

### Worked Scenario 22: Accessible Uncertainty

**Context**: Exercises co-06 and co-07. Scenario 13's "visual treatment" representation -- a muted
colour and an icon -- fails for colour-blind and screen-reader users unless it is redesigned to
carry its meaning through more than colour alone. This scenario produces that redesign and confirms
it against WCAG AA.

**Decision artifact**:

> **Accessible uncertainty treatment -- before and after**
>
> - **Before**: low-confidence answers rendered with a muted orange background only. Fails for
>   protanopia/deuteranopia (colour alone conveys the distinction) and fails for screen readers
>   (background colour is not announced).
> - **After**: low-confidence answers carry (1) a text label, "Verify before acting," (2) a distinct
>   icon shape (a triangle, not merely a colour swatch), and (3) an `aria-label` announcing "Nimbus
>   flagged this answer for verification" -- so the same signal survives a colour-blind reading, a
>   greyscale printout, and a screen-reader pass.
> - **Contrast check**: the label text against its background measures 6.1:1, clearing the 4.5:1
>   WCAG AA minimum for normal text.

**Verify**: the redesign carries its meaning through a text label and a distinct shape independent
of colour, includes a screen-reader-announced label, and its text contrast measures at or above
4.5:1 -- satisfying co-06 and co-07's rule that an uncertainty signal must survive both a
colour-blind and a screen-reader reading.

**Key takeaway**: An uncertainty signal that depends on colour alone is not actually a signal for
every user -- text, shape, and an accessible label are what make it a signal for all of them.

**Why It Matters**: This closes Theme B on the same accessibility bar this course's own diagrams are
held to -- a pattern this course teaches and then fails to meet in its own worked example would
undermine the very point of teaching it. Every uncertainty and confidence pattern from this theme
should be checked against this same WCAG AA bar before shipping.

---

← Previous: [Theme A: The Wrong Answer Is Coming](./theme-a-the-wrong-answer-is-coming.md) &middot; Next: [Theme C: Humans in the Loop, Friction, and Recovery](./theme-c-humans-in-the-loop-friction-and-recovery.md) →
