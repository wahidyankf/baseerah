# 50 · Containers & Orchestration (By Example, YAML/CLI †)

**prd row**: Pass 3 · Build for the Real World · By Example · YAML/CLI † · Learn 150 / Drill 250 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: packaging and running services with containers and Kubernetes — images, Dockerfiles,
Compose for local multi-service, then K8s objects (Pods/Deployments/Services/Ingress), config/secrets,
and health/scaling. `†`: the "language" is Dockerfiles + YAML + the `docker`/`kubectl` CLIs against a
real app (the [`11-backend-essentials`](./11-backend-essentials.md) service). Ingress-vs-Gateway-API is
handled with the license/standards-awareness lens (DD-15). Cloud provisioning is
[`51-cloud-and-iac`](./51-cloud-and-iac.md).

## Why this exists · the big idea

- **The problem before the solution**: "works on my machine" and hand-run servers fail the moment you need
  reproducibility, many identical copies, and automatic recovery — manual ops don't scale and drift silently.
- **Keep-this-if-you-forget-everything**: package the app with its environment into an immutable image, then
  declare the desired state and let the orchestrator's control loops continuously reconcile reality to it —
  you describe _what you want_, not the steps to get there.
- **Big ideas touched**: `mechanism-vs-policy` (you declare desired state; the orchestrator is the
  reconciling mechanism), `determinism-vs-emergence` (immutable images buy reproducibility; self-healing
  emerges from control loops).

## Prerequisites

- **Prior topics**: [topic 11 Backend Essentials](./11-backend-essentials.md) (an app to containerize),
  [topic 5 Just Enough Bash](./05-just-enough-bash.md) (CLI fluency), and
  [topic 10 SQL Essentials](./10-sql-essentials.md) (a DB to run as a companion service).
- **Tools & environment**: a macOS/Linux terminal; **Docker** (or a compatible engine) + `docker compose`;
  a local Kubernetes (kind/minikube/k3d) + `kubectl`; the backend app + a DB image. Images pinned by
  digest where practical (DD-15/supply-chain).
- **Assumed knowledge**: running a service locally + env-based config (topic 11); shell basics (topic 05);
  reading YAML.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: **Ingress is frozen; Gateway API is the recommendation.** kubernetes.io states
  verbatim "The Ingress API has been frozen" and "recommends using Gateway instead of Ingress" — Ingress
  stays GA/stable with no removal planned but takes no further changes. (kubernetes.io/docs/concepts/services-networking/ingress)
- 2026-07-12 — verified: **`docker compose` (v2 CLI plugin, space not hyphen)** is current; v1
  (`docker-compose`) is EOL. Use `compose.yaml` with **no `version:` key** (the current recommended form).
  (docs.docker.com/compose/release-notes)
- 2026-07-12 — verified: file stays version-agnostic on K8s object versions (good) — current stable is
  Kubernetes v1.36.2 (2026-06-09), v1.37.0 due 2026-08-26; keep it unpinned. Multi-stage builds, non-root
  users, `.dockerignore`, digest-pinned images remain current supply-chain best practice.

## Items

- Containers: images vs containers, layers, Dockerfiles, multi-stage builds, non-root users, `.dockerignore`.
- Local multi-service: Docker Compose (app + DB + cache) for a realistic dev stack.
- Kubernetes objects: Pods, Deployments, ReplicaSets, Services, Ingress (frozen) vs Gateway API (DD-15).
- Config & secrets: ConfigMaps, Secrets (and why secrets need real handling), env injection.
- Health & scaling: liveness/readiness probes, resource requests/limits, horizontal scaling.
- The build → ship → run loop from the CLI.

## Worked examples

Colocated under `containers-and-orchestration/learning/`; Dockerfiles + Compose + K8s manifests, each
applied from the CLI (DD-20/DD-30).

- **beginner** — a multi-stage Dockerfile for the backend app (non-root, small image); run it with `docker
run`.
- **intermediate** — a Compose stack (app + DB + cache) brought up with `docker compose up`; verify the app
  talks to the DB.
- **advanced** — K8s manifests (Deployment + Service + Ingress + ConfigMap + Secret + probes) applied to a
  local cluster; verify the app is reachable and self-heals on pod kill.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take the Backend-Essentials service and fully containerize + orchestrate it — a hardened
  multi-stage image, a Compose dev stack (app + DB + cache), then a Kubernetes deployment
  (Deployment/Service/Ingress + ConfigMap/Secret + liveness/readiness probes + resource limits) on a local
  cluster — proving reachability, config injection, and self-healing from the CLI.
- **Concepts exercised**: [ ] a multi-stage non-root Dockerfile [ ] a Compose multi-service stack
  [ ] K8s Deployment + Service + Ingress [ ] ConfigMap + Secret injection [ ] liveness/readiness probes
  [ ] self-healing (pod kill → reschedule) [ ] resource requests/limits.
- **Ordered steps**:
  1. `.../learning/capstone/Dockerfile` — a multi-stage, non-root image. Verify `docker build` succeeds and
     the running container serves the app.
  2. `compose.yaml` — app + DB + cache. Verify `docker compose up` brings all three up and the app reaches
     the DB.
  3. `k8s/` manifests — Deployment + Service + Ingress + ConfigMap + Secret + probes + limits, applied with
     `kubectl apply`. Verify the app is reachable through the Ingress and config comes from the ConfigMap.
  4. Delete a pod (`kubectl delete pod`). Verify the Deployment reschedules it and the app recovers with no
     manual step.
- **Acceptance criteria**: the image is multi-stage + non-root; Compose runs the full stack; the K8s app is
  reachable with injected config; killing a pod self-heals; secrets are injected (not baked into the image).
- **Done bar**: runnable end-to-end on a local cluster + web-verified.

## Read more

**Books**

- **Kubernetes: Up and Running** — Kelsey Hightower, Brendan Burns, Joe Beda (1st ed., 2017; 3rd ed. with Lachlan Evenson, 2022). Written by Kubernetes co-creators/maintainers; the standard introductory and reference text.
- **Docker Deep Dive** — Nigel Poulton (multiple editions). Widely used practical reference for Docker fundamentals and container runtime internals.
- **The Kubernetes Book** — Nigel Poulton (annual editions). Popular, frequently updated companion covering Kubernetes objects and operations.

**Papers & articles**

- **Large-scale cluster management at Google with Borg** — Abhishek Verma et al. (2015), EuroSys. Describes the internal Google system that directly inspired Kubernetes's design. <https://research.google/pubs/large-scale-cluster-management-at-google-with-borg/>
- **Kubernetes Documentation** — Cloud Native Computing Foundation (ongoing). The official, canonical reference for Kubernetes concepts and APIs. <https://kubernetes.io/docs/home/>

---

← Previous: [49 · Information Architecture & SEO](./49-information-architecture-and-seo.md) · Next: [51 · Cloud & IaC](./51-cloud-and-iac.md) →
