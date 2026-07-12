# 57 · Defensive Security (blue team, SOC/IR) (By Example, Python + shell †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python + shell † · Learn 157 / Drill 257 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: detecting, responding to, and recovering from the attacks the red-team topic produced —
logging/monitoring, detection engineering (Sigma rules, MITRE ATT&CK mapping), the incident-response
lifecycle, and hardening. `†`: Python + shell driving a log/SIEM stack (ELK/OpenSearch) against
lab-generated data. The direct counterpart to [`56-offensive-security`](./56-offensive-security.md);
applies [`55-it-and-application-security`](./55-it-and-application-security.md). This topic **closes Pass 3** and anchors **three
inter-topic capstones** (Pass-3 boundary + two cross-cutting), specified at the end of this file.

## Why this exists · the big idea

- **The problem before the solution**: attacks that succeed unseen are total losses — without detection and
  a rehearsed response, a breach is found months later by someone else, and every red-team finding without a
  matching detection is a blind spot.
- **Keep-this-if-you-forget-everything**: assume you will be attacked and instrument for it — centralize
  telemetry, write detections mapped to known techniques, and rehearse the incident-response loop, tuning
  the false-positive/false-negative balance that keeps the signal usable.
- **Big ideas touched**: `layering-and-leaks` (detection spans every layer the attacker crosses),
  `correctness-vs-pragmatism` (detection engineering is a false-positive/false-negative trade-off, never
  perfect).

## Prerequisites

- **Prior topics**: [topic 56 Offensive Security](./56-offensive-security.md) (the attacks to detect),
  [topic 55 IT / Application Security](./55-it-and-application-security.md) (threats, OWASP, crypto), and
  [topic 5 Just Enough Bash](./05-just-enough-bash.md) (log wrangling).
- **Tools & environment**: a macOS/Linux terminal; the **same isolated local lab** from topic 56
  (attacks generate the telemetry); a local log/SIEM stack (ELK/OpenSearch or an equivalent) + Python for
  detection logic; sample attack logs. Self-owned lab only.
