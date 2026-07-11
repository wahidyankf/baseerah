# 21 · Advanced Networking (Annotated-concept, Python \*)

**prd row**: Pass 2 · Solidify the Core · Annotated-concept · Python \* · Learn 121 / Drill 221 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the deep networking pass — the OSI/TCP-IP models, addressing/subnetting, transport
internals, the modern application layer (HTTP/1.1→2→3, TLS, WebSockets), diagnostics, and edge/delivery
infrastructure. The practical slice is the prerequisite
[`10-networking-essentials`](./10-networking-essentials.md); code appears in Python where it fits (`*`),
otherwise annotated diagrams and real tool output.

## Prerequisites

- **Prior topics**: [topic 10 Networking Essentials](./10-networking-essentials.md) (HTTP, DNS, sockets,
  `curl`/`dig`) and [topic 04 Just Enough Python](./04-just-enough-python.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x**; the diagnostic CLIs **`ping`**,
  **`traceroute`**, **`dig`**, **`netstat`/`ss`**, and **`tcpdump`** (may need `sudo`); network access.
- **Assumed knowledge**: what happens when you hit a URL (topic 10); TCP vs UDP at a glance; reading a
  `curl -v`/`dig` transcript.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: HTTP/3 = **RFC 9114**, QUIC transport = **RFC 9000**, QUIC's TLS 1.3 integration
  = **RFC 9001** (TLS 1.3 itself = **RFC 8446**) — all finalized unchanged IETF standards in 2026; HTTP/3
  supported by >95% of major browsers. CIDR/subnet arithmetic stable; `tcpdump`/`traceroute` output
  formats stable (spot-check at authoring). (datatracker.ietf.org / rfc-editor.org)

## Items

- Network models: the OSI 7-layer and TCP/IP 4-layer models, encapsulation, per-layer responsibilities.
- Link & Internet layers: MAC/ARP, IPv4 vs IPv6 addressing, subnetting/CIDR, NAT, routing basics.
- Transport deep: TCP (handshake, reliability, flow & congestion control) vs UDP; ports & sockets.
- Application deep: DNS resolution detail, HTTP/1.1 vs HTTP/2 vs HTTP/3 (QUIC), the TLS handshake,
  WebSockets.
- Diagnostics & performance: ping/traceroute/dig/netstat/tcpdump; latency vs bandwidth vs throughput.
- Delivery & edge: firewalls, proxies, load balancers, CDNs, and the well-known ports.

## Worked examples

Colocated under `advanced-networking/learning/code/`; annotated diagrams + real tool output + small Python
where it fits (DD-20/DD-30).

- **request-trace** — trace a URL request end to end (DNS → TCP handshake → TLS → HTTP) as an annotated
  WCAG-accessible sequence diagram, corroborated with `curl -v`/`tcpdump`.
- **subnetting** — subnet a network by hand: split a CIDR block, compute host counts, gateway and
  broadcast addresses (with a small Python verifier).
- **diagnostics** — read real `dig`, `traceroute`, and `tcpdump` output and explain each record/hop.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a networking diagnostics toolkit + report: a Python CIDR/subnet calculator (validated
  against hand math), a script that traces and annotates a real request path (DNS→TCP→TLS→HTTP), and a
  written analysis of captured `traceroute`/`tcpdump` output — a runnable + documented deliverable.
- **Concepts exercised**: [ ] CIDR/subnet arithmetic (hosts/gateway/broadcast) [ ] the layered model
  applied to a real packet [ ] TCP handshake + TLS handshake narrated [ ] reading `traceroute`/`tcpdump`
  [ ] latency vs bandwidth vs throughput reasoning.
- **Ordered steps**:
  1. `.../learning/capstone/code/subnet.py` — a CIDR calculator (network/broadcast/host-range/host-count).
     Verify its output matches a hand-computed example for at least two prefixes.
  2. `trace.py` — resolve a host, open a connection, and print an annotated DNS→TCP→TLS→HTTP timeline.
     Verify it emits a real status line and each stage is labelled.
  3. `analysis.md` — capture and annotate real `traceroute` + `tcpdump` output for one request. Verify each
     hop/packet is explained and tied to a layer.
- **Acceptance criteria**: the subnet calculator is correct on multiple prefixes; the trace narrates all
  four stages against live output; the analysis correctly maps observed traffic to the model.
- **Done bar**: runnable end-to-end (calculator + trace) + produces the analysis + web-verified.

---

← Previous: [20 · Advanced Algorithms](./20-advanced-algorithms.md) · Next: [22 · Software Engineering Practices](./22-software-engineering-practices.md) →
