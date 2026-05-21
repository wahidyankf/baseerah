---
title: "Overview"
weight: 10000000
date: 2026-05-21T00:00:00+07:00
draft: false
description: "Learn defensive security through annotated examples covering threat detection, incident response, SIEM queries, and security hardening"
tags: ["blue-team", "defensive-security", "soc", "siem", "threat-detection", "by-example"]
---

**Want to learn defensive security through practical, annotated examples?** This by-example guide
teaches SOC and blue team skills through hands-on log analysis, SIEM queries, detection rules,
and incident response playbooks covering **threat detection, triage, containment, and hardening**.

## What Is Blue Team By-Example Learning?

Blue team by-example learning is a **detection-first approach** where you learn through annotated
log samples, SIEM queries, and response procedures rather than abstract theory. Each example shows:

- **What it detects** — the attack technique or anomaly the example identifies
- **Why it indicates compromise** — the behavioral pattern or IOC and its significance
- **How to respond** — triage steps, containment actions, and escalation criteria
- **False positive handling** — how to distinguish malicious activity from legitimate behavior

This approach is **ideal for SOC analysts, incident responders, and detection engineers** building
practical skills in identifying and responding to real-world threats.

## Learning Path

Examples are organized into three progressive levels, from foundational log analysis to advanced
threat hunting and detection engineering.

## Coverage Philosophy

This guide covers the full defensive lifecycle using the MITRE ATT&CK framework as an
organizational reference for mapping detections to adversary techniques.

### What Is Covered

- **Log analysis** — Windows Event Logs, Linux syslogs, application logs, and network logs
- **SIEM queries** — Splunk SPL, Elastic KQL/EQL, Microsoft Sentinel KQL, and Sigma rule writing
  for common attack patterns
- **Threat detection** — detecting reconnaissance, initial access, execution, and persistence
- **Incident triage** — alert prioritization, IOC extraction, timeline reconstruction
- **Incident response** — containment, eradication, and recovery procedures
- **Threat hunting** — hypothesis-driven hunting, anomaly baselines, and proactive detection
- **Detection engineering** — writing, testing, and maintaining detection rules

### What Is Not Covered

- Offensive exploitation techniques (see Red Team by Example)
- Strategic security governance (see CISO by Example)
- General IT infrastructure hardening (see IT Security by Example)

## Prerequisites

- Familiarity with Linux/Unix command line
- Basic understanding of Windows and Linux system administration
- Comfort reading log files and structured data

## Structure of Each Example

Every example follows a consistent five-part format:

1. **Brief Explanation** — what the example detects or responds to and why it matters
   (2-3 sentences)
2. **Scenario Context** — the attack technique being detected and the environment
3. **Annotated Log Sample or Query** — raw logs, SIEM queries, or response scripts with inline
   comments explaining each indicator and decision point
4. **Key Takeaway** — the core defensive insight to retain (1-2 sentences)
5. **Why It Matters** — real-world SOC relevance and detection coverage impact (50-100 words)

## Examples by Level

### Beginner (Examples 1–28)

Examples coming soon.

### Intermediate (Examples 29–57)

Examples coming soon.

### Advanced (Examples 58–85)

Examples coming soon.
