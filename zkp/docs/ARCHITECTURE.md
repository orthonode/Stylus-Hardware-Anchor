# SHA × vlayer — ZK Architecture Specification

**Version:** 1.0 (Phase 1)  
**Date:** 2026-02-19  
**Branch:** `feat/zkp-vlayer-integration`

---

## 1. System Overview

SHA (Stylus Hardware Anchor) provides silicon-rooted hardware identity on Arbitrum Stylus. This document specifies the architecture for extending SHA with ZK execution proofs via vlayer.

The core principle:

```
SHA  = hardware authenticity   ("this device exists and is approved")
vlayer = execution correctness  ("this device computed this correctly")
```

Combined, they form **hardware-backed verified computation** — a primitive required for trustless DePIN networks, hardware oracles, and silicon-bonded compute markets.

---

## 2. Current System (SHA v1)

### Receipt Structure

On-device computation on ESP32-S3:

```
receipt_material (117 bytes):
  [0:13]    "anchor_RCT_V1"   ← domain tag (13 bytes ASCII)
  [13:45]   hardware_id       ← eFuse-derived, 32 bytes
  [45:77]   firmware_hash     ← approved firmware version, 32 bytes
  [77:109]  execution_hash    ← opaque hash of device computation
  [109:117] counter           ← monotonic u64, big-endian

receipt_digest = keccak256(receipt_material)
```

### On-Chain Verification (v1)

```
verify_receipt(hw_id, fw_hash, exec_hash, counter, claimed_digest):
  1. authorized_nodes[hw_id] == true       ← hardware allowlist
  2. approved_firmware[fw_hash] == true    ← firmware gating
  3. counter > counters[hw_id]             ← replay protection
  4. keccak256(material) == claimed_digest ← integrity
  5. counters[hw_id] = counter             ← state update
```

### What v1 Proves

✅ The specific silicon device is on the allowlist  
✅ The device runs an approved firmware version  
✅ The receipt has not been replayed  
✅ The receipt digest is cryptographically intact  

### What v1 Does NOT Prove

❌ That `execution_hash` was computed correctly  
❌ That the device didn't fabricate the execution output  
❌ That the computation logic matches any specification  

`execution_hash` is opaque to the contract. A compromised device could submit any hash it wants — as long as the hardware identity and firmware checks pass, the receipt verifies.

---

## 3. ZK Extension (SHA v2)

### Design Principle

ZK augments SHA v1. It does not replace it. Hardware checks run first. ZK runs last.

```
Stage 1: Hardware identity  ← same as v1
Stage 2: Firmware approval  ← same as v1
Stage 3: Replay protection  ← same as v1
Stage 4: ZK execution proof ← NEW: vlayer verifier
```

Stage 4 answers: *"Did the device correctly compute the execution_hash from its inputs, according to the circuit spec?"*

### ZK Proof Flow

```
                    ┌─────────────────────────────┐
                    │       ESP32-S3 Device        │
                    │                              │
                    │  1. Execute computation      │
                    │  2. Hash output → exec_hash  │
                    │  3. Build receipt material   │
                    │  4. Send exec_data to prover │
                    └──────────────┬───────────────┘
                                   │ exec_data (off-chain)
                    ┌──────────────▼───────────────┐
                    │       vlayer Prover           │
                    │       (off-chain server)      │
                    │                              │
                    │  Circuit inputs:             │
                    │    private: exec_data        │
                    │    public:  exec_hash        │
                    │             fw_hash          │
                    │                              │
                    │  Output: zk_proof (bytes)   │
                    └──────────────┬───────────────┘
                                   │ (hw_id, fw_hash, exec_hash,
                                   │  counter, digest, zk_proof)
                    ┌──────────────▼───────────────┐
                    │     SHA v2 Stylus Contract    │
                    │                              │
                    │  verify_receipt_with_zk()   │
                    │    Stage 1: hw allowlist     │
                    │    Stage 2: fw approval      │
                    │    Stage 3: counter + digest │
                    │    Stage 4: zk_verifier      │
                    │             .verify(proof,   │
                    │              exec_hash)      │
                    └──────────────────────────────┘
```

