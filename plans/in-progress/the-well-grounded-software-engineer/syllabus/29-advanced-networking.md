# 29 · Advanced Networking (Annotated-concept, Python \*)

**prd row**: Pass 2 · Depth, Design & Craft · Annotated-concept · Python \* · Learn 129 / Drill 229 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the deep networking pass — the OSI/TCP-IP models, addressing/subnetting, transport
internals, the modern application layer (HTTP/1.1→2→3, TLS, WebSockets), diagnostics, and edge/delivery
infrastructure. The practical slice is the prerequisite
[`12-networking-essentials`](./12-networking-essentials.md); code appears in Python where it fits (`*`),
otherwise annotated diagrams and real tool output.

## Why this exists · the big idea

- **The problem before the solution**: the essentials explain one clean request; production networks fail
  in layered, subtle ways — congestion, MTU, TLS negotiation, a bad subnet — you cannot debug blind.
- **Keep-this-if-you-forget-everything**: the layered model _is_ the debugging tool — every network problem
  localizes to a layer, so you bisect down the stack instead of guessing.
- **Big ideas touched**: `layering-and-leaks` — OSI/TCP-IP layering and exactly where each layer leaks into
  the one above; `consistency-latency-throughput` — latency, bandwidth, and throughput are three different
  things you must stop conflating.

## Prerequisites

- **Prior topics**: [topic 12 Networking Essentials](./12-networking-essentials.md) (HTTP, DNS, sockets,
  `curl`/`dig`) and [topic 4 Just Enough Python](./04-just-enough-python.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x**; the diagnostic CLIs **`ping`**,
  **`traceroute`**, **`dig`**, **`netstat`/`ss`**, and **`tcpdump`** (may need `sudo`); network access.
- **Assumed knowledge**: what happens when you hit a URL (topic 12); TCP vs UDP at a glance; reading a
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
- Real-time transport trade-offs: WebSockets vs Server-Sent Events (SSE) vs WebTransport vs WebRTC —
  full-duplex vs one-way, TCP vs QUIC/UDP, and when each fits (chat, live feeds, low-latency media).
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

## Read more

**Books**

- **Computer Networking: A Top-Down Approach** — James F. Kurose & Keith W. Ross (2000; multiple editions since). Widely used standard networking textbook covering the protocol stack from application to physical layers.
- **TCP/IP Illustrated, Volume 1: The Protocols** — W. Richard Stevens (1994; 2nd ed. by Kevin R. Fall, 2011). The classic deep-dive reference on TCP/IP internals, including congestion control.
- **High Performance Browser Networking** — Ilya Grigorik (2013). Free, practitioner-focused guide to TCP, TLS, HTTP/2, and modern web performance networking. <https://hpbn.co/>

**Papers & articles**

- **RFC 8446 – The Transport Layer Security (TLS) Protocol Version 1.3** — IETF (2018). The current standard defining modern TLS handshakes and cipher negotiation. <https://www.rfc-editor.org/rfc/rfc8446>
- **RFC 9113 – HTTP/2** — IETF (2022). The current standard for HTTP/2 framing, multiplexing, and stream prioritization. <https://www.rfc-editor.org/rfc/rfc9113>
- **RFC 9000 – QUIC: A UDP-Based Multiplexed and Secure Transport** — IETF (2021). Defines QUIC, the transport underlying HTTP/3. <https://www.rfc-editor.org/rfc/rfc9000>
- **RFC 9114 – HTTP/3** — IETF (2022). Defines HTTP/3 as a mapping of HTTP semantics onto QUIC streams. <https://www.rfc-editor.org/rfc/rfc9114>
- **RFC 5681 – TCP Congestion Control** — IETF (2009). Defines slow start, congestion avoidance, and fast retransmit/recovery, the algorithms behind TCP's congestion control. <https://www.rfc-editor.org/rfc/rfc5681>

---

← Previous: [28 · Build Your Own ORM & Query Builder](./28-build-your-own-orm-and-query-builder.md) · Next: [30 · Software Engineering Practices](./30-software-engineering-practices.md) →
