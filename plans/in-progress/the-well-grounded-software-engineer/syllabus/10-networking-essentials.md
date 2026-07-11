# 10 · Networking Essentials (By Example, Python)

**prd row**: Pass 1 · First Working Software · By Example · Python · Learn 110 / Drill 210 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **usable slice** — what happens when you hit a URL, HTTP in practice, DNS, and a
sockets intro, all from the terminal. OSI layering, subnetting, congestion control, HTTP/2-3, and
`tcpdump` analysis go to [`21-advanced-networking`](./21-advanced-networking.md) (DD-11).

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md) (socket examples are
  Python).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x**; the **`curl`**, **`dig`**, and
  **`ping`** CLIs; network access to reach a real URL.
- **Assumed knowledge**: reading/writing basic Python; comfort running terminal commands. No prior
  networking background required.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: Python `socket` TCP API (`bind`/`listen`/`accept`/`connect`,
  `create_connection()`) unchanged; `curl -v` verbose format (`>` sent, `<` received, `*` info) unchanged;
  **TLS 1.3 (RFC 8446)** is current, 1-RTT handshake description accurate. `dig` output format stable but
  not primary-source re-quoted — spot-check `man dig` at authoring. (docs.python.org / ietf.org)

## Items

- **Client/server model**; what happens when you hit a URL (DNS → TCP → TLS → HTTP) at a practical level.
- **HTTP in practice**: request/response, methods, status codes, headers, ports; HTTP vs HTTPS.
- **DNS basics**: names → IPs; resolving with `dig`/`nslookup`.
- **Sockets intro**: a TCP client/server with Python `socket`; TCP vs UDP at a glance.
- **Practical tooling** from the terminal: `curl`, `dig`, `ping`.

## Worked examples

Colocated under `networking-essentials/learning/code/`; runnable Python + annotated CLI transcripts
(DD-20/DD-30).

- **beginner** — `curl -v` and `dig` a real URL and read the output; annotate an HTTP request/response.
- **intermediate** — a Python `socket` TCP echo client/server, line by line.
- **advanced** — a tiny stdlib HTTP client; contrast a UDP datagram exchange.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a tiny TCP request/response protocol over Python `socket` (a line-based echo/command
  server + client) and a companion script that resolves a host with `dig`, opens a TCP connection, and
  narrates the DNS→TCP→HTTP path — runnable end-to-end on localhost.
- **Concepts exercised**: [ ] TCP client/server with `socket` [ ] request/response framing [ ] DNS
  resolution [ ] HTTP request read via stdlib [ ] TCP-vs-UDP contrast.
- **Ordered steps**:
  1. `.../learning/capstone/code/server.py` + `client.py` — a line-based TCP echo/command server.
     Verify `python3 server.py &` then `python3 client.py` round-trips messages.
  2. Add a small command set (e.g. `PING`→`PONG`, `TIME`→timestamp). Verify each command's response.
  3. `explore.py` — resolve a real host, open a socket, issue a minimal HTTP GET, print the status line.
     Verify it prints a real HTTP status.
- **Acceptance criteria**: server/client round-trip works; commands return correct responses; the explore
  script narrates the resolution + connection + response; a UDP variant is contrasted in prose.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [09 · Backend Essentials](./09-backend-essentials.md) · Next: [11 · Just Enough TypeScript](./11-just-enough-typescript.md) →
