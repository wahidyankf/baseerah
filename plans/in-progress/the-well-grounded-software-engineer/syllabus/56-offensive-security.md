# 56 · Offensive Security (red team, Kali) (By Example, Python + shell †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python + shell † · Learn 156 / Drill 256 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: thinking like an attacker to defend better — reconnaissance, scanning, exploitation, and
web-app attacks — taught **ethics-first and lab-local only**. Every technique is exercised **exclusively**
against deliberately vulnerable targets you own and run locally (OWASP Juice Shop, DVWA, a local vuln VM)
or your own app; **never** against systems you are not explicitly authorized to test. `†`: Python + shell
driving standard tooling (nmap, sqlmap, Burp/ZAP). Pairs with [`57-defensive-security`](./57-defensive-security.md)
(the blue-team counterpart) and applies [`55-it-and-application-security`](./55-it-and-application-security.md).

> **ETHICS + LEGAL (DD-15, hard rule)**: this topic teaches authorized security testing only. All labs
> are self-hosted, isolated, and owned by the learner. Unauthorized access to any system is illegal and
> out of scope. The topic opens with the ethics/authorization/scope-of-engagement framing and repeats the
> "authorized targets only" rule at every hands-on step.

## Why this exists · the big idea

- **The problem before the solution**: you cannot defend what you don't understand as an attacker — a
  defender who has never chained a real exploit guesses at what matters and hardens the wrong things.
- **Keep-this-if-you-forget-everything**: an attacker turns your assumptions into attack surface and needs
  only one working chain — thinking offensively (recon → exploit → write-up) tells you which weaknesses are
  actually reachable, so defense targets reality. Authorized, lab-local targets only.
- **Big ideas touched**: `layering-and-leaks` (attacks find the gap between trust boundaries),
  `correctness-vs-pragmatism` (one working exploit beats a hundred theoretical ones — reachability over
  completeness).

## Prerequisites

- **Prior topics**: [topic 55 IT / Application Security](./55-it-and-application-security.md) (OWASP Top 10, threat modeling, crypto),
  [topic 17 Security Essentials](./17-security-essentials.md) (injection, auth), and
  [topic 5 Just Enough Bash](./05-just-enough-bash.md) (tool driving).
- **Tools & environment**: a macOS/Linux terminal; an **isolated local lab** — deliberately vulnerable
  targets you own (OWASP Juice Shop, DVWA, a local vuln VM) on a private/host-only network; standard
  tooling (nmap, sqlmap, an intercepting proxy — ZAP/Burp) driven from Python/shell. **No** target you are
  not authorized to test.
- **Assumed knowledge**: the OWASP Top 10 + how vulns manifest (topic 55); shell + running CLI tools
  (topic 05); HTTP/requests (topic 11).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: tools (nmap, sqlmap, ZAP/Burp) are correctly left version-unpinned — their CLI
  syntax is stable across releases and a pinned number would go stale fast. Ethics/authorization framing
  (self-owned/authorized-only labs, written scope, no third-party targets) matches current OWASP/PTES
  rules-of-engagement practice, unchanged.
- 2026-07-12 — verified (GAP for plan owner): the file names OWASP Juice Shop, DVWA, nmap, sqlmap, and
  Burp/ZAP but does **not** mention Kali Linux anywhere. If Kali is wanted as the named lab OS, add it
  explicitly — it is free, Debian-derived, GPL-family, no licensing concern. Otherwise the OS-agnostic
  tool-only framing is intentional and correct as-is.

## Items

- **Ethics, authorization, and scope of engagement first** (rules of engagement, legal boundaries,
  responsible disclosure) — the gate every later step passes through.
- The attack lifecycle: reconnaissance → scanning/enumeration → exploitation → post-exploitation (lab
  framing).
- Reconnaissance & scanning: host/port/service discovery + enumeration (nmap) against the lab.
- Web-app attacks (hands-on, lab-local): injection (SQLi with sqlmap), XSS, broken auth/access control,
  against Juice Shop/DVWA.
- Using an intercepting proxy (ZAP/Burp) to inspect and tamper with requests.
- Writing up a finding: reproduction, impact, remediation (the bridge to blue team).

## Tensions & trade-offs — when NOT to reach for this

- **Offense informs but doesn't equal defense**: a pentest proves a vuln is reachable but is a point-in-time
  snapshot. Treating a passed pentest as "secure" is the classic mistake — absence of _found_ bugs is not
  absence of bugs.
