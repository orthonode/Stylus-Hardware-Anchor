# Stylus Hardware Anchor (SHA) — Roadmap

## Overview

Stylus Hardware Anchor (SHA) is a reusable hardware identity verification primitive for Arbitrum Stylus contracts. It binds immutable silicon identifiers (ESP32 eFuse values) to on-chain logic, enabling deterministic device identity, replay-safe receipt validation, and firmware-governed execution.

This roadmap reflects **Phase 1 only**.

---

# Phase 1 — Current Grant Scope

**Total Grant Request:** $25,000 USD  
**Duration:** 6 months  
**Network Scope:** Arbitrum Sepolia (testnet only)  
**Milestones:** 1  

Phase 1 focuses strictly on:

- Security hardening of the existing Sepolia prototype  
- Reproducible validation artifacts  
- Threat model and security documentation  
- Tagged developer-ready open-source release  

Developer SDK, package registry publishing, and integration tooling are deferred to Phase 2 under a separate grant.  

No professional third-party audit, mainnet deployment, or ecosystem expansion is included in this scope.

---

# Milestone 1 — Security Hardening & Developer Release (Months 1–6)

**Allocation:** $25,000 USD  
**Scope:** Testnet-only (Arbitrum Sepolia)

## Objective

Strengthen the existing prototype through internal security hardening, systematic fuzz testing, reproducible benchmarking, and formal documentation of security assumptions.

No paid external consulting or professional third-party audit is included in this milestone.

---

## Deliverables

### 1. Hardened Stylus Verification Contract

- Input size enforcement and DoS mitigation  
- Deterministic replay protection using monotonic counters  
- Firmware hash validation gating  
- Clear failure-state handling  

---

### 2. Coverage-Guided Fuzz Testing

- `cargo-fuzz` harness targeting receipt validation logic  
- ≥1,000,000 execution cycles  
- Published fuzz logs and reproduction instructions  
- Edge-case documentation  

---

### 3. Cryptographic Parity Validation

- ≥10,000 device ↔ contract test vectors  
- Deterministic hash equivalence proof  
- Reproducible validation scripts  

---

### 4. Gas Benchmark Documentation

- Preliminary benchmark (~45k gas on Arbitrum Sepolia under controlled test conditions)  
- Reproducible benchmark scripts  
- Published cost profile documentation  

---

### 5. Threat Model & Security Documentation

- Attack surface analysis  
- Trust boundary mapping  
- Explicit security assumptions  
- Known limitations & mitigation notes  

---

## Milestone 1 Budget Breakdown

| Category | Cost (USD) |
|-----------|------------|
| Internal Development & Security Hardening | $21,000 |
| Minimal Hardware (3–5 ESP32-S3 boards for validation testing) | $1,000 |
| Documentation & Developer Release Preparation | $2,000 |
| Contingency (technical remediation buffer) | $1,000 |
| **Total Milestone 1** | **$25,000** |

**Note:**  
No professional third-party audit, paid external review, consulting services, or audit subsidy applications are included in Phase 1.

---

# Phase 1 Summary

- Milestone 1: $25,000 USD  

**Total Phase 1 Grant Request: $25,000 USD**  
**Duration: 6 months (testnet-only)**

---

# Completion Criteria (Phase 1)

Phase 1 will be considered complete when:

- Hardened contract version publicly tagged with deterministic build instructions  
- ≥1,000,000 fuzzing executions completed and logs published  
- ≥10,000 cryptographic parity test vectors published and reproducible  
- Gas benchmarks documented across defined input ranges and independently executable  
- Threat model and security documentation publicly released  

All deliverables will be publicly verifiable through GitHub releases, package registries, and on-chain testnet deployments.

---

# Risk Management (Phase 1)

## Smart Contract Vulnerabilities

Mitigation:

- Coverage-guided fuzz testing  
- Internal security hardening  
- Public repository transparency  
- Professional third-party audit planned for Phase 2 (future grant)

---

## Cryptographic Implementation Risk

Mitigation:

- Deterministic test vectors  
- Cross-platform hash equivalence verification  
- Reproducible validation scripts  

---

## Execution Risk (Solo Founder)

Mitigation:

- Sequential milestone structure  
- Testnet-only scope  
- Clearly bounded deliverables  
- Artifact-based completion metrics  

---

# Phase 2 (Future Grant — Not Included in Current Scope)

Planned future scope (subject to separate grant approval):

- Python SDK (`anchor-verifier`) published to PyPI  
- Rust crate (`stylus-hardware-primitives`) published to crates.io  
- Reference integration templates deployable on Sepolia  
- Professional third-party security audit  
- Mainnet deployment  
- Hardware expansion to ARM Cortex-M and RISC-V targets  
- Ecosystem integrations with Orbit-based DePIN networks  
- Governance exploration (subject to community approval)  

No Phase 2 budget is included in the current $25,000 USD request.

---

# Strategic Positioning

SHA aims to provide a reusable hardware identity primitive for Stylus-based contracts, enabling secure hardware-bound execution patterns on Arbitrum.

Phase 1 focuses exclusively on:

- Security hardening  
- Testnet validation  
- Developer tooling  

Mainnet deployment and professional audit are explicitly deferred to Phase 2.
