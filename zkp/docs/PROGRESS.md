# SHA × vlayer — Progress Tracker

> This file tracks momentum for the vlayer grant application submitted 2026-02-19.
> Updated each time a phase milestone is reached.

---

## Grant Application

- **Applied:** 2026-02-19
- **Grant:** vlayer ZK Integration Grant
- **Applicant:** Orthonode Infrastructure Labs
- **Project:** Stylus Hardware Anchor (SHA) × vlayer ZKP Extension

---

## Baseline Evidence (SHA v1 — Pre-ZK)

| Artifact | Value |
|----------|-------|
| Sepolia Contract | `0xD661a1aB8CEFaaCd78F4B968670C3bC438415615` |
| Deploy TX | `0x1a9eaa02f816d86a71f9bf234425e83b5c090d1f3e4f3691851964b71747a489` |
| Activate TX | `0x353d26f4dea36a4410454b7b081cc41610f691dfea7ce29d5c9b1e9aa968f955` |
| WASM sha256 | `4c00997c2bb00e8b786f2ea9d4e3eb87600bf6995bf4e3dd4debf6c473a5bd26` |
| Gas (single) | 118,935 |
| Gas (batch N=50) | 12,564/receipt |
| Test vectors | ≥10,000 validated |
| Release | [v1.0.0](https://github.com/arhantbarmate/stylus-hardware-anchor/releases/tag/v1.0.0) |

---

## ZKP Integration Progress

### Phase 1 — Architecture & Interface Design
**Started:** 2026-02-19  
**Status:** ✅ COMPLETE

| Deliverable | Status | Link |
|-------------|--------|------|
| Branch created | ✅ | `feat/zkp-vlayer-integration` |
| Directory scaffold | ✅ | `zkp/` |
| `IZkVerifier` trait | ✅ | `zkp/contracts/IZkVerifier.rs` |
| SHA v2 interface | ✅ | `zkp/contracts/sha_v2_interface.rs` |
| `prove_and_submit.py` | ✅ | `zkp/scripts/prove_and_submit.py` |
| Architecture doc | ✅ | `docs/zkp/ARCHITECTURE.md` |
| Circuit spec | ✅ | `docs/zkp/CIRCUIT_SPEC.md` |
| ZK Roadmap | ✅ | `docs/zkp/ZK_ROADMAP.md` |
| Integration guide | ✅ | `docs/zkp/INTEGRATION.md` |

### Phase 2 — Circuit + SHA v2 + Testnet
**Status:** ⏳ PENDING

| Deliverable | Status |
|-------------|--------|
| `execution_proof.nr` Noir circuit | ⏳ |
| SHA v2 full Stylus contract | ⏳ |
| vlayer verifier deployed on Sepolia | ⏳ |
| End-to-end proof flow working | ⏳ |
| Gas benchmarks (ZK path) | ⏳ |
| ≥10 ZK-verified receipts on testnet | ⏳ |

### Phase 3 — Batch Aggregation
**Status:** ⏳ PENDING Phase 2

### Phase 4 — Recursive ZK
**Status:** 🔮 FUTURE SCOPE

---

## Key Architecture Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-02-19 | ZK as additive Stage 4, not replacement | Preserves v1 guarantees; no breaking changes |
| 2026-02-19 | Off-chain prover model (ESP32 too weak for ZK generation) | ESP32-S3 has 512KB SRAM; ZK proving needs hundreds of MB |
| 2026-02-19 | Groth16/PLONK selection deferred to Phase 2 | Follow vlayer SDK recommendation for Stylus compatibility |
| 2026-02-19 | `zk_mode_enabled` flag for migration safety | Allows audit mode before enforcement |
| 2026-02-19 | `exec_hash` as ZK public input | Already in SHA v1 receipt; no format changes needed |
