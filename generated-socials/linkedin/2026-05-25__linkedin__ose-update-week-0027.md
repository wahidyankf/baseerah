Posted: Monday, May 25, 2026
Platform: LinkedIn

---

OPEN SHARIA ENTERPRISE
Week 27 / Phase 1, Week 15

Window: 2026-05-18 17:27 +0700 → 2026-05-25 17:25 +0700. ~351 commits across the three repos (ose-public 195, ose-primer 92, ose-infra 64).

Highlights: the ose-projects parent repo is gone — ose-public, ose-primer, and ose-infra are now three independent siblings; rhino-cli finished its Go → Rust port and landed in all three repos under a strict Rust 2024 baseline; Hugo fully removed; multi-harness support added (Amazon Q Developer + a vendor-neutral binding layer).

🌐 Cross-repo

- ose-projects deleted. The parent coordination repo no longer exists. ose-public, ose-primer, and ose-infra are three independent sibling repos living side by side under a plain ~/ose-projects/ directory — no parent git repo binding them. generated-socials moved out of the old parent into ose-public; ecosystem docs now name all three siblings and document ose-infra for the first time.
- rhino-cli: Go → Rust. The repo-management CLI was fully ported to Rust with byte-identical Go parity verified via a shadow-diff harness, then rolled into all three repos. ose-public and ose-infra promoted Rust to canonical (Go preserved at archived/rhino-cli); ose-primer deliberately keeps both versions live as a template (see below). Baseline: Rust 2024 edition, forbid(unsafe_code) at crate roots, pedantic clippy, plus fmt:check / deny:check / check:msrv CI gates.
- Hugo removed everywhere; hugo-commons renamed to golang-link-commons; the swe-hugo-dev agent retired.
- Multi-harness compatibility: added Amazon Q Developer (generated .amazonq/ bridge + deterministic parity guard) and documented Codex, Copilot, Cursor, Windsurf, Junie, Antigravity, and Pi. New harness-compatibility checker/fixer + audit workflow; vendor-audit extended.
- Harness/vendor neutrality: the sync:claude-to-opencode script became the vendor-neutral generate:bindings; npm-script naming and governance language scrubbed of vendor specifics.
- Security guardrail (all repos): a PreToolUse hook now hard-blocks agent access to .env\* files (.env.example only), with a pre-commit guard rejecting any staged env file. A husky guard also prohibits per-repo git identity overrides.

🌳 ose-public

rhino-cli: the bulk of the window — ~80 commits porting every command to Rust (test-coverage, spec-coverage, all docs validators, the repo-governance audit umbrella, agents sync/validate, env init/backup/restore, doctor, git pre-commit), each flipped from Go behind verified parity.

ayokoding-web: large learn-tree reorg — ddd-hex-in-practice renamed to cases (broader scope); information-security split into foundations + roles; infrastructure and security concepts folded into canonical tracks; domains renamed (plural algorithms, platforms/{linux,web,mobile}, personal-development). New procedural architecture track (15 tier files), 4 security by-example tracks (~340 examples), a coding-agents section (OpenClaw, Hermes, Pi), claude-code and Rust CLI tutorials, and FP/OOP examples expanded with TS / Kotlin / C# / Haskell tabs.

Governance: specs-tree-uniform — a canonical five-folder spec tree with CLI domain subdirs enforced by rhino-cli. rust-governance-audit closed out (edition 2024, dual-root forbid-unsafe). New grill-me planning-interrogation skill, TDD-shaped delivery checklists, and a formalized workflow-composability model.

🏗️ ose-infra

rhino-cli Rust migration (Phases 0–7) adapted for infra, including ose-infra-specific Java and contracts validators. Adopted the same env-file guardrail, Amazon Q binding emitter, generate:bindings neutrality pass, Hugo removal, and git-identity guard.

📦 ose-primer

rhino-cli, both ways: unlike ose-public — which archived Go and promoted Rust to canonical — the template deliberately keeps two live versions side by side, rhino-cli-go and rhino-cli-rust. Go is still a perfectly reasonable choice for a CLI in plenty of situations, and as a template ose-primer is more useful showing both. The rust-strictness pass aligned rhino-cli-rust to ose-public's strict baseline (dropping the Go-parity clippy allows). Also picked up the planning-and-dev-practice work (grill-me + TDD checklists), multi-harness + Amazon Q bindings, harness/vendor neutrality, env-file guardrail, Hugo removal, and git-identity guard. Spec adoption plus small fixes across the polyglot demo apps (Python, Kotlin, F#/Giraffe CRUD).

🔜 Next 2–4 weeks

With the tooling foundation now settled — one Rust CLI, a vendor-neutral multi-harness binding layer, and a uniform spec tree across all three repos — focus returns to product. The goal: deploy the first real backend application for ose-public — ose-app-be (F#/Giraffe) shipped to production on top of the DDD + Hexagonal and TDD baselines. First time the platform moves from scaffolding to a running service. Let's see where it gets us.

On Rust: porting rhino-cli changed how I see the language. Rust reads more clearly to me than Go did — the type system carries intent on the surface — and that same explicitness hands AI agents far stronger signals to reason from. Writing it by hand can be painful, but that pain mostly disappears with an AI agent doing the heavy lifting. It is nowhere near my strongest language yet, by a large margin, but this window convinced me it is worth investing in, and I intend to keep learning it.

Insha Allah.

- ose-public: https://github.com/wahidyankf/ose-public
- ose-primer: https://github.com/wahidyankf/ose-primer
- OrganicLever: https://www.organiclever.com/
- Updates: https://www.oseplatform.com/updates/
- Learning: https://www.ayokoding.com/
