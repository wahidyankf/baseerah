---
title: "Overview"
weight: 10000000
date: 2026-05-21T00:00:00+07:00
draft: false
description: "Learn offensive security through annotated examples covering reconnaissance, exploitation, post-exploitation, and adversary simulation"
tags: ["red-team", "offensive-security", "penetration-testing", "adversary-simulation", "by-example"]
---

**Want to learn offensive security through practical, annotated examples?** This by-example guide
teaches adversarial techniques through hands-on tool usage, scripts, and attack scenarios covering
**reconnaissance, exploitation, post-exploitation, and adversary simulation**.

> **Ethical Use Notice:** All examples are for authorized penetration testing, CTF competitions,
> lab environments, and defensive understanding only. Never apply offensive techniques against
> systems without explicit written authorization.

## What Is Red Team By-Example Learning?

Red team by-example learning is a **technique-first approach** where you learn through annotated
tool output, exploit scripts, and adversary playbooks rather than abstract theory. Each example shows:

- **What it does** — step-by-step annotations documenting attack flow, tool output, and system state
- **Why it works** — the vulnerability or misconfiguration being exploited and the underlying mechanism
- **When to use it** — which phase of an engagement this technique applies to
- **Detection surface** — what artifacts the technique leaves for defenders to detect

This approach is **ideal for security professionals** pursuing penetration testing certifications
(OSCP+, PNPT, HTB CPTS) or learning adversary simulation for defensive purposes.

## Learning Path

Examples are organized into three progressive levels, from foundational recon and scanning to
advanced adversary simulation and evasion.

## Coverage Philosophy

This guide covers the full offensive engagement lifecycle using the MITRE ATT&CK framework as
an organizational reference.

### What Is Covered

- **Reconnaissance** — passive OSINT, active scanning, service enumeration, network mapping
- **Initial access** — exploitation of common vulnerabilities, phishing simulation, credential attacks
- **Execution and persistence** — shell payloads, scheduled tasks, startup persistence mechanisms
- **Privilege escalation** — local privilege escalation on Linux and Windows
- **Lateral movement** — credential reuse, pass-the-hash, pivoting techniques
- **Exfiltration simulation** — data staging, covert channel basics
- **Post-exploitation** — situational awareness, credential dumping, living-off-the-land binaries

### What Is Not Covered

- Defensive detection and response (see Blue Team by Example)
- Strategic risk management and governance (see CISO by Example)
- General IT security hardening (see IT Security by Example)

## Prerequisites

- Familiarity with Linux/Unix command line
- Basic understanding of networking (TCP/IP, ports, protocols)
- Access to a legal lab environment (e.g., HackTheBox, TryHackMe, local VM)

## Structure of Each Example

Every example follows a consistent five-part format:

1. **Brief Explanation** — what the technique demonstrates and its place in the attack chain
   (2-3 sentences)
2. **Scenario Context** — target environment, assumed access level, and engagement phase
3. **Annotated Tool Output or Script** — commands, tool output, or exploit code with inline
   comments explaining each step and artifact
4. **Key Takeaway** — the core offensive insight and its defensive implication (1-2 sentences)
5. **Why It Matters** — real-world relevance for penetration testers and red teamers (50-100 words)

## Examples by Level

### Beginner (Examples 1–28)

Examples coming soon.

### Intermediate (Examples 29–57)

Examples coming soon.

### Advanced (Examples 58–85)

Examples coming soon.
