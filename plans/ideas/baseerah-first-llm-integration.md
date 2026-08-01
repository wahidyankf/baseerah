# Baseerah first LLM integration

One-line summary: give `baseerah-be` its first real AI-assistant capability — currently there is
no capture, no notes, no LLM calls, and no prompt plumbing at all.

> Idea, added 2026-07-31, filed from `baseerah-repo-reset`'s Product Scope § Out of scope.

## Problem / context

`baseerah-repo-reset` scoped `baseerah-be`/`baseerah-fe` as a pure hello-world walking skeleton:
"Every product feature [is out of scope]. No capture, no notes, no LLM calls, no prompt plumbing,
no AI SDK dependency, no scheduling, no posting." That was correct for establishing the engineering
harness, but it means none of Baseerah's actual stated purpose — an AI assistant, per
[Baseerah Vision](../../repo-governance/vision/beaver-nest.md) — exists yet.

## Why now

Not yet — this is a placeholder for the plan that picks the first concrete AI-assistant capability
to build. Choosing a provider/SDK before a feature is scoped would be speculative.

## Prior art / precedents

- [Baseerah Vision](../../repo-governance/vision/beaver-nest.md) — states the product is "an AI
  assistant, a content builder, a posting helper, and a personal workflow engine"; this idea is the
  first slice of the "AI assistant" facet.
- `vercel:ai-architect` agent (already available in this repo's agent roster) — specializes in
  architecting AI-powered applications, choosing AI SDK patterns, and configuring providers; the
  natural agent to drive this once scoped.
- [baseerah-persistence-layer](./baseerah-persistence-layer.md) — an LLM integration that needs to
  remember anything (conversation history, captured notes) depends on this idea landing first or
  alongside it.

## Proposed direction (sketch)

- Pick one small, concrete first capability (e.g., a single free-text capture endpoint that an LLM
  summarizes or tags) rather than building general-purpose prompt plumbing up front.
- Use the Vercel AI SDK, consistent with the `vercel:ai-architect` agent already in this repo's
  toolset and with the `[domain]-be` backend pattern.

## Rough scope & non-goals

In scope: eventually, the first LLM-backed route in `baseerah-be`.

Out of scope (for now): choosing a specific model/provider, prompt design, or any persistence for
LLM output — those depend on the concrete feature this idea is deferred until.

## Risks & open questions

- Which capability is the first LLM-backed feature — capture, notes, or something else? (open —
  determines the whole shape)
- Does this depend on [baseerah-persistence-layer](./baseerah-persistence-layer.md) landing first,
  or can a stateless first LLM call (no storage) ship independently? (open)
- Provider choice and cost/rate-limit implications for a personal-use product? (open)

## What success looks like + promotion signal

Success: `baseerah-be` serves one real LLM-backed route, however small. Ready to promote once a
maintainer picks the first concrete AI capability to build — until then it correctly stays an
under-specified idea.
