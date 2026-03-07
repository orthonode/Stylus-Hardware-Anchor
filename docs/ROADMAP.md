# Stylus Hardware Anchor — Operational Roadmap
# ORTHONODE SYSTEMS™ | Type A — Core Infrastructure

> Derived from root ROADMAP.md and current CLAUDE.md state.
> Source of truth for active phase and open items.

---

## Current State

- **Network:** Arbitrum Sepolia (testnet-only)
- **Contract:** `0xD661a1aB8CEFaaCd78F4B968670C3bC438415615`
- **On-chain activity:** 89+ verified transactions
- **Phase:** Phase 1 — Security Hardening & Developer Release (active)
- **Grant:** $25,000 USD — Stylus Sprint / Questbook — submitted Feb 2026 — under review

---

## Phase 1 — Security Hardening & Developer Release (Months 1–6)

**Scope:** Testnet-only (Arbitrum Sepolia). $25,000 USD total.

### Open Items

| Item | Status | Notes |
|:-----|:-------|:------|
| Hardened Stylus verification contract | In progress | Counter sync issue in single-call `verifyReceipt` — fix deferred to v0.2 |
| cargo-fuzz harness — ≥1,000,000 execution cycles | Pending | Not yet documented / published |
| Keccak-256 parity test vectors — ≥10,000 vectors | Partial | 10,000+ vectors validated in tests; final published artifacts pending |
| Gas benchmark documentation | Done | 12,523 gas/receipt (N=50), 118,935 gas/single — see BENCHMARKS.md |
| Threat model & security documentation | Done | SECURITY.md, docs/ARCHITECTURE.md present |
| Tagged v1.0.0 developer-ready release | Done | v1.0.0 released on GitHub |

### Completion Criteria (Phase 1)

- [ ] Hardened contract publicly tagged with deterministic build instructions
- [ ] ≥1,000,000 fuzzing executions completed and logs published
- [ ] ≥10,000 cryptographic parity test vectors published and reproducible
- [ ] Gas benchmarks documented and independently executable
- [ ] Threat model and security documentation publicly released

---

## Known Bugs / Improvement Items

- **Single-call `verifyReceipt` counter sync issue** — deferred to v0.2. Batch verification is primary interface.
- **ruint pin** — ruint forced to v1.12.3 via Cargo.toml git patch. Do not upgrade without testing against current stylus-sdk.
- **ABI camelCase** — snake_case method calls silently revert with `0x`. All ABI callers must use camelCase.
- **Keccak-256 padding** — 0x01 (Ethereum). Not 0x06 (NIST SHA-3). Documented in docs/TECHNICAL_CHALLENGES.md.

---

## Phase 2 — Future Scope (Separate Grant Required)

Not in current scope. Requires Arhant's explicit decision after Phase 1 completion.

| Item | Notes |
|:-----|:------|
| Python SDK (`anchor-verifier`) — PyPI | Deferred |
| Rust crate (`stylus-hardware-primitives`) — crates.io | Deferred |
| Professional third-party security audit | Deferred |
| Mainnet deployment (Arbitrum One) | Deferred — requires audit completion |
| ARM Cortex-M and RISC-V hardware targets | Deferred |
| Orbit-based DePIN ecosystem integrations | Deferred |

---

## Decision Log

| Decision | Date | Details |
|:---------|:-----|:--------|
| ruint pinned to v1.12.3 | — | Cargo.toml git patch; v1.17.2 required unstable edition2024 which breaks build |
| Single-call verifyReceipt deferred | — | Counter sync issue; batch verification is primary interface |
| Mainnet deferred | — | No professional audit yet; testnet-only for Phase 1 |
| Grant amount confirmed | 2026-03-06 | $25,000 USD (not $49,000) — confirmed by Arhant Barmate |

---

ORTHONODE SYSTEMS™ | Infrastructure Labs // Physical Verification Layer
https://orthonode.xyz | @OrthonodeSys | github.com/orthonode | Arhant Barmate
