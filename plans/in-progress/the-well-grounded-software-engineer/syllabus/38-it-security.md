# 38 · IT Security (Annotated-concept, Python \*)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · Python \* · Learn 138 / Drill 238 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the security body of knowledge an engineer needs — the CIA triad, threat modeling
(STRIDE), the OWASP Top 10 (2025), applied cryptography (hashing/symmetric/asymmetric/TLS), identity
(OAuth2/OIDC deepened), and secure-SDLC practices. `*`: Python where a mechanism is shown runnably (e.g. a
password-hash verifier, a JWT check), else annotated. This is the conceptual spine feeding the two hands-on
security topics — [`39-offensive-security`](./39-offensive-security.md) and
[`40-defensive-security`](./40-defensive-security.md) — and it deepens
[`14-security-essentials`](./14-security-essentials.md).

## Prerequisites

- **Prior topics**: [topic 14 Security Essentials](./14-security-essentials.md) (auth, hashing, injection
  basics), [topic 28 Backend at Scale](./28-backend-at-scale.md) (OAuth2/OIDC, the surface to secure), and
  [topic 04 Just Enough Python](./04-just-enough-python.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with pinned CVE-clean crypto/JWT libs
  (never roll your own crypto); a Markdown editor for the threat model. All work against your own code/data
  only.
- **Assumed knowledge**: tokens vs sessions + password hashing (topic 14); OAuth2/OIDC at a using level
  (topic 28); Python basics (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: **OWASP Top 10:2025** released and current. Order: A01 Broken Access Control,
  A02 Security Misconfiguration, A03 **Software Supply Chain Failures (new)**, A04 Cryptographic Failures,
  A05 Injection, A06 Insecure Design, A07 Authentication Failures (renamed), A08 Software/Data Integrity
  Failures, A09 Security Logging & Alerting Failures (renamed), A10 **Mishandling of Exceptional Conditions
  (new)**; SSRF folded into A01. STRIDE remains the dominant threat-model mnemonic (extended for AI, not
  superseded). (owasp.org/Top10/2025)
- 2026-07-12 — verified: Argon2id baseline `m=19456 KiB (19 MiB), t=2, p=1` (alt `m=47104, t=1, p=1`),
  OWASP first-choice hash. TLS 1.3 preferred, TLS 1.2 the accepted floor. RSA ≥ 2048-bit floor per NIST
  SP 800-131A Rev. 2 (still the finalized rev; Rev. 3 in draft — teaching guidance "RSA ≥ 2048, prefer
  ECC/larger" unaffected). (cheatsheetseries.owasp.org / csrc.nist.gov)
- 2026-07-12 — verified (CORRECTION of framing): **OAuth 2.1 is NOT a ratified RFC** — it is an active IETF
  draft (`draft-ietf-oauth-v2-1-15`, 2026-03-02). Phrase it as "OAuth 2.1 (IETF draft consolidating OAuth
  2.0 + current security BCPs)," not a finalized standard co-equal with RFC 6749. (datatracker.ietf.org)

## Items

- Foundations: the CIA triad, defense in depth, least privilege, threat modeling with STRIDE.
- The OWASP Top 10 (2025): each category, how it manifests, how to prevent it.
- Applied cryptography: hashing vs encryption, symmetric vs asymmetric, digital signatures, TLS, key
  management — "don't roll your own crypto".
- Identity & access: OAuth2/OIDC deepened, JWT pitfalls, session security, MFA.
- Secure SDLC: dependency/supply-chain risk, SAST/DAST/SCA, secrets management, security headers.
- Reading a CVE / advisory and reasoning about exploitability.

## Worked examples

Colocated under `it-security/learning/`; annotated threat models + runnable Python security mechanisms
(DD-20/DD-30).

- **threat-model** — a STRIDE threat model for the backend app, annotated (assets, entry points, threats,
  mitigations).
- **crypto-mechanisms** — runnable Python: verify an argon2id password hash; verify/reject a tampered JWT;
  a digital-signature verify.
- **owasp-walkthrough** — an annotated mapping of the OWASP Top 10 (2025) to concrete code smells +
  fixes in the app.

## Capstone spec — intra-topic (subject → threat-model artifact + runnable mechanisms)

- **Goal**: produce a complete security assessment of the backend app — a STRIDE threat model mapping
  assets/entry-points/threats/mitigations, a mapping of the OWASP Top 10 (2025) to the app with concrete
  prevention notes, and a set of runnable Python mechanisms (argon2id hash verify, tamper-detecting JWT
  check, a signature verify) — the conceptual + hands-on backbone the red/blue topics build on.
- **Concepts exercised**: [ ] a STRIDE threat model [ ] OWASP Top 10 (2025) mapped to the app
  [ ] password hashing done right (argon2id) [ ] JWT tamper detection [ ] a digital-signature verify
  [ ] a secure-SDLC checklist (deps/secrets/headers).
- **Ordered steps**:
  1. `.../learning/capstone/threat-model.md` — STRIDE over the app: assets, entry points, per-category
     threats + mitigations. Verify every entry point has at least one identified threat + mitigation.
  2. `.../learning/capstone/code/crypto.py` — argon2id verify + a JWT integrity check. Verify a correct
     password/token passes and a tampered one is rejected.
  3. `owasp-2025.md` — each Top-10 category mapped to a concrete place in the app + its prevention. Verify
     every category is addressed (present or justified N/A).
  4. `secure-sdlc.md` — a dependency/supply-chain + secrets + security-headers checklist run against the
     app. Verify each item has a concrete status.
- **Acceptance criteria**: the threat model covers every entry point; the OWASP 2025 mapping is complete;
  the crypto mechanisms correctly accept valid and reject tampered inputs; the secure-SDLC checklist is
  filled with concrete statuses.
- **Done bar**: threat-model artifact complete + mechanisms runnable + web-verified (esp. OWASP 2025 list).

---

← Previous: [37 · Creating AI-Powered Apps](./37-creating-ai-powered-apps.md) · Next: [39 · Offensive Security (red team, Kali)](./39-offensive-security.md) →
