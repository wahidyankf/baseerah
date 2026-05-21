---
title: "Overview"
date: 2026-05-21T00:00:00+07:00
draft: false
weight: 10000000
description: "Coding agents — AI-powered tools that read, write, and execute code on your behalf"
tags: ["coding-agents", "ai", "openclaw", "hermes", "pi", "learning-path"]
---

Coding agents are AI-powered systems that can read your codebase, write and edit files, run
shell commands, and complete multi-step engineering tasks with minimal human intervention.
They differ from AI chat tools in one key way: they have tools that act on real files and
processes, not just generate text.

## Three Agents, Three Levels of Abstraction

This section covers three tools that represent different points on the spectrum:

| Tool         | What it is                                      | Abstraction level                          |
| ------------ | ----------------------------------------------- | ------------------------------------------ |
| **OpenClaw** | Autonomous AI agent framework, channel-based UI | High — framework for building agents       |
| **Hermes**   | Meta's JavaScript engine for React Native       | Low — runtime that coding agents run on    |
| **Pi**       | Minimal coding agent harness, 4 primitive tools | Medium — minimal harness for custom agents |

Understanding all three gives you a complete picture: Pi shows you what a coding agent needs
at its core, OpenClaw shows how to build a production agent framework around those primitives,
and Hermes shows what happens at the JavaScript runtime layer that powers agent tool execution
on mobile.

## OpenClaw — Agent Framework

[OpenClaw](/en/learn/artificial-intelligence/coding-agents/openclaw/overview) is an
open-source autonomous AI agent framework that runs locally and connects to any LLM
(Claude, GPT, DeepSeek). It uses messaging platforms (WhatsApp, Telegram, Slack) as its
UI, organizes capabilities into markdown-based Skills files, and runs everything on your
device. Created November 2025, renamed to OpenClaw January 2026, it had 247,000+ GitHub
stars by March 2026.

Key concepts: Skills System, Channel abstraction, Gateway control plane, local-first
execution, SOUL.md / AGENTS.md / TOOLS.md persona files.

## Hermes — JavaScript Runtime

[Hermes](/en/learn/artificial-intelligence/coding-agents/hermes/overview) is Meta's
open-source JavaScript engine built specifically for React Native mobile apps. It solves
the cold-start problem by compiling JavaScript to bytecode at build time (AOT compilation),
eliminating JIT overhead on launch. Hermes V1 became the default engine in React Native 0.84
(February 2026).

Key concepts: Ahead-of-Time compilation, Hades garbage collector, JSI (JavaScript Interface),
New Architecture integration, version coupling with React Native.

## Pi — Minimal Agent Harness

[Pi](/en/learn/artificial-intelligence/coding-agents/pi/overview) is a minimal,
open-source terminal-based coding agent harness. It ships with only four tools: Read, Write,
Edit, and Bash. Everything else — sub-agents, plan mode, MCP support — is an extension users
build or install. Pi v0.75.4 was released May 2026 by Earendil Inc.

Key concepts: Four primitive tools, TypeScript extension API, tree-structured sessions,
context engineering (AGENTS.md / SYSTEM.md / skills), multi-provider LLM support (15+
providers), self-extensibility.

## Learning Path

Recommended order for software engineers new to coding agents:

1. **[Pi — Beginner](/en/learn/artificial-intelligence/coding-agents/pi/by-concept/beginner)**
   — Start here. Pi's minimal design makes the fundamental concepts clear: what tools an agent
   needs, how context engineering works, and what "agentic loop" means.

2. **[OpenClaw — Beginner](/en/learn/artificial-intelligence/coding-agents/openclaw/by-concept/beginner)**
   — After Pi, OpenClaw shows how those primitives scale into a full agent framework with
   channels, skills, and memory.

3. **[Hermes — Beginner](/en/learn/artificial-intelligence/coding-agents/hermes/by-concept/beginner)**
   — If you work in React Native, Hermes shows what happens in the JavaScript runtime that
   your agent or app depends on.

No prior AI or agent experience required. Basic JavaScript/TypeScript familiarity helps for
code examples.
