---
title: "Artifact: Trust-Calibration Diagram"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 6
---

> Worked Scenario 6 -- trust-calibration diagram -- exercises co-04.

Trust calibration plots actual reliability on one axis against observed user trust on the other.
The diagonal is the calibrated zone: trust tracking reliability. The two off-diagonal quadrants are
both design failures, and each is corrected by a different set of interface moves.

```mermaid
%% Color Palette: Blue #0173B2, Orange #DE8F05, Teal #029E73, Purple #CC78BC
%% Reliability (x-axis, conceptual) vs. user trust (y-axis, conceptual)
graph TD
    Cal["Calibrated zone<br/>trust tracks reliability"]:::teal

    OverTrust["Over-trust quadrant<br/>high trust, moderate risk<br/>co-04, co-05, co-08"]:::orange
    OverTrustFix["Surface uncertainty (co-06)<br/>Show provenance (co-09)<br/>Confidence != correctness"]:::blue

    Distrust["Blanket-distrust quadrant<br/>low trust, high reliability<br/>Scenario 9"]:::purple
    DistrustFix["Case-by-case signals (co-06)<br/>Citations to check (co-09)<br/>Cheap verification (co-10)"]:::blue

    OverTrust --> OverTrustFix
    Distrust --> DistrustFix
    Cal -.->|drift toward over-trust as<br/>reliability rises unchecked| OverTrust
    Cal -.->|drift toward distrust after<br/>an early, unsignalled miss| Distrust

    classDef teal fill:#029E73,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef orange fill:#DE8F05,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef purple fill:#CC78BC,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef blue fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
```

_Figure: the calibrated zone sits between two named failure quadrants. A feature drifts toward
over-trust as its measured reliability rises without a matching uncertainty signal (Scenario 8), and
toward blanket distrust after an early miss with no case-by-case signal to fall back on (Scenario
9). Each quadrant's fix is a distinct set of interface moves, not a single universal remedy._

**Verify**: both off-diagonal quadrants are present, each is labelled with the specific scenario
that instantiates it (Scenario 8 for over-trust, Scenario 9 for distrust), and each has its own,
distinct fix list -- satisfying co-04's rule that trust calibration is a two-sided design problem.

**Key takeaway**: There is no single "add more trust signals" fix -- over-trust and blanket distrust
require opposite interventions, and a design that treats trust as one-dimensional will
under-correct one quadrant while over-correcting the other.

**Why It Matters**: This diagram is the map every later worked scenario about trust, uncertainty, or
provenance implicitly places itself on. Recognising which quadrant a given feature is drifting
toward -- before choosing a fix -- is what prevents a team from, for example, adding more caveats
(a distrust-quadrant fix) to a feature that is actually suffering from over-trust, which would make
the real problem worse rather than better.

---

← Back to [Theme A: The Wrong Answer Is Coming](../theme-a-the-wrong-answer-is-coming.md)