- **Tooling vs understanding**: sqlmap/Metasploit make exploitation push-button, which teaches the button,
  not the mechanism. Leaning on tools without understanding the underlying flaw produces a script-runner who
  can't adapt the attack or explain the fix.
- **When NOT (hard boundary, not a trade-off)**: offensive technique is exercised _only_ against authorized,
  self-owned, isolated targets. This "when not" is a legal and ethical absolute — unauthorized testing is a
  crime regardless of intent.

## Lineage — why it beat the alternative

- Offensive security professionalized from the recognition that defenders who never attack build imaginary
  threat models. Penetration testing, red-teaming, and responsible-disclosure norms (CERT, then bug bounties
  from ~2010) turned adversarial skill into a sanctioned discipline with rules of engagement — precisely so
  the attacker's reachability-first mindset could improve defense _legally_. The output — reproducible
  findings with impact and remediation — is the bridge that hands work to the blue team in
  [`57-defensive-security`](./57-defensive-security.md); the vulnerability classes themselves come from
  [`55-it-and-application-security`](./55-it-and-application-security.md).

## Worked examples

Colocated under `offensive-security/learning/`; Python + shell driving tooling against the local lab only
(DD-20/DD-30). Every example header restates "authorized lab target only".

- **beginner** — a scoped nmap scan of the local lab host: discover services + versions; interpret the
  output.
- **intermediate** — exploit a SQL-injection point in DVWA/Juice Shop (sqlmap + a manual proof); document
  the finding.
- **advanced** — chain a web-app attack (e.g. broken access control → data exposure) through an intercepting
  proxy; write it up with reproduction + impact + remediation.

## Capstone spec — intra-topic (subject → full runnable, lab-local)

- **Goal**: run a small, fully-authorized penetration test against your own local vulnerable lab
  (Juice Shop/DVWA) end to end — recon/scan, enumerate, exploit at least two distinct OWASP-class
  vulnerabilities, and produce a professional finding report (reproduction, impact, CVSS-style rating,
  remediation) — establishing the attacker's view that the defensive topic then detects.
- **Concepts exercised**: [ ] rules-of-engagement + authorization scope stated up front [ ] recon + scan
  (nmap) [ ] enumeration [ ] two distinct exploited vulns (e.g. SQLi + broken access control) [ ] proxy-
  driven request tampering [ ] a finding report with reproduction + impact + remediation.
- **Ordered steps**:
  1. `.../learning/capstone/rules-of-engagement.md` — scope + authorization (self-owned lab) + targets +
     boundaries. Verify only self-hosted lab targets are listed and out-of-scope is explicit.
  2. `.../learning/capstone/recon.sh` — a scoped nmap scan + service enumeration of the lab host. Verify
     discovered services match the known lab and output is captured.
  3. `.../learning/capstone/exploit/` — exploit two distinct vulns (Python/sqlmap/proxy) with a captured
     proof each. Verify each exploit reproduces and is confined to the lab.
  4. `.../learning/capstone/report.md` — per finding: reproduction + impact + severity + remediation.
     Verify each finding is reproducible from the steps and pairs a concrete remediation.
- **Acceptance criteria**: authorization/scope is stated first; recon + two exploits reproduce against the
  self-owned lab only; each finding has reproduction + impact + remediation; nothing touches an unauthorized
  system.
- **Done bar**: runnable end-to-end against the local lab + web-verified + ethics framing present at every
  hands-on step.

## Read more

**Books**

- **Penetration Testing: A Hands-On Introduction to Hacking** — Georgia Weidman (2014). Widely used, beginner-accessible introduction to offensive security methodology and tooling.
- **Metasploit: The Penetration Tester's Guide** — David Kennedy, Jim O'Gorman, Devon Kearns, Mati Aharoni (2011). The canonical guide to the most widely used open-source exploitation framework.
- **The Hacker Playbook 3: Practical Guide to Penetration Testing** — Peter Kim (2018). Popular, practically oriented red-team methodology reference.

**Papers & articles**

- **MITRE ATT&CK** — MITRE Corporation (ongoing). The industry-standard knowledge base of adversary tactics and techniques used to structure red-team engagements. <https://attack.mitre.org/>
- **OWASP Web Security Testing Guide (WSTG)** — OWASP Foundation (ongoing). The de facto standard methodology reference for web application penetration testing. <https://owasp.org/www-project-web-security-testing-guide/>

---

← Previous: [55 · IT / Application Security](./55-it-and-application-security.md) · Next: [57 · Defensive Security](./57-defensive-security.md) →