- **Assumed knowledge**: the attack lifecycle + how a web attack looks on the wire (topic 56); the OWASP
  Top 10 + threat modeling (topic 55); shell + log filtering (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (CORRECTION, recent): **MITRE ATT&CK v19 (2026-04-28)** restructured the Enterprise
  matrix — the "Defense Evasion" tactic **split into "Stealth" (keeps TA0005) + new "Defense Impairment"
  (TA0112)**; Enterprise now has **15 tactics** (was 14). Do NOT name "Defense Evasion" as a current tactic —
  reference Stealth/Defense Impairment or teach generically. (attack.mitre.org/tactics/TA0112)
- 2026-07-12 — verified (CORRECTION): the classic 6-phase IR lifecycle (prep → detect/analyze → contain →
  eradicate → recover → lessons-learned) is SANS **PICERL** / NIST SP 800-61 **Rev. 2 (2012)** framing —
  NIST **superseded it with SP 800-61 Rev. 3 (April 2025)**, which drops the standalone lifecycle and maps
  IR onto CSF 2.0 functions (Govern/Identify/Protect/Detect/Respond/Recover). Either teach the classic model
  cited as SANS/industry-standard, or teach the current CSF-2.0-mapped Rev. 3 — do not attribute the 6-phase
  model to "current NIST." (csrc.nist.gov/pubs/sp/800/61/r3/final)
- 2026-07-12 — verified: Sigma rule format current (SigmaHQ YAML, `pySigma` → 40+ query langs). ELK/OpenSearch
  (DD-21 flag): **OpenSearch is cleanly Apache-2.0** (OpenSearch Software Foundation) and satisfies DD-21;
  Elasticsearch's default distribution ships under **Elastic License 2.0 (not OSI-approved)** despite the
  AGPLv3 option added 2024 — default to OpenSearch, or be explicit about the AGPLv3 self-managed Elastic
  build. (sigmahq.io / elastic.co/pricing/faq/licensing / opensearch.org)

## Items

- Logging & monitoring: what to log, centralized logging, a SIEM (ELK/OpenSearch), dashboards.
- Detection engineering: writing detections (Sigma rules), mapping to MITRE ATT&CK tactics/techniques,
  reducing false positives.
- The incident-response lifecycle: preparation → detection/analysis → containment → eradication → recovery
  → lessons-learned.
- Threat hunting: hypothesis-driven searching through telemetry.
- Hardening & recovery: closing the gaps the red team found; backups, least privilege, patching.
- Purple-teaming: red findings → blue detections, closing the loop.

## Tensions & trade-offs — when NOT to reach for this

- **False positives vs false negatives**: a detection tuned too tight misses the attack; tuned too loose it
  buries the SOC in noise until real alerts are ignored (alert fatigue is itself a breach cause). Detection
  engineering is continuous tuning of that balance, never a finished ruleset.
- **Detection vs prevention**: you can't detect your way out of a preventable hole — but you also can't
  prevent everything, so over-investing in either extreme fails. The mix depends on what is cheap to prevent
  versus what must be caught in flight.
- **When NOT to build it big**: not every environment needs a full SIEM + 24/7 SOC. A small shop may get more
  safety from patching and least privilege than from a detection pipeline it can't staff. Right-size
  detection to what you can actually triage.

## Lineage — why it beat the alternative

- Blue-team practice matured from ad-hoc log-grepping into detection engineering as attacks industrialized.
  MITRE ATT&CK (2013+) gave a shared language of adversary techniques so detections could map to real
  behavior instead of guesswork; Sigma made detections portable across SIEMs; the SANS/NIST IR lifecycles
  codified how to respond under pressure (NIST SP 800-61 Rev. 3, 2025, re-maps it onto CSF 2.0). The
  purple-team loop closes it: every offense from [`56-offensive-security`](./56-offensive-security.md) should
  produce a detection here. The invariant — assume breach, instrument, rehearse — is the defensive face of
  the same threat-driven thinking as [`55-it-and-application-security`](./55-it-and-application-security.md).

## Worked examples

Colocated under `defensive-security/learning/`; Python + shell over lab-generated telemetry (DD-20/DD-30).

- **beginner** — ingest the lab's attack logs into the SIEM; build a dashboard that surfaces the recon scan
  from topic 56.
- **intermediate** — write a Sigma detection for the SQL-injection attempt + map it to a MITRE ATT&CK
  technique; verify it fires on the attack log and not on benign traffic.
- **advanced** — run a full incident-response tabletop over one attack: detect → contain → eradicate →
  recover → write the post-incident report.

## Capstone spec — intra-topic (subject → full runnable, lab-local)

- **Goal**: stand up a blue-team pipeline over the telemetry the topic-39 red-team capstone generated —
  centralized logging + a SIEM, detection rules (Sigma) for each exploited attack mapped to MITRE ATT&CK,
  a dashboard, and a full incident-response run (detect → contain → eradicate → recover → lessons-learned)
  producing a post-incident report — closing the purple-team loop.
- **Concepts exercised**: [ ] centralized logging + a SIEM dashboard [ ] a Sigma detection per attack
  [ ] MITRE ATT&CK technique mapping [ ] a threat-hunt query [ ] the IR lifecycle end to end [ ] a
  post-incident report with lessons-learned.
- **Ordered steps**:
  1. `.../learning/capstone/ingest/` — pipe the topic-39 attack logs into the SIEM + a dashboard. Verify the
     recon scan and the exploits are visible in the dashboard.
  2. `.../learning/capstone/detections/` — a Sigma rule per exploited vuln, each mapped to a MITRE ATT&CK
     technique. Verify each rule fires on its attack and stays quiet on benign traffic (low false positives).
  3. `.../learning/capstone/hunt.md` — a hypothesis-driven threat-hunt query over the telemetry. Verify it
     surfaces the attacker activity from the raw logs.
  4. `.../learning/capstone/ir-report.md` — an IR run (detect → contain → eradicate → recover) +
     lessons-learned mapped back to the red-team findings. Verify every topic-39 finding has a matching
     detection + remediation (purple-team loop closed).
- **Acceptance criteria**: attacks are visible in the SIEM; each exploited vuln has a firing, low-false-
  positive Sigma detection mapped to ATT&CK; the threat hunt finds the activity; the IR report closes every
  red-team finding with a detection + remediation.
- **Done bar**: runnable end-to-end against the local lab telemetry + web-verified.

---

## Capstone spec — inter-topic: capstone-real-world-delivery (Pass-3 boundary)

**Weight**: `learning/_index.md` and the drilling mirror place this capstone at **575** (Pass-3 boundary,
after topic 57; ahead of Pass 4). Kind: **subject → full runnable** (DD-27). This is the Pass-3 graduation
project — the "Build for the Real World" pass made real.

- **Goal**: take the Pass-2 `capstone-solid-core` application and deliver it the way a real team ships:
  choose and justify a data layer beyond a single SQL DB (NoSQL/graph where it fits — topics 34/35),
  scale the backend (39) with an event-driven slice (45) modeled with DDD (43) under a documented
  architecture (42) and a system-design capacity plan (44), containerize + orchestrate it (50), describe
  its infrastructure as code (51), add a CI/CD pipeline that builds, tests, and deploys it (52), and secure it end to end (55) with red-team validation (56) and
  blue-team detection (57) — a complete, deployed-as-code, secured, observable system.
- **Integrates topics**: 34 NoSQL · 35 Graph (where it fits) · 39 Backend at Scale · 42 Architecture ·
  43 DDD · 44 System Design · 45 Event-Driven · 50 Containers/K8s · 51 Cloud/IaC · 52 CI/CD · 55 IT Security ·
  56 Offensive (validation) · 57 Defensive (detection). (37/53 optional where the domain benefits.)
- **Concepts exercised**: [ ] a justified polyglot-persistence choice [ ] a scaled, event-driven backend
  slice [ ] a DDD-modeled domain under a documented (C4) architecture [ ] a system-design capacity plan
  [ ] containerized + orchestrated deployment [ ] infrastructure described as code [ ] a CI/CD pipeline (build → test → deploy) [ ] a security
  assessment + red-team validation + blue-team detections.
- **Ordered steps**:
  1. `.../capstone-real-world-delivery/design/` — architecture (C4) + a system-design capacity/trade-off
     plan + the persistence-choice rationale. Verify the diagrams match the intended build and the capacity
     numbers are arithmetic-checked.
  2. `.../capstone-real-world-delivery/app/` — the DDD-modeled, scaled backend with an event-driven slice +
     the chosen data layer(s). Verify the domain invariants hold, the event slice is reliable (no lost/
     double messages), and the data-layer choice is exercised.
  3. `.../capstone-real-world-delivery/deploy/` — Dockerfile(s) + K8s manifests + Terraform/OpenTofu (local
     provider) + a CI/CD pipeline (52) that builds, tests, and deploys on push. Verify the app deploys to the local cluster via the IaC + pipeline and self-heals.
  4. `.../capstone-real-world-delivery/security/` — a threat model + red-team validation (lab-local) + a
     blue-team detection set. Verify each identified threat has a mitigation and a firing detection.
- **Acceptance criteria**: the system runs deployed-as-code on a local cluster; the domain + event slice are
  correct; capacity/architecture are documented and matched by the build; the security loop (model →
  red-team → blue-team) is closed. All work stays within self-owned labs.
- **Done bar**: runnable end-to-end (deployed via IaC to the local cluster) + security loop closed +
  web-verified.

## Capstone spec — inter-topic: capstone-secure-service (cross-cutting)

**Weight**: `learning/_index.md` and the drilling mirror place this capstone at **576** (cross-cutting,
just after `capstone-real-world-delivery`). Kind: **subject → full runnable** (DD-27). A focused
security-thread capstone that can be pursued independently of the full Pass-3 project.

- **Goal**: build (or take a prior) HTTP service and make it demonstrably secure end to end — apply the
  OWASP Top 10 (2025) mitigations, do proper identity (OAuth2/OIDC + JWT done right), harden it, then
  **prove** the security by attacking it from the red-team lab (56) and **detecting** those attacks from
  the blue-team stack (57) — a single service where the full security lifecycle is visible.
- **Integrates topics**: 17 Security Essentials · 39 Backend (auth surface) · 55 IT Security (OWASP/crypto/
  identity) · 56 Offensive (validation) · 57 Defensive (detection). (50 for a hardened container image
  where used.)
- **Concepts exercised**: [ ] OWASP Top 10 (2025) mitigations applied [ ] OAuth2/OIDC + JWT done right
  [ ] hardening (headers, least privilege, secrets in env) [ ] red-team validation of the mitigations
  [ ] blue-team detections for the attempted attacks [ ] a before/after security posture writeup.
- **Ordered steps**:
  1. `.../capstone-secure-service/app/` — the service with OWASP-2025 mitigations + OAuth2/OIDC + JWT
     integrity + hardening. Verify each Top-10 category is addressed and auth gates behave correctly.
  2. `.../capstone-secure-service/attack/` — lab-local red-team attempts against the service. Verify the
     mitigations hold (attacks that succeeded pre-hardening now fail) — authorized self-owned target only.
  3. `.../capstone-secure-service/detect/` — blue-team detections (Sigma + ATT&CK mapping) for the attempted
     attacks. Verify each attempt raises a detection with low false positives.
  4. `.../capstone-secure-service/posture.md` — a before/after security-posture writeup. Verify each fixed
     weakness is tied to its mitigation, its failed attack, and its detection.
- **Acceptance criteria**: the OWASP-2025 mitigations demonstrably hold under lab-local attack; identity is
  correct; every attempted attack is detected; the posture writeup ties mitigation → validation → detection.
- **Done bar**: runnable end-to-end + attacks demonstrably mitigated + detected (lab-local) + web-verified.

## Capstone spec — inter-topic: capstone-data-pipeline (cross-cutting)

**Weight**: `learning/_index.md` and the drilling mirror place this capstone at **577** (cross-cutting,
just after `capstone-secure-service`). Kind: **subject → full runnable** (DD-27). A focused data-thread
capstone, independently pursuable.

- **Goal**: build an end-to-end data path — ingest raw data through a medallion pipeline (bronze/silver/
  gold) with data-quality gates (37), model it with advanced SQL for serving (26), optionally serve a
  read-optimized store (34/35 where the access pattern fits), and expose it through an AI-powered,
  RAG-grounded query interface (53) over a backend (39) — a complete "raw data → governed warehouse →
  intelligent interface" slice.
- **Integrates topics**: 10 SQL · 26 Advanced SQL · 34 NoSQL / 35 Graph (where the serving pattern fits) ·
  39 Backend at Scale (serving) · 37 Data Engineering (the pipeline) · 53 AI-Powered Apps (the interface).
- **Concepts exercised**: [ ] a medallion ETL/ELT pipeline (bronze→silver→gold) [ ] data-quality gates
  [ ] a star schema + advanced-SQL serving queries [ ] a fit-for-purpose serving store [ ] a RAG-grounded
  query interface [ ] served over a backend endpoint.
- **Ordered steps**:
  1. `.../capstone-data-pipeline/pipeline/` — raw → bronze → silver → gold with quality gates (37). Verify
     idempotent re-runs, a bad batch caught by the gate, and a reconciling star schema.
  2. `.../capstone-data-pipeline/serve/` — advanced-SQL serving queries (26) + optionally a read-optimized
     store (34/35). Verify serving queries match hand-computed expected results.
  3. `.../capstone-data-pipeline/interface/` — a RAG-grounded (53) query interface over the gold data,
     served via a backend endpoint (39). Verify answers are grounded in the served data + cited.
  4. `.../capstone-data-pipeline/eval.md` — an eval of the interface's answer quality + a data-freshness/
     quality report. Verify the eval is reproducible and the freshness/quality metrics are reported.
- **Acceptance criteria**: the medallion pipeline is idempotent + quality-gated; serving queries are
  correct; the RAG interface answers are grounded in the governed data + cited; the eval is reproducible.
- **Done bar**: runnable end-to-end (raw data → governed warehouse → grounded interface) + web-verified.

## Read more

**Books**

- **Blue Team Handbook: Incident Response Edition** — Don Murdoch (2nd ed., 2014). Widely used field reference for defenders and incident responders.
- **Applied Network Security Monitoring** — Chris Sanders, Jason Smith (2013). Standard reference for building detection and monitoring practice (NSM).

**Papers & articles**

- **SP 800-61 Rev. 2: Computer Security Incident Handling Guide** — NIST (2012). The canonical US government reference framework for incident response process. <https://csrc.nist.gov/pubs/sp/800/61/r2/final>
- **MITRE ATT&CK** — MITRE Corporation (ongoing). Also the standard reference for defenders to map detections and coverage against known adversary techniques. <https://attack.mitre.org/>

---

← Previous: [56 · Offensive Security](./56-offensive-security.md) · Next: [58 · IT Governance, Risk & Compliance](./58-it-governance-grc.md) →
