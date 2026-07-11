# 35 · Cloud & Infrastructure as Code (Annotated-concept, HCL/YAML †)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · HCL/YAML † · Learn 135 / Drill 235 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the cloud service-model mental map (compute/storage/network/managed services) and
declarative infrastructure as code — the Terraform/OpenTofu plan → apply → destroy lifecycle, state,
modules, and the cost/security discipline of cloud. `†`: the "language" is HCL + YAML; `*`-style annotated
where a concept is diagrammed. Runnable locally against a local provider (LocalStack / a local backend) so
no paid cloud account is required (DD-20). Containers/K8s are [`34-containers-and-orchestration`](./34-containers-and-orchestration.md).

## Prerequisites

- **Prior topics**: [topic 34 Containers & Orchestration](./34-containers-and-orchestration.md) (the
  workload to run), [topic 05 Just Enough Bash](./05-just-enough-bash.md)
  (CLI + env), and [topic 09 Backend Essentials](./09-backend-essentials.md) (the service being deployed).
- **Tools & environment**: a macOS/Linux terminal; **Terraform** (or OpenTofu — note the license split,
  DD-15) + a **local provider / LocalStack** so `apply` needs no paid account; a local state backend;
  `docker` (from topic 34). No real cloud credentials committed (secrets rule).
- **Assumed knowledge**: containers + a deployable workload (topic 34); shell + env vars (topic 05); reading
  declarative config (YAML from topic 34).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: **Terraform is on the Business Source License 1.1** (HashiCorp writes it "BSL 1.1,
  also known as BUSL 1.1" — both abbreviations are sanctioned; prefer spelling it out once as "Business
  Source License 1.1 (BUSL 1.1, sometimes abbreviated BSL)"). **OpenTofu is MPL-2.0** (Mozilla Public
  License 2.0), a Linux Foundation project. (opentofu.org / github.com/opentofu/opentofu)
- 2026-07-12 — verified: plan → apply → destroy lifecycle, state, modules, providers/resources/variables/
  outputs are unchanged Terraform-core concepts. LocalStack remains the standard no-paid-account local
  AWS-API-compatible provider.

## Items

- Cloud service models: IaaS/PaaS/SaaS; compute, storage, network, managed DB/queue/cache; regions/AZs.
- Infrastructure as Code: declarative vs imperative; why IaC (reproducibility, review, drift detection).
- Terraform/OpenTofu: providers, resources, variables, outputs, modules; the plan → apply → destroy
  lifecycle; state + why state is sensitive (DD-15 license note: Terraform BSL vs OpenTofu MPL).
- Cost & security discipline: least-privilege IAM, tagging, cost awareness, secrets never in state files.
- Environments: dev/stage/prod via variables + workspaces/modules.
- Drift, import, and safe destroy.

## Worked examples

Colocated under `cloud-and-iac/learning/`; HCL + YAML applied against a local provider (DD-20/DD-30).

- **beginner** — a minimal Terraform config (provider + one resource) run through `init → plan → apply →
destroy` against a local backend.
- **intermediate** — variables + outputs + a reusable module; a dev-vs-stage difference driven by a
  variable.
- **advanced** — provisioning the backend workload (container/service + its config) as code, with
  least-privilege + tagging + secrets kept out of state.

## Capstone spec — intra-topic (subject → full runnable, local provider)

- **Goal**: describe the deployment of the backend service entirely as code — a reusable Terraform/OpenTofu
  module (variables/outputs), a dev and a stage environment driven by variables, provisioned through the
  full `plan → apply → destroy` lifecycle against a local provider (LocalStack), with least-privilege +
  tagging + secrets kept out of state — a reproducible, reviewable infrastructure definition.
- **Concepts exercised**: [ ] provider + resource + the plan/apply/destroy lifecycle [ ] variables + outputs
  - a reusable module [ ] dev vs stage from one config [ ] least-privilege + tagging [ ] secrets kept out of
    state [ ] drift detection via re-plan.
- **Ordered steps**:
  1. `.../learning/capstone/` — a module (provider + the service's resources + variables + outputs). Verify
     `terraform init && plan` produces a clean, readable plan against the local provider.
  2. `apply` it, then re-`plan`. Verify `apply` creates the resources and the re-plan shows no drift.
  3. Add a dev and a stage environment from the same module driven by variables. Verify each environment
     differs only by its variable values.
  4. Confirm secrets are supplied via variables/env (never hard-coded, never in committed state) + tagging +
     least-privilege, then `destroy`. Verify `destroy` removes everything and no secret is present in any
     committed file.
- **Acceptance criteria**: the full lifecycle runs against the local provider; re-plan shows no drift; two
  environments come from one module; no secret appears in committed files or state; resources are tagged
  and least-privilege.
- **Done bar**: runnable end-to-end (local provider) + web-verified.

---

← Previous: [34 · Containers & Orchestration](./34-containers-and-orchestration.md) · Next: [36 · Data Engineering](./36-data-engineering.md) →
