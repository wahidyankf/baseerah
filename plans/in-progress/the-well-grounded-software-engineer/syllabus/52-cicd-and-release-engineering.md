# 52 · CI/CD & Release Engineering (By Example, YAML + Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · YAML + Python † · Learn 152 / Drill 252 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: automating the path from commit to production — GitHub Actions hands-on (matrix
builds, caching, artifacts, environments, secrets, reusable/composite workflows), CD strategies
(blue-green, canary, progressive delivery), release automation, and supply-chain basics (SLSA,
provenance, signing). GitHub Actions is free for public repos, so every example is reproducible.
`†`: pipelines are YAML; automation scripts are Python, fully type-annotated (DD-34) — every snippet
carries type hints in the mypy-clean spirit.

## Why this exists · the big idea

- **The problem before the solution**: manual releases are where quality goes to die — a human runs
  the tests "usually", copies a build to a server on a Friday, and forgets a step, so the difference
  between what was tested and what's in production is a mystery nobody can reconstruct. Ship-by-hand
  makes every deploy a risk and every rollback a scramble.
- **Keep-this-if-you-forget-everything**: make the path from commit to production a single automated,
  repeatable pipeline — every change goes through the same gates, produces the same kind of artifact,
  and deploys the same way, so a release is boring and a rollback is a button.
- **Big ideas touched**: `correctness-vs-pragmatism` (a pipeline encodes "provably tested" as gates
  but must still ship — you tune which checks block vs warn so the gate protects without paralyzing),
  `mechanism-vs-policy` (the CI/CD engine is mechanism — runners, steps, artifacts — while _what
  must pass to deploy_ and _who approves production_ are policy layered on top, and keeping them
  separate is what makes both reusable).

## Prerequisites

- **Prior topics**: [topic 6 Version Control & Git](./06-version-control-and-git.md) (branches, PRs,
  the trunk the pipeline triggers on), [topic 15 Software Testing](./15-software-testing.md) (the
  gates a pipeline runs), [topic 30 Software Engineering Practices](./30-software-engineering-practices.md)
  (review, trunk-based flow), [topic 50 Containers & Orchestration](./50-containers-and-orchestration.md)
  (the image artifact you build and deploy), and [topic 51 Cloud & IaC](./51-cloud-and-iac.md) (the
  environments you deploy into).
- **Tools & environment**: a macOS/Linux terminal; a **GitHub** repo (public, so Actions is free);
  the `gh` CLI; **Python** at a recent stable release with type hints and `mypy` for automation
  scripts; a container registry and a deploy target (from topics 50/51); optionally `cosign`/an SLSA
  provenance tool; Neovim/VSCode with YAML + Python LSPs (DD-17).
- **Assumed knowledge**: opening a PR against a trunk (topic 06); running a test suite from the CLI
  (topic 15); building a container image (topic 50); provisioning an environment (topic 51).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: **GitHub Actions** remains free for standard runners on public repositories
  and is the widely adopted CI/CD platform for open source. The workflow syntax (jobs/steps/matrix/
  caching/`environment`/`secrets`, reusable and composite workflows) is stable; left correctly
  version-unpinned, but pin specific action versions (by SHA for supply-chain safety) at drafting.
- 2026-07-12 — verified (GAP for plan owner): supply-chain tooling is the fastest-moving part —
  **SLSA** (provenance levels) and artifact signing (Sigstore/`cosign`) evolve; teach the concepts
  (provenance, attestation, signature verification) and pin exact tool versions/commands only when the
  examples are drafted. DORA metrics remain the standard delivery-performance frame.

## Items

- The pipeline as one artifact: build → test → package → deploy, triggered by a commit/PR, producing a
  single promotable artifact.
- GitHub Actions hands-on: jobs and steps, matrix builds across versions/OSes, dependency and build
  caching, and uploading/downloading artifacts between jobs.
- Environments, secrets, and approvals: protected `environment`s, secret injection, and required
  reviewers as the policy gate on production.
- Reuse without copy-paste: reusable workflows and composite actions to factor shared pipeline logic.
- CD strategies: blue-green, canary, and progressive delivery — shifting traffic gradually with an
  automatic rollback on a bad signal.
- Release automation and supply chain: versioning/changelogs, and provenance/attestation/signing
  (SLSA, Sigstore) so consumers can verify what they run.

## Tensions & trade-offs — when NOT to reach for this

- **A slow or flaky pipeline is worse than none**: if CI takes 40 minutes or fails randomly,
  developers learn to ignore or bypass it, and the gate stops protecting anything. Pipeline speed and
  reliability are first-class — cache aggressively, parallelize, and quarantine flakes, or the whole
  discipline erodes.