### Why ESP32 Doesn't Generate the Proof

ESP32-S3 is an embedded MCU (Xtensa LX7, ~240MHz, 512KB SRAM). ZK proof generation requires hundreds of MB RAM and minutes of compute. The device produces the *witness* (execution_data) and submits it to an off-chain prover. The proof is then submitted on-chain alongside the receipt. The trust model is unchanged: the circuit constrains what the prover can prove.

---

## 4. Circuit Design (Phase 2 Specification)

### Public Inputs

```
exec_hash  : bytes32   ← claimed hash of device computation
fw_hash    : bytes32   ← firmware version (must match approved_firmware mapping)
```

### Private Inputs (Witness)

```
exec_data  : bytes     ← raw computation output from device
             (sensor readings, oracle values, computation state, etc.)
```

### Circuit Constraint

```
assert keccak256(exec_data) == exec_hash
assert sha256(circuit_logic_committed_to_fw_hash) == fw_hash  // optional: bind to circuit version
```

The circuit proves: *"I know exec_data such that keccak256(exec_data) == exec_hash"*

This makes exec_hash non-malleable. A device cannot claim an arbitrary exec_hash — it must know the preimage.

### Extension: Computation Logic Verification (Phase 3+)

A more advanced circuit (Phase 3) proves:
```
assert compute(exec_data, fw_hash) == exec_hash
```
where `compute` is the device's actual logic encoded in the circuit. This proves not just hash preimage knowledge but correct execution of specific logic. Phase 3 work.

---

## 5. Storage Changes (SHA v1 → v2)

v1 storage slots are **never reordered**. v2 appends new slots only.

```rust
// v2-only additions:
zk_verifier:      StorageAddress   // vlayer verifier contract address
zk_mode_enabled:  StorageBool      // false = audit mode, true = enforcement
zk_verify_count:  StorageU256      // monitoring counter
```

---

## 6. Backward Compatibility

```
verify_receipt()           → SHA v1 path, never changes
verify_receipt_with_zk()   → SHA v2 path, additive only
```

Existing integrations using `verify_receipt()` continue to work without modification. The ZK path is opt-in per-call.

`zk_mode_enabled` gives the owner a migration path:
- **Audit mode** (false): ZK proof checked, failure emits event only — allows monitoring ZK before enforcement
- **Enforcement mode** (true): ZK proof invalid = transaction reverts — full security guarantee

---

## 7. Gas Cost Model

### SHA v1 (baseline)
| Operation | Gas |
|-----------|-----|
| Single verifyReceipt | 118,935 |
| Batch N=50 | 12,564/receipt |

### SHA v2 (projected — Phase 2 will benchmark)
| Operation | Estimated Gas | Notes |
|-----------|--------------|-------|
| verifyReceiptWithZk | ~150k–200k | v1 cost + ZK verifier call |
| ZK verifier call alone | ~30k–80k | depends on vlayer proof system |
| Batch ZK (Phase 3) | TBD | aggregation target |

Actual benchmarks will be published after Phase 2 Sepolia deployment.

---

## 8. Trust Model

| Component | Trusted by | Trust basis |
|-----------|-----------|-------------|
| ESP32 eFuse | Contract | Manufacturer-burned, read-only silicon |
| Firmware hash | Owner | Owner explicitly approves each fw version |
| ZK circuit | vlayer + public | Open circuit, verifiable by anyone |
| vlayer verifier | Contract | Deployed at known address, callable on-chain |
| Counter enforcement | Contract | On-chain state, monotonic |

The ZK circuit is the new trust anchor for execution correctness. Its security comes from the cryptographic hardness of the proof system, not from trusting the device.
