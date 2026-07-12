# 51 · Cloud & Infrastructure as Code (Annotated-concept, HCL/YAML †)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · HCL/YAML † · Learn 151 / Drill 251 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the cloud service-model mental map (compute/storage/network/managed services) and
declarative infrastructure as code — the Terraform/OpenTofu plan → apply → destroy lifecycle, state,
modules, and the cost/security discipline of cloud. `†`: the "language" is HCL + YAML; `*`-style annotated
where a concept is diagrammed. Runnable locally against a local provider (LocalStack / a local backend) so
no paid cloud account is required (DD-20). Containers/K8s are [`50-containers-and-orchestration`](./50-containers-and-orchestration.md).

## Why this exists · the big idea

- **The problem before the solution**: infrastructure clicked together by hand in a console is unreviewable,
  unreproducible, and drifts — nobody can say what exists, why, or how to rebuild it after a loss.
- **Keep-this-if-you-forget-everything**: describe infrastructure as declarative code and let the tool
  compute the plan to converge reality to it — infra becomes reviewable, reproducible, and diff-able, at the
  cost of a state file you must guard.
- **Big ideas touched**: `mechanism-vs-policy` (you declare the desired infra; the provider reconciles it),
  `determinism-vs-emergence` (code-defined infra buys reproducibility and drift detection).

## Prerequisites

- **Prior topics**: [topic 50 Containers & Orchestration](./50-containers-and-orchestration.md) (the
  workload to run), [topic 5 Just Enough Bash](./05-just-enough-bash.md)
  (CLI + env), and [topic 11 Backend Essentials](./11-backend-essentials.md) (the service being deployed).
- **Tools & environment**: a macOS/Linux terminal; **Terraform** (or OpenTofu — note the license split,
  DD-15) + a **local provider / LocalStack** so `apply` needs no paid account; a local state backend;
  `docker` (from topic 50). No real cloud credentials committed (secrets rule).
- **Assumed knowledge**: containers + a deployable workload (topic 50); shell + env vars (topic 05); reading
  declarative config (YAML from topic 50).

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
- Serverless & edge compute: functions-as-a-service, cold starts, edge runtimes, and when managed compute
  beats provisioned servers.
- Infrastructure testing: plan-diff review, policy-as-code (OPA/Sentinel), and ephemeral
  apply-then-destroy test environments.

## Tensions & trade-offs — when NOT to reach for this

- **State is the soft underbelly**: IaC's power comes from a state file mapping code to real resources — and
  that file holds secrets, corrupts under concurrent applies, and drifts the moment someone clicks in the
  console. Remote locked state and no-manual-changes discipline are the _cost_ of the reproducibility.
- **Abstraction vs control**: modules and higher-level frameworks (CDK, Terragrunt) buy reuse and charge a
  leaky abstraction over the provider; when the abstraction breaks you debug two layers. Start with plain
  resources and abstract only when duplication actually hurts.
- **When NOT to use it**: a one-off throwaway environment or a tiny personal project may not repay the IaC
  setup cost — click it and move on. IaC earns its keep for environments that must be reproduced, reviewed,
  or rebuilt.

## Lineage — why it beat the alternative

- IaC answered the "works in prod, nobody knows why" era of hand-configured servers (snowflakes) —
  CFEngine/Puppet/Chef brought convergence, then Terraform (2014) brought declarative, provider-agnostic,
  plan-before-apply infra reviewable like code. The 2023 HashiCorp BSL relicensing and the OpenTofu fork are
  a live reminder that even your tooling's license is an engineering input (DD-15). The invariant: infra you
  can review, reproduce, and diff beats infra you merely remember — the same determinism-over-emergence bet
  as immutable images in [`50-containers-and-orchestration`](./50-containers-and-orchestration.md).

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

## Read more

**Books**

- **Terraform: Up & Running** — Yevgeniy Brikman (1st ed., 2019; 3rd ed., 2022). The standard practical reference for Terraform and infrastructure-as-code workflows.
- **Infrastructure as Code** — Kief Morris (2nd ed., 2020). Broader, tool-agnostic treatment of IaC principles and patterns.

**Papers & articles**

- **AWS Well-Architected Framework** — Amazon Web Services (ongoing). The canonical vendor framework for cloud architecture quality attributes and the "well-architected" mindset. <https://docs.aws.amazon.com/wellarchitected/latest/framework/welcome.html>
- **Google Cloud Architecture Framework** — Google Cloud (ongoing). Google's equivalent canonical framework for cloud system design tradeoffs. <https://cloud.google.com/architecture/framework>
- **Terraform Documentation** — HashiCorp (ongoing). The official, authoritative reference for the Terraform configuration language and providers. <https://developer.hashicorp.com/terraform/docs>

---

← Previous: [50 · Containers & Orchestration](./50-containers-and-orchestration.md) · Next: [52 · CI/CD & Release Engineering](./52-cicd-and-release-engineering.md) →