- **Not every project needs canary and provenance**: blue-green, progressive delivery, and full SLSA
  provenance solve real risks at scale, but on a small internal service they're operational weight
  with little payoff. Match the deployment strategy to the blast radius, not to the trend.
- **Gates can ossify**: every mandatory check is a tax on every change. A gate that blocks more than it
  catches — a redundant lint, a duplicative test tier — should be demoted to a warning. The goal is
  the fewest gates that keep production safe, not the most gates possible.

## Lineage — why it beat the alternative

- CI/CD grew out of the pain of "integration hell" and big-bang releases: continuous integration
  (merge and test constantly) answered the first, and _Continuous Delivery_ (Humble and Farley, 2010)
  named the discipline of keeping software always-releasable through an automated pipeline. The
  _Accelerate_/DORA research then showed empirically that elite delivery performance — frequent,
  low-risk deploys with fast recovery — correlates with these practices, which settled the debate in
  their favor. Managed platforms like GitHub Actions made the pipeline itself version-controlled and
  reusable. This hands a reliable, automated release path to
  [topic 89 Platform Engineering & Developer Experience](./89-platform-engineering-and-devex.md)
  (golden paths built on it) and depends on the containers and infrastructure of
  [topic 50 Containers & Orchestration](./50-containers-and-orchestration.md) and
  [topic 51 Cloud & IaC](./51-cloud-and-iac.md).

## Worked examples

Colocated under `cicd-and-release-engineering/learning/code/`; each is a runnable GitHub Actions
workflow (public-repo free) plus typed Python automation, `mypy`-clean (DD-20/DD-30/DD-34).

- **beginner** — a CI workflow that runs a matrix of test jobs with dependency caching and uploads a
  build artifact; verify it goes green on a PR.
- **intermediate** — add a protected `environment` with a required reviewer and secret injection, and
  factor shared steps into a reusable/composite workflow; verify a deploy job waits for approval.
- **advanced** — a progressive-delivery/canary deploy that shifts traffic and auto-rolls-back on a bad
  signal, plus artifact signing + provenance so the deployed image is verifiable.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a full commit-to-production pipeline for a small containerized service on GitHub
  Actions — matrix CI with caching and artifacts, a protected production environment with approval,
  reusable workflow factoring, a canary/progressive deploy with automatic rollback, and a signed
  artifact with provenance — reproducible on a free public repo.
- **Concepts exercised**: [ ] matrix build + caching + artifacts [ ] protected environment + secrets +
  approval [ ] reusable/composite workflow [ ] canary/progressive deploy + rollback [ ] artifact
  signing + provenance [ ] a typed deploy/automation script.
- **Ordered steps**:
  1. `.../learning/capstone/.github/workflows/ci.yml` — matrix test jobs, dependency cache, artifact
     upload. Verify the workflow passes on a PR and produces the artifact.
  2. `.../learning/capstone/.github/workflows/deploy.yml` — a protected `environment` with a required
     reviewer + secrets, deploying the built image. Verify the deploy blocks until approval and injects
     secrets safely (never logged).
  3. `.../learning/capstone/.github/actions/` — factor shared logic into a reusable/composite workflow.
     Verify both CI and deploy consume it with no duplication.
  4. `.../learning/capstone/code/rollout.py` — a typed progressive/canary rollout that watches a health
     signal and rolls back on failure, plus image signing + provenance. Verify a simulated bad deploy
     auto-rolls-back and the artifact's signature/provenance verifies.
- **Acceptance criteria**: CI is green with caching and artifacts; production is gated by approval;
  shared logic is reused not copied; a bad canary rolls back automatically; the artifact is signed and
  its provenance verifies; all Python is type-annotated and `mypy`-clean.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Continuous Delivery** — Jez Humble, David Farley (2010). The book that named and defined the
  discipline of continuous delivery.
- **The DevOps Handbook** — Gene Kim, Jez Humble, Patrick Debois, John Willis (2016). Widely read
  synthesis connecting CI/CD practice to organizational flow.
- **Accelerate** — Nicole Forsgren, Jez Humble, Gene Kim (2018). The empirical research base (DORA
  metrics) behind modern CI/CD and release-engineering best practice.

**Papers & articles**

- **DORA (DevOps Research and Assessment)** — Google Cloud DORA team (ongoing). The primary empirical
  research program behind the deployment-frequency/lead-time metrics used across the industry.
  <https://dora.dev/>
- **GitHub Actions Documentation** — GitHub (ongoing). The authoritative reference for the most widely
  adopted CI/CD platform in open source. <https://docs.github.com/actions>

---

← Previous: [51 · Cloud & IaC](./51-cloud-and-iac.md) · Next: [53 · Creating AI-Powered Apps](./53-creating-ai-powered-apps.md) →
