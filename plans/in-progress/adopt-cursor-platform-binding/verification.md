# Cursor Verification Record

Recorded during Phase 1 execution. Access date: 2026-07-28.

## U1 — The canonical Cursor model-ID slug for Composer 2.5

**Question:** What slug does Cursor document for Composer 2.5 (non-fast)?

**Method:** `web-researcher` survey of Cursor docs (cursor.com/docs), subagent configuration docs, and model listing pages.

**Finding:** First-party Cursor documentation and forum examples use the bare slug `composer-2.5` for Composer 2.5. The fast variant is separately documented as `composer-2.5-fast`.

**Confidence label:** Verified

**Source URL + access date:** https://cursor.com/docs/models (2026-07-28); Cursor Community Forum subagent model discussion (2026-07-28)

**Fallback taken?** no

## U2 — Whether bracket parameter syntax is accepted in an agent file's `model:` field

**Question:** Does `composer-2.5[fast=false]` work in `.cursor/agents/*.md` frontmatter?

**Method:** Documentation survey; no first-party schema documents bracket parameters on agent `model:` fields.

**Finding:** No documented acceptance of bracket syntax in agent frontmatter. Emitter uses bare slug `composer-2.5`.

**Confidence label:** Unverified

**Source URL + access date:** https://cursor.com/docs/agent/subagents (2026-07-28)

**Fallback taken?** yes — bare slug without brackets

## U3 — What Cursor does with an unrecognised `model:` value such as `sonnet`

**Question:** How does Cursor resolve Anthropic tier aliases (`sonnet`, `opus`, `haiku`) in subagent frontmatter?

**Method:** Documentation survey only in Phase 1; empirical probe deferred to Phase 5 live subagent session.

**Finding:** Cursor docs do not document behaviour for unrecognised Claude aliases.

**Confidence label:** Unverified

**Source URL + access date:** https://cursor.com/docs/agent/subagents (2026-07-28)

**Fallback taken?** yes — documented as unverified until Phase 5 probe

## U4 — Whether the two staff-confirmed defects are fixed in the installed Cursor version

**Question:** Are subagent `model:` frontmatter ignored / auto-switch to `composer-2.5-fast` bugs fixed?

**Method:** Changelog and forum re-check via `web-researcher`.

**Finding:** Staff described fixes as rolling out with unclear scope. Phase 5 empirical probe is the authoritative gate.

**Confidence label:** Unverified

**Source URL + access date:** https://forum.cursor.com/ (2026-07-28)

**Fallback taken?** yes — ship with defect documented; Phase 5 probe gates lock-in
