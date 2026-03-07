# SHA Threat Model

Stylus Hardware Anchor (SHA) — Arbitrum Sepolia
Contract: `0xD661a1aB8CEFaaCd78F4B968670C3bC438415615`
Scope: Phase 1 (testnet only). No mainnet. No professional audit yet.

---

## Assets

| Asset | Description | Location |
|-------|-------------|----------|
| **HW_ID** | 32-byte hardware identity derived from eFuse-burned MAC + chip info via Keccak-256 | ESP32-S3 silicon (read-only after fabrication) |
| **FW_HASH** | Keccak-256 of approved firmware binary | On-chain `approved_firmware` mapping |
| **EXEC_HASH** | Keccak-256 of computation output submitted per-receipt | Inside 117-byte receipt, verified on-chain |
| **Monotonic counters** | Per-device `uint64` counters preventing receipt reuse | On-chain `counters` mapping |
| **authorized_nodes** | Allowlist of valid HW_IDs | On-chain storage, owner-only write |
| **Owner private key** | Controls `authorizeNode`, `approveFirmware`, `revokeNode` | Off-chain, `.env` only |
| **Receipt (117 bytes)** | `DOMAIN(13) \| HW_ID(32) \| FW_HASH(32) \| EXEC_HASH(32) \| COUNTER(8)` | In-flight (ESP32 → middleware → chain) |
| **Keccak-256 digest** | Final 32-byte digest of receipt material, verified on-chain | Submitted with each verification call |

---

## Threats

### T1 — Identity Spoofing (Fake Device)
**Description:** Attacker attempts to forge a valid HW_ID without owning the physical ESP32-S3.
**Attack vector:** Software simulation of eFuse values; cloning MAC address in firmware.
**Impact:** Unauthorized hardware passes identity check (Stage 1).

### T2 — Unauthorized Firmware Execution
**Description:** Device runs non-approved firmware and submits receipts with that firmware's hash.
**Attack vector:** Flashing custom firmware; bypassing firmware hash in receipt.
**Impact:** Firmware governance check (Stage 2) passes for unapproved code.

### T3 — Replay Attack
**Description:** Attacker reuses a previously valid receipt to re-trigger on-chain state changes.
**Attack vector:** Capture and resubmit a valid `(hw_id, fw_hash, exec_hash, counter, digest)` tuple.
**Impact:** Counter check (Stage 3) passes; duplicate verification counted as valid.

### T4 — Execution Tampering (v1)
**Description:** Device claims an arbitrary `exec_hash` not derived from actual computation.
**Attack vector:** Craft `exec_hash` to pass digest reconstruction without performing the declared computation.
**Impact:** On-chain receipt accepted for work that was never executed or was executed incorrectly.
**Note:** SHA v1 does not verify the preimage of `exec_hash`. This is mitigated in SHA v2 via ZK proof (Stage 4).

### T5 — Digest Malleability (Padding Confusion)
**Description:** Cross-platform implementation uses NIST SHA-3 (0x06 padding) instead of Keccak-256 (0x01 padding).
**Attack vector:** ESP32 standard SHA-3 library; incorrect Python implementation.
**Impact:** Digests computed off-chain never match on-chain reconstruction; or worse, a crafted input passes only on one platform.

### T6 — Owner Key Compromise
**Description:** Private key used for `authorizeNode` and `approveFirmware` is leaked.
**Attack vector:** Key stored in tracked file, exposed via git history, or leaked from `.env`.
**Impact:** Attacker can authorize arbitrary HW_IDs and approve malicious firmware.

### T7 — Monotonic Counter Overflow
**Description:** Counter reaches `uint64` maximum (`2^64 - 1`) and wraps or reverts.
**Attack vector:** High-throughput device exhausting counter space.
**Impact:** Denial of service for that device; subsequent receipts permanently fail Stage 3.

### T8 — Gas DoS on Batch Verification
**Description:** Submitting maximum-size batch to push gas cost beyond block limit.
**Attack vector:** Craft batch inputs that maximize WASM execution cycles per byte.
**Impact:** Batch call reverts; denial of service for legitimate batch submitters.

### T9 — ZK Circuit Bug (SHA v2, future)
**Description:** Soundness failure in the Noir execution proof circuit.
**Attack vector:** Malformed proof passes `verify()` on the vlayer verifier.
**Impact:** Invalid execution accepted as proven-correct when `zk_mode_enabled = true`.

