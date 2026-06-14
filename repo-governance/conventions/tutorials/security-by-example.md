---
title: Security By-Example Tutorial Convention
description: Standards for security-domain by-example tutorials using tool output, lab scenarios, and annotated security artifacts — extends the SWE By-Example Convention
category: explanation
subcategory: conventions
tags:
  - convention
  - tutorial
  - by-example
  - security
  - tool-output
created: 2026-05-21
---

# Security By-Example Tutorial Convention

## Purpose

This convention **extends the [SWE By-Example Tutorial Convention](./swe-by-example.md) for the
security domain**, adapting the code-first model to security tool output, lab scenarios, shell
sessions, and annotated security artifacts.

**Base requirements**: Security by-example tutorials inherit all standards from the
[SWE By-Example Convention](./swe-by-example.md) and override only the differences documented
below.

**Target audience**: Software engineers without a formal security background who want to learn
security through hands-on scenarios rather than abstract theory.

## How It Differs from SWE By-Example

### Artifact type

| SWE By-Example                                    | Security By-Example                                             |
| ------------------------------------------------- | --------------------------------------------------------------- |
| Runnable source code (`go run`, `python`, `java`) | Tool output, shell sessions, configs, SIEM queries, log samples |
| Compile-and-run verification                      | Lab-reproducible with stated prerequisites                      |
| Standard library first                            | Built-in OS tools first                                         |

### Self-containment definition

**SWE by-example**: Copy-paste-runnable with a single command (`go run main.go`).

**Security by-example**: Fully reproducible in a stated lab environment with no hidden steps.
Each example must specify:

- **Lab requirement** — the minimum environment needed (e.g., "Ubuntu 22.04 LTS", "HackTheBox
  VPN connected", "Kali Linux", "local VM running Metasploitable 3")
- **Prerequisites installed** — tools required beyond a base OS install
- **All commands shown** — no "run the previous setup" cross-references

### Annotation semantics (`# =>`)

**SWE by-example**: Annotates variable state and return values.

```go
result := transform(y)  // => result is "20-transformed" (string)
```

**Security by-example**: Annotates what each output field or artifact line means and its
security implication.

```bash
nmap -sV 10.10.10.5
# => -sV: probe open ports to determine service/version info
# Output:
# PORT   STATE SERVICE VERSION
# 22/tcp open  ssh     OpenSSH 7.9 (Debian)
# => Port 22 open: SSH available — test for weak credentials or key reuse
# => OpenSSH 7.9: released 2018, check CVE list for unpatched vulns on this version
# 80/tcp open  http    Apache httpd 2.4.38
# => Port 80 open: HTTP (not HTTPS) — cleartext traffic, potential login form exposure
```

### Coverage metric

**SWE by-example**: 95% of language/framework features.

**Security by-example**: Coverage maps to the domain's primary framework:

- **Foundations (IT Security)**: Coverage of essential security controls — network, crypto,
  hardening, IAM, monitoring, incident response
- **Red Team**: Coverage of MITRE ATT&CK Enterprise tactics — Reconnaissance through Impact
- **Blue Team**: Coverage of MITRE ATT&CK detection surface — detection, triage, hunting,
  response per tactic

Coverage percentages per level follow the same pattern:

- Beginner: 0–40%
- Intermediate: 40–75%
- Advanced: 75–95%

### Mermaid diagram use cases

Security by-example diagrams visualize:

- **Attack chains**: Recon → Initial Access → Execution → Persistence → Lateral Movement
- **Kill chains**: Lockheed Martin or Unified Kill Chain phases
- **Network topologies**: Attacker, DMZ, internal segments, target hosts
- **Incident timelines**: Sequence of detected events leading to compromise
- **Detection logic**: Alert correlation flow, triage decision trees
- **TLS/PKI flows**: Certificate chain, handshake sequence

Same color-blind palette as SWE by-example applies (Blue #0173B2, Orange #DE8F05, Teal #029E73,
Purple #CC78BC, Brown #CA9161).

### Core-first principle for security tools

Apply the same "core features first" principle from [SWE By-Example](./swe-by-example.md), adapted
for security:

**Beginner level — built-in OS tools only (zero specialized tool installation)**:

- Network inspection: `ss`, `netstat`, `ip`, `ping`, `traceroute`, `dig`, `host`, `whois`
- Packet capture: `tcpdump` (ships with most Linux distros)
- Cryptography: `openssl` (standard), `sha256sum`, `gpg`
- File permissions: `ls`, `find`, `stat`, `chmod`, `chown`
- Log reading: `cat`, `grep`, `awk`, `journalctl`, `tail -f`
- SSH: `ssh`, `ssh-keygen`, `scp` (standard OpenSSH)
- Process inspection: `ps`, `top`, `lsof`, `strace`

**Intermediate level — introduce specialized tools with justification**:

- `nmap` — when `ss`/`netstat` are insufficient for remote host discovery
- `gobuster`/`ffuf` — when manual curl enumeration is too slow for coverage
- `Suricata`/`Snort` — when manual log parsing is insufficient for real-time detection
- `Splunk`/`Elastic` — when grep/awk are insufficient for correlation across log sources
- `Vault` — when environment variables are insufficient for secrets management
- Mark each introduction: "Note: This example uses [tool]. Install with: [command]"

**Advanced level — full ecosystem including frameworks**:

- `Metasploit`, `Mimikatz`, `BloodHound` — with explicit authorized-lab framing
- Cloud CLIs (`aws`, `az`, `gcloud`) for cloud security examples
- `volatility3`, `autopsy` for forensics
- SOAR platforms for detection automation

### Ethical use requirements (Red Team content only)

Every Red Team level page (`beginner.md`, `intermediate.md`, `advanced.md`) MUST open with:

```markdown
> **Ethical Use:** All examples are for authorized penetration testing, CTF competitions,
> lab environments, and defensive understanding only. Never apply these techniques against
> systems without explicit written authorization.
```

Foundations and Blue Team level pages do not require this notice.

---

## Five-Part Format (security-adapted)

Every example follows the same five-part structure as SWE by-example, with security-specific
adaptations:

### Part 1: What This Covers (2-3 sentences)

Same as SWE by-example. Must answer:

- What security technique, control, or detection does this example demonstrate?
- Why does it matter in a real environment?
- When would a practitioner use it?

### Part 2: Scenario (1-2 sentences)

Replace "Brief Explanation" with explicit scenario context:

- State the environment (OS, network segment, tool version)
- State authorization framing for offensive examples ("authorized pentest on lab target 10.10.10.5")
- State analyst role for defensive examples ("Tier 1 SOC analyst reviewing alerts")

```markdown
**Scenario:** Authorized internal pentest against a lab Ubuntu 22.04 server at 10.10.10.5.
You have completed host discovery and are performing service enumeration.
```

### Part 3: Annotated Tool Output or Config

Replace "runnable source code" with the security artifact, fully annotated with `# =>`:

- Show the exact command(s) to run
- Show realistic (but fictional/lab) output
- Annotate every output field that matters with `# =>`
- Use fictional IP ranges (10.x.x.x, 192.168.x.x, RFC 5737 documentation ranges)
- Use fictional but plausible hostnames, usernames, hashes

Density target: same 1.0–2.25 annotation lines per non-blank, non-comment content line per example.

### Part 4: Key Takeaway (1-2 sentences)

Same as SWE by-example. For Red Team examples, include the defensive implication:

```markdown
**Key Takeaway:** OpenSSH 7.9 with password authentication enabled is a high-value target;
defenders should enforce key-only auth and monitor for repeated authentication failures (Event
ID 4625 on Windows, auth.log failures on Linux).
```

### Part 5: Why It Matters (50-100 words)

Same as SWE by-example. Production-focused, active voice, specific to the technique.

---

## Coverage Levels

### Beginner (Examples 1–28, 0–40%)

**Focus**: Security fundamentals every engineer should know.

- Built-in OS tools only (zero specialized installs)
- Network basics: reading packet captures, firewall rules, port states
- Cryptography basics: symmetric/asymmetric encryption, hashing, TLS handshake
- System hardening: file permissions, SSH config, PAM, log reading
- Foundational concepts: CVE/CVSS, vulnerability classes, log formats

**Self-containment**: Runnable on any Ubuntu 22.04 LTS install with no additional packages.

### Intermediate (Examples 29–57, 40–75%)

**Focus**: Production-grade controls and specialized tool usage.

- Introduce specialized tools with explicit installation instructions
- Domain-specific patterns: SIEM queries, IDS rules, cloud IAM misconfigs, AD enumeration
- Incident response lifecycle, forensic triage, credential management
- Defender perspective: detection rules, log correlation, alert triage

### Advanced (Examples 58–85, 75–95%)

**Focus**: Expert-level techniques, frameworks, and full-chain scenarios.

- Full attack/defense lifecycle scenarios
- Framework-level tooling: Metasploit, Mimikatz, volatility3, SOAR
- Advanced detection engineering, threat hunting, purple team exercises
- Cloud-native security, container/Kubernetes hardening

---

## Applies To

This convention governs security by-example content in ayokoding-www:

- `information-security/foundations/by-example/` — IT security foundations track
- `information-security/roles/red-team/by-example/` — Red Team offensive track
- `information-security/roles/blue-team/by-example/` — Blue Team defensive track

The CISO track (`information-security/roles/ciso/by-example/`) is governed by the
[Scenario By-Example Tutorial Convention](./scenario-by-example.md), not this convention.

---

## Validation Criteria

Extend the [SWE By-Example validation checklist](./swe-by-example.md#quality-checklist) with:

- [ ] Lab environment clearly stated at example start
- [ ] All commands shown (no hidden prerequisite steps)
- [ ] Fictional IP ranges used (10.x, 192.168.x, RFC 5737)
- [ ] Red Team level pages open with ethical use notice
- [ ] Beginner level uses only built-in OS tools (no specialized installs)
- [ ] Intermediate/Advanced introductions of specialized tools include install command
- [ ] Annotations explain security implication, not just output field name
- [ ] MITRE ATT&CK technique referenced where applicable

---

## Principles Implemented/Respected

- **[Progressive Disclosure](../../principles/content/progressive-disclosure.md)** — Coverage
  levels (Beginner/Intermediate/Advanced) layer complexity progressively; beginners use only
  built-in OS tools while advanced examples introduce full-ecosystem tooling.
- **[No Time Estimates](../../principles/content/no-time-estimates.md)** — Coverage expressed
  as percentages of the domain's primary framework (MITRE ATT&CK, essential security controls)
  rather than time-based estimates; practitioners set their own pace.
- **[Accessibility First](../../principles/content/accessibility-first.md)** — Color-blind
  friendly Mermaid palette (Blue #0173B2, Orange #DE8F05, Teal #029E73, Purple #CC78BC, Brown
  #CA9161) required for all diagrams; WCAG AA compliance throughout.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**
  — Each example must specify lab environment, prerequisites, and all commands explicitly; no
  hidden steps or "run the previous setup" cross-references permitted.

---

## Related Documentation

- [SWE By-Example Tutorial Convention](./swe-by-example.md) — base convention this extends
- [Scenario By-Example Tutorial Convention](./scenario-by-example.md) — for CISO/governance content
- [General Tutorial Convention](./general.md) — base tutorial standards
- [Diagrams Convention](../formatting/diagrams.md) — Mermaid diagram standards
