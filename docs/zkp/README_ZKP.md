# Stylus Hardware Anchor (SHA) — ZKP Integration Branch

![Branch](https://img.shields.io/badge/branch-feat%2Fzkp--vlayer--integration-blue)
![Phase](https://img.shields.io/badge/phase-1%20Architecture-yellow)
![vlayer](https://img.shields.io/badge/vlayer-grant%20applied%202026--02--19-purple)
![Arbitrum](https://img.shields.io/badge/Arbitrum-Stylus-blue)

> **This branch extends SHA v1 with ZK execution correctness proofs via vlayer.**  
> SHA v1 is live on Arbitrum Sepolia. This branch is the ZK integration track.

---

## What SHA Does Today (v1 — Live on Sepolia)

SHA binds ESP32-S3 silicon identity to Arbitrum Stylus contracts:

```
eFuse MAC → HW_ID → on-chain allowlist
Firmware hash → approved_firmware mapping
Monotonic counter → replay protection
Keccak receipt → cryptographic integrity
```

**Deployed:** `0xD661a1aB8CEFaaCd78F4B968670C3bC438415615`  
**Gas:** 118,935 (single) / 12,564 per receipt (batch N=50)  
**Test vectors:** ≥10,000 validated

---

## What This Branch Adds (v2 — ZK Layer)

```
Layer 1: Silicon Identity     ← eFuse → Keccak → allowlist      ✅ LIVE
Layer 2: Firmware Governance  ← approved firmware hash gating   ✅ LIVE
Layer 3: Replay Protection    ← monotonic counter enforcement    ✅ LIVE
Layer 4: ZK Execution Proof   ← vlayer circuit + verifier       🔄 THIS BRANCH
```

**The security gap being closed:**

SHA v1 proves: *"This approved device submitted this receipt."*  
SHA v2 proves: *"This approved device **correctly computed** this and submitted it."*

---

## Current Phase: 1 — Architecture & Interfaces ✅

Phase 1 is complete. The architecture is fully designed, interfaces are defined and stable, and all documentation is published. This demonstrates architectural readiness before circuit implementation begins.

### Phase 1 Deliverables

| File | Description |
|------|-------------|
| `zkp/contracts/IZkVerifier.rs` | Rust trait for vlayer verifier interface |
| `zkp/contracts/sha_v2_interface.rs` | Full SHA v2 contract interface with ZK extension |
| `zkp/scripts/prove_and_submit.py` | End-to-end prover + submission script (scaffold) |
| `docs/zkp/ARCHITECTURE.md` | Complete ZK architecture specification |
| `docs/zkp/CIRCUIT_SPEC.md` | Noir circuit design for Phase 2 implementation |
| `docs/zkp/ZK_ROADMAP.md` | Detailed phase-by-phase roadmap with KPIs |
| `docs/zkp/INTEGRATION.md` | Developer integration guide |
| `PROGRESS.md` | Grant milestone tracker |
| `WINDSURF_COMMANDS.md` | Phase-gated execution instructions |

---

## Next Phase: 2 — Circuit + SHA v2 Contract + Testnet ⏳

Phase 2 implements the actual ZK layer:

1. **Noir circuit** (`zkp/circuits/execution_proof.nr`) — proves `keccak256(exec_data) == exec_hash`
2. **SHA v2 Stylus contract** — full `verify_receipt_with_zk()` implementation
3. **vlayer verifier** deployed on Arbitrum Sepolia
4. **End-to-end flow** — ESP32 → vlayer prover → Sepolia → verified
5. **Gas benchmarks** — ZK path vs v1 baseline

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                     │
│          (DePIN Sensors, Oracles, HW Custody)           │
└──────────────────────────┬──────────────────────────────┘
                           │
┌──────────────────────────┴──────────────────────────────┐
│                  Stylus Contract (SHA v2)                │
│                                                          │
│  Stage 1: Hardware Identity  ✅ (v1 preserved)           │
│  Stage 2: Firmware Check     ✅ (v1 preserved)           │
│  Stage 3: Counter + Digest   ✅ (v1 preserved)           │
│  Stage 4: ZK Proof Verify    🔄 vlayer verifier (v2 new) │
└──────────────────────────┬──────────────────────────────┘
                           │
             ┌─────────────┴─────────────┐
             │                           │
┌────────────┴──────────┐   ┌────────────┴──────────────┐
│   Hardware (ESP32-S3)  │   │    vlayer Prover           │
│  eFuse → HW_ID         │   │    (off-chain)             │
│  Keccak receipt        │──▶│  exec_data → zk_proof     │
└────────────────────────┘   └───────────────────────────┘
```

---

## Backward Compatibility

`verify_receipt()` (SHA v1) is **never modified**.  
`verify_receipt_with_zk()` is **additive only**.  
Existing integrations continue to work without any changes.

---

## Links

- [ZKP README](zkp/README.md)
- [Architecture](docs/zkp/ARCHITECTURE.md)
- [Circuit Spec](docs/zkp/CIRCUIT_SPEC.md)
- [Roadmap](docs/zkp/ZK_ROADMAP.md)
- [Progress](PROGRESS.md)
- [Main branch README](README.md)
- [SHA v1 on Sepolia](https://sepolia.arbiscan.io/address/0xD661a1aB8CEFaaCd78F4B968670C3bC438415615)
