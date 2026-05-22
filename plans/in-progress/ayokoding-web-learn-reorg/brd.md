# BRD — Learn-Tree Reorganization

## Problem Statement

`apps/ayokoding-web/content/en/learn/` accumulated 1117 markdown files across 6 domains without a single consistent shape. Three failure modes are now visible:

1. **Reader cannot predict where to look.** A learner who finds `software-engineering/automation-tools/git/by-example/` and then visits `information-security/foundations/by-example/` sees almost the same shape — but the same learner visiting `information-security/concepts/explanation/` sees a completely different one. There is no rule they can carry across the site.
2. **Authors cannot predict where to write.** When the maker-checker-fixer agent family adds new content (e.g., the recent OpenClaw / Hermes / Pi Coding Agent / Claude Code work), it has to invent a shape on each topic. The result was 50 broken internal links and 15 stale `_index.md` files shipped to `main` [Judgment call — counts from commit diff summaries, not re-audited] (see commits `dfe53a1d1`, `bebc1552e`, `a83568ecd` [Repo-grounded], repaired in `9035b41d9` and `90dfee30d` [Repo-grounded] on 2026-05-22).
3. **Validation cannot enforce.** `ayokoding-cli links check` catches dead links but cannot catch shape drift — there is no canonical shape to compare against. New `concepts/`, `foundations/`, `cases/` folders keep appearing.

## Why Now

The Hugo-to-Next.js migration is finished (Hugo artifacts removed on 2026-05-22 in commit `2cd845b05` [Repo-grounded]). With no live Hugo, the directory tree is purely Next.js content and we can rename folders, move files, and add redirects without fighting two routing engines simultaneously. Every additional week of new content authored against the current chaotic shape multiplies the eventual migration cost.

## Cost of Inaction

- Each new topic added at the wrong shape requires a follow-up link-repair and index-regen pass (demonstrated by commits `9035b41d9` and `90dfee30d`). [Repo-grounded]
- The `apps-ayokoding-web-by-example-checker` and `apps-ayokoding-web-in-the-field-checker` agents cannot enforce structural rules they cannot define; the platform's three-track learning model is hollow without a tree to match it.
- Inbound links from past LinkedIn posts and external references progressively decay as informal renames happen.

## Cost of Action

- ~80-120 directories renamed or relocated. `git mv` preserves blame so commit archaeology survives.
- ~150-250 redirect entries to author and ship in Next.js config.
- One round of content cross-link sweep (`/en/learn/...` references inside markdown bodies).
- One pass of `generate-indexes.ts` to refresh every affected `_index.md` TOC.

The Hugo-era cleanup proved this is mechanical, not creative. The 2026-05-22 link-fix commit (`9035b41d9` [Repo-grounded]) demonstrated the cleanup pattern works at scale.

## Success Metrics

| Metric                                                                                                                          |                     Today |                                          Target |
| ------------------------------------------------------------------------------------------------------------------------------- | ------------------------: | ----------------------------------------------: |
| Distinct sub-topic vocabularies (`by-concept`, `by-example`, `in-the-field`, `concepts`, `explanation`, `foundations`, `cases`) |                         7 |                                               3 |
| Topics matching canonical shape                                                                                                 |                   partial |                                            100% |
| `platform-<name>/` folders                                                                                                      |                         3 |                                               0 |
| Broken internal links (`ayokoding-cli links check`)                                                                             |   0 (post-2026-05-22 fix) |                                    0 maintained |
| Stale `_index.md` files (`generate-indexes.ts --validate`)                                                                      | 0 (post-2026-05-22 regen) |                                    0 maintained |
| Redirect entries for renamed URLs                                                                                               |                         0 | 100% coverage of every URL renamed in this plan |

## Affected Roles

- **Readers** — the only stakeholder whose URL stability we owe externally; redirect map is for them.
- **Content authors and the maker/checker/fixer agent family** — gain a single rule to follow and validate against.
- **Self (repository owner)** — owns deployment risk and the SEO impact of the redirect transition.

## Non-Goals

- We are not changing the three-track pedagogy (`by-concept` / `by-example` / `in-the-field`). The skills that author those tracks are stable and out of scope.
- We are not authoring new content. Empty track folders that should exist but don't (e.g., a domain has `by-example/` but no `by-concept/`) stay empty until separate authoring plans fill them.
- We are not changing top-level domain count or removing thin domains (`business/`, `it-governance/`). They get the same shape treatment as everyone else.
- We are not touching Indonesian (`content/id/`). A parallel id-reorg is a separate plan if it happens at all.

## Risk Register

| Risk                                                    | Likelihood | Impact | Mitigation                                                                                          |
| ------------------------------------------------------- | ---------- | ------ | --------------------------------------------------------------------------------------------------- |
| Inbound link decay before redirects deploy              | medium     | medium | Land redirects in the same PR as the renames; verify with `curl -I` against staging                 |
| Coverage regression below 82% threshold [Repo-grounded] | low        | high   | Track folder moves don't touch test files; pre-push hook gates push                                 |
| `_index.md` regeneration loop produces unexpected churn | medium     | low    | Regenerate once at end of each phase, commit deltas                                                 |
| Agent family (checker/fixer) hardcoded to old paths     | low        | medium | Search agent definitions for `concepts/explanation`, `foundations/`, `platform-` before final merge |
| Worktree drift if multiple phases run in parallel       | low        | medium | Single worktree, sequential phases — the plan is not parallelizable                                 |

## Decision: Proceed

Reasons:

- Cost of inaction grows linearly with content; cost of action is bounded and largely mechanical.
- The 2026-05-22 cleanup proved the link-validation + index-regen tooling is reliable.
- The three-track skill family already exists; this plan operationalizes what the skills assume.
