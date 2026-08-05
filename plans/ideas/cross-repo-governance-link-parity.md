# Cross-repo governance citation and anchor parity

One-line summary: every link gate this repository runs stops at its own repo root and skips inline
code, so the cross-repo citations and heading anchors that governance documents inherit from the OSE
siblings are, by construction, never checked.

> Demoted from a full `backlog/` plan to a two-pager on 2026-08-05. The backlog folder never grew
> past a three-line stub; it was captured from an upstream `adopt-cursor-platform-binding` Phase 8
> hand-off that no longer exists in this repository's history.

## Problem / context

The link validator is repo-bounded and deliberately blind to inline code. In
`apps/rhino-cli/src/application/docs/links.rs`, the scan options carry a single `repo_root`, and
`extract_links` runs `strip_inline_code_spans` on every line before matching link syntax. A reference
written as `` `some/path.md#anchor` `` is therefore invisible to the gate, on purpose. Meanwhile the
[Documentation Linking Convention](../../repo-governance/conventions/formatting/linking.md) requires
relative paths for real links — which a cross-repo reference cannot be. So the entire class of
cross-repo citation is unverifiable with today's tooling, and the repository has quietly accumulated
a lot of it.

Measured on the current tree: 16 distinct `plans/done/<date>__<slug>` folder citations appear as
inline code across `docs/`, `repo-governance/`, `plans/`, and `.claude/`; 14 of them name folders
that do not exist in this repository. A representative case is
[the SDLC Gate Standard's byte-identity section](../../docs/reference/sdlc-gate-standard.md#rhino-cli-byte-identity-boundary),
which cites `plans/done/2026-07-03__unify-rhino-cli-sdlc-parity/tech-docs.md#4-rhino-cli-source-identity-standard`
— a path _and_ an anchor fragment, neither of which resolves here, and no gate reports it.

**This repository's version of the problem is not the upstream one.** `beaver-nest` stands outside
the `ose-public` ↔ `ose-primer` content-parity loop, and its `apps/rhino-cli` is an explicit fork
rather than a member of the three-repo byte-identity boundary — see
[Related Repositories](../../docs/reference/related-repositories.md#consequences-of-standing-outside-the-parity-loop).
The original stub framed this as validating shared anchors _during multi-repo landings_, which
describes the loop, not this repo. Here the failure mode is one-directional **adoption drift**:
governance text adopted from upstream arrives carrying upstream's claims, paths, and anchors, and
nothing re-checks them locally. That is observable today —
[the SDLC Gate Standard](../../docs/reference/sdlc-gate-standard.md) states in its Divergence Policy
that "the standardization layer is identical across all 3 repos" and records a Parity Status verified
across `ose-public`, `ose-primer`, and `ose-private`: a live assertion, inside a `beaver-nest`
document, about a set `beaver-nest` is not part of.

## Why now

The repo reset and the rebrand both just finished, and both were link-heavy. The reset alone
surfaced 61 broken links repo-wide in one run, and the standard remedy applied across the sweep was
to strip the hyperlink and keep the citation as plain text — which converts a checked link into an
unchecked string. That remedy was correct and is not being second-guessed here; the point is that it
scaled the unchecked-citation surface in a single pass, and nothing measures it. The rebrand then
demonstrated the failure mode inside a single repo: two links broke purely through anchor-fragment
drift while still pointing at the right file, and only the pre-push hook caught them
([rebrand learnings](../done/2026-08-01__beaver-nest-rebrand/learnings.md), Phase 11 addendum).
Across repos there is no equivalent catch at all. Compounding it, all three wirings of the validator
(`.husky/pre-push`, `main-ci.yml`, `pr-quality-gate.yml`) pass `--exclude plans/done`, so archived
plans' own outbound links are not scanned either.

## Prior art / precedents

- [Documentation Linking Convention](../../repo-governance/conventions/formatting/linking.md) — its
  Anchor Links section documents the `broken-anchor` finding and the GitHub slug algorithm the
  validator implements; the same slug logic is what a cross-repo anchor check would need.
- [Large-Scale Rename Plan Link Management](../../repo-governance/development/workflow/large-scale-rename-plan-link-management.md)
  — the in-repo precedent, written after anchor fragments drifted away from the headings they encode.
- [Repository Validation](../../repo-governance/development/quality/repository-validation.md) —
  describes `md links validate` as a full-repo scan, making the repo boundary explicit.
- [Tri-repo rhino-cli byte-identity drift gate](./tri-repo-rhino-cli-byte-identity-gate.md) — a
  sibling idea for a standing cross-repo diff gate; it carries the same unresolved run-location and
  cross-repo-access questions, so the two should be designed together or not at all.
- [Related Repositories](../../docs/reference/related-repositories.md) — defines the four-repo family
  and states plainly that `beaver-nest` does not participate in parity syncs in either direction.

## Proposed direction (sketch)

Treat this as **citation** validation rather than link validation, in two independent layers so the
cheap half can ship without the expensive half:

- **In-repo citation check.** Recognize a restricted shape of repo-relative path written as inline
  code — paths rooted at `plans/`, `docs/`, `repo-governance/`, `apps/`, `specs/` — and report the
  ones that do not resolve, anchors included. Pair it with an explicit escape marker for citations
  that are deliberately dangling (historical references to deleted plans are legitimate and must stay
  writable).
- **Adoption manifest + anchor-surface diff.** Record, for each document adopted from an OSE sibling,
  the upstream path it came from. A periodic job then compares heading-slug _sets_ rather than full
  text: a slug that no longer exists upstream flags every inbound anchor here as suspect. Comparing
  slug surfaces avoids demanding content parity, which this repo explicitly does not want.

Because `apps/rhino-cli` here is a fork, prototyping either layer costs the three-repo byte-identity
boundary nothing — and equally, buys the loop nothing until someone reimplements it upstream.

## Rough scope & non-goals

In scope: detecting and reporting stale citations and anchors; a documented citation shape with an
escape marker; an adoption manifest for documents inherited from siblings.

Out of scope:

- Auto-fixing, auto-rewriting, or auto-de-linking any citation — detection only.
- Joining the `ose-public` ↔ `ose-primer` content-parity loop, or any sync into `beaver-nest` in
  either direction.
- Returning this repository's forked `apps/rhino-cli` to the three-repo byte-identity boundary.
- Validating external `https://github.com/...` URLs, which is a network concern with a different
  failure mode and different flakiness.
- Changing the GitHub slug algorithm or the existing relative-link behaviour of `md links validate`.
- Sweeping the tree to repair every stale citation; the gate lands first, the cleanup is separate
  work sized from whatever the gate reports.

## Risks & open questions

- Where would a cross-repo check execute, given one sibling is private and the others are cloned
  independently with no submodule wiring? (open — and identical to the open question blocking the
  byte-identity gate idea.)
- Who maintains an adoption manifest, and what updates it when upstream renames a heading? A manifest
  that rots is worse than no manifest. (open)
- Recognizing inline-code paths will produce false positives on illustrative and hypothetical paths,
  which agent definitions and convention documents use heavily. The escape-hatch shape is undesigned.
  (open)
- Whether this is worth building at all for a repo outside the parity loop, versus the far cheaper
  competing option: rewrite the inherited cross-repo assertions so `beaver-nest` documents simply
  stop making claims about the loop. That option would close this idea rather than promote it.
  (open)
- Volume is only partly measured: 14 dangling plan-folder citations are confirmed, but no count
  exists across all inline-code path shapes, so the true size of the problem is unknown. (open)

## What success looks like + promotion signal

Success: a citation that goes stale — whether by a local rename, a deletion, or an upstream heading
change in a document this repo adopted — is reported by a gate rather than discovered by chance
during unrelated work, and the count of unverifiable cross-repo references in the tree is a known
number instead of an unmeasured one.

Promotion signal: promote to a `backlog/` plan when the cheaper competing option above has been
explicitly evaluated and rejected, **and** either the run-location question for a cross-repo check is
answered, or scope is deliberately narrowed to the in-repo inline-code citation check, which needs no
cross-repo access at all. A concrete trigger to force that decision: the first adopted upstream
governance document that lands here with a citation or anchor already broken on arrival and no gate
saying so.