### T10 — Prover Compromise (SHA v2, future)
**Description:** Off-chain prover generates fraudulent proofs for fabricated `exec_data`.
**Attack vector:** Compromised prover host; malicious prover binary.
**Impact:** Fraudulent ZK proofs submitted on behalf of real devices.

---

## Mitigations

| Threat | Mitigation | Status |
|--------|------------|--------|
| **T1 — Identity Spoofing** | eFuse values are burned at fabrication and cannot be changed or emulated in software. A VM has no eFuse. HW_ID = Keccak-256(eFuse material) — not guessable. | ✅ Active (hardware guarantee) |
| **T2 — Unauthorized Firmware** | `approveFirmware(fw_hash)` allowlist enforced at Stage 2. Only owner can approve. Revocation via `revokeFirmware`. | ✅ Active (on-chain) |
| **T3 — Replay Attack** | Monotonic counter stored per HW_ID on-chain. Stage 3 requires `counter > counters[hw_id]`. Counter updated atomically on success. | ✅ Active (on-chain) |
| **T4 — Execution Tampering** | SHA v1: not mitigated — `exec_hash` is accepted as-is. SHA v2: ZK proof of knowledge of `exec_data` preimage required at Stage 4. | ⚠️ v1 unmitigated; v2 planned |
| **T5 — Padding Confusion** | Keccak-256 with 0x01 padding enforced in contract, ESP32 firmware (custom implementation), and Python middleware. 10,000 cross-platform test vectors validate parity. | ✅ Active (test vectors + custom firmware impl) |
| **T6 — Owner Key Compromise** | Key stored in `.env` only. `.env` is gitignored. `.env.example` contains only placeholders. Pre-push hook blocks tracked `.env`. | ✅ Active (4-layer git protection) |
| **T7 — Counter Overflow** | `uint64` supports ~1.8×10¹⁹ receipts per device. Overflow not practically reachable in testnet scope. Documented as known edge case for Phase 2. | ⚠️ Known limit; acceptable for Phase 1 |
| **T8 — Gas DoS** | Input size enforced via receipt length check (117 bytes per receipt). Batch function validates packed blob length before processing. | ✅ Active (input bounds) |
| **T9 — ZK Circuit Bug** | `zk_mode_enabled = false` (audit mode) by default. Circuit failures emit `ZkProofAuditFailed` event without reverting. Enforcement only after manual owner review. | ✅ Designed (Phase 2 deployment) |
| **T10 — Prover Compromise** | Off-chain prover model: no on-chain secrets accessible to prover. Prover only generates proofs; on-chain verifier independently validates. Multiple verifier implementations planned. | ✅ Designed (Phase 2 deployment) |

---

## Trust Boundary Summary

```
[ESP32-S3 eFuse] ── immutable hardware identity ──▶ [HW_ID computation]
                                                            │
[Owner key] ── authorizeNode() ──▶ [on-chain allowlist] ◀──┘
[Owner key] ── approveFirmware() ──▶ [firmware allowlist]
                                                            │
[ESP32-S3 firmware] ── generates receipt ──▶ [117-byte receipt]
                                                            │
[Python middleware] ── submits to chain ──▶ [4-stage verification]
    Stage 1: authorized_nodes[hw_id] == true
    Stage 2: approved_firmware[fw_hash] == true
    Stage 3: counter > counters[hw_id]
    Stage 4: keccak256(DOMAIN|hw_id|fw_hash|exec_hash|counter) == claimed_digest
                                                            │
                                              [counter updated; receipt accepted]
```

**Trust is rooted in silicon** (eFuse) and enforced on-chain. The middleware is untrusted transport.

---

## Known Limitations (Phase 1)

- No professional third-party audit — deferred to Phase 2.
- `verifyReceipt` (single-call) has a counter synchronization issue with the batch path. Batch is the primary interface.
- ZK execution proof (T4 mitigation) requires Phase 2 deployment.
- Mainnet not in scope for Phase 1.

---

*Sources: `docs/MILESTONE_1.md`, `docs/zkp/ARCHITECTURE.md`, `CLAUDE.md` security notes.*
*ORTHONODE SYSTEMS™ | Stylus Hardware Anchor | Arbitrum Sepolia*
