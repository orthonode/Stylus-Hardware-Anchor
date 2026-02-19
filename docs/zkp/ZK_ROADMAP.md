# SHA × vlayer — ZK Integration Roadmap

**vlayer Grant Applied:** 2026-02-19  
**SHA Baseline:** v1.0.0 deployed on Arbitrum Sepolia  
**Branch:** `feat/zkp-vlayer-integration`

---

## Strategic Context

SHA v1 provides silicon-rooted hardware identity — the strongest possible guarantee of *who submitted a receipt*. The remaining gap is *what computation they ran*. vlayer's ZK infrastructure closes this gap by making execution correctness cryptographically verifiable on-chain.

This is not a pivot. It is a natural extension: SHA provides the hardware trust anchor, vlayer provides the execution validity layer. Neither replaces the other.

---

## Phase 1 — Architecture & Interface Design
**Status: ✅ IN PROGRESS**  
**Timeline: Weeks 1–2**  
**Scope: This branch**

### Deliverables
- [x] `feat/zkp-vlayer-integration` branch created from main
- [x] Full ZKP directory structure scaffolded
- [x] `IZkVerifier` Rust trait defined
- [x] `SHA v2` contract interface with `verify_receipt_with_zk()` specified
- [x] `prove_and_submit.py` scaffold with full CLI interface
- [x] Architecture documentation (`docs/zkp/ARCHITECTURE.md`)
- [x] Circuit specification (`docs/zkp/CIRCUIT_SPEC.md`)
- [x] Integration guide (`docs/zkp/INTEGRATION.md`)
- [x] Progress tracking (`PROGRESS.md`)

### What This Proves to Reviewers
The architecture is designed. Interfaces are stable. The team understands ZK integration deeply enough to specify circuits, define storage layouts, and design backward-compatible extension paths before writing a line of circuit code.

---

## Phase 2 — ZK Circuit + SHA v2 Contract + Testnet Deployment
**Status: ⏳ PENDING Phase 1 review**  
**Timeline: Weeks 3–8**  
**Scope: Working ZK verification on Arbitrum Sepolia**

### Deliverables

#### 2a. Noir Circuit (`zkp/circuits/execution_proof.nr`)
```noir
fn main(
    exec_hash: pub [u8; 32],   // public: matches exec_hash in SHA receipt
    exec_data: [u8; 128],      // private: device computation output
) {
    let computed = std::hash::keccak256(exec_data, exec_data.len());
    assert(computed == exec_hash);
}
```

#### 2b. SHA v2 Full Stylus Contract (`contracts/src/lib.rs` on this branch)
Complete implementation of `sha_v2_interface.rs`:
- All v1 functions preserved byte-for-byte
- `verify_receipt_with_zk()` fully implemented
- `set_zk_verifier()` / `set_zk_mode()` admin functions
- Events: `ZkProofVerified`, `ZkProofAuditFailed`

#### 2c. vlayer Verifier Deployment
- Deploy vlayer-generated verifier contract to Arbitrum Sepolia
- Call `set_zk_verifier(verifier_address)` on SHA v2
- Verify with cast: `cast call $SHA_V2 "getZkVerifier()(address)"`

#### 2d. End-to-End Prover Script (`zkp/prover/generate_proof.py`)
```python
def generate_zk_proof(exec_data: bytes, exec_hash: bytes) -> bytes:
    result = subprocess.run([
        "vlayer", "prove",
        "--circuit", "zkp/circuits/execution_proof.nr",
        "--input", json.dumps({
            "exec_hash": exec_hash.hex(),
            "exec_data": list(exec_data),
        })
    ], capture_output=True, check=True)
    return bytes.fromhex(json.loads(result.stdout)["proof"])
```

#### 2e. Gas Benchmarks (`docs/zkp/BENCHMARKS_ZK.md`)
| Path | Gas | Notes |
|------|-----|-------|
| verifyReceipt (v1) | 118,935 | baseline |
| verifyReceiptWithZk | TBD | v1 + ZK verifier call |
| ZK verifier alone | TBD | isolated benchmark |

#### 2f. Integration Tests (`zkp/tests/test_zk_verify.rs`)
- Happy path: valid proof → verify succeeds
- Invalid proof → revert with `ZkProofInvalid`
- Missing verifier → revert with `ZkVerifierNotSet`
- Audit mode: invalid proof → event emitted, no revert

### Phase 2 KPIs
- [ ] End-to-end flow working on Sepolia (ESP32 → prover → contract)
- [ ] Gas benchmarks published
- [ ] ZK verifier address publicly set on SHA v2
- [ ] At least 10 ZK-verified receipts submitted on testnet

---

## Phase 3 — Batch Proof Aggregation
**Status: ⏳ PENDING Phase 2**  
**Timeline: Weeks 9–14**  
**Scope: DePIN-scale throughput**

### Technical Approach

Instead of N on-chain verifier calls for N receipts, aggregate proofs off-chain:

```
Devices:    proof_1, proof_2, ..., proof_50
Aggregator: aggregate(proof_1..50) → proof_aggregate (single proof)
Contract:   verify(proof_aggregate, [exec_hash_1..exec_hash_50])
```

One on-chain call verifies 50 receipts. Gas per receipt drops proportionally.

#### Phase 3 Contract Addition

```rust
pub fn verify_receipts_batch_with_zk(
    &mut self,
    packed_receipts: Bytes,        // same format as v1 batch
    aggregate_proof: Bytes,        // single aggregated ZK proof
    exec_hashes: Vec<FixedBytes<32>>, // public inputs for each receipt
) -> Result<FixedBytes<32>, ShaError> {
    // 1. Run v1 batch SHA checks on all receipts
    // 2. Verify aggregate ZK proof against all exec_hashes
    todo!("Phase 3")
}
```

#### Phase 3 KPIs
- Batch N=50 ZK verification gas per receipt < 20k
- Aggregation latency documented
- Benchmark comparison: v1 batch vs v2+ZK batch

---

## Phase 4 — Recursive ZK for DePIN Networks
**Status: 🔮 FUTURE**  
**Timeline: TBD (separate grant scope)**

### Vision

For networks with 1000+ devices submitting receipts, recursive ZK aggregation reduces verification cost to a constant:

```
1000 device receipts → 1 recursive proof → 1 on-chain verify
```

Architecture:
```
Devices → individual proofs
        → L1 aggregator (proof_1..100)
        → L2 recursive aggregator (aggregated_1..10 → 1 recursive proof)
        → Stylus contract: 1 verify call for 1000 receipts
```

This makes hardware-backed verified computation viable at the scale required for real DePIN infrastructure.

---

## What Changes in Main Branch

When Phase 2 is complete and audited, the following changes merge to main:

1. `contracts/src/lib.rs` — SHA v2 (additive only, v1 preserved)
2. `zkp/circuits/` — published circuit files
3. `zkp/scripts/prove_and_submit.py` — complete implementation
4. `docs/zkp/` — full documentation suite
5. `BENCHMARKS.md` — updated with ZK benchmark rows
6. `README.md` — updated with ZK layer description

**Nothing in the v1 contract interface changes. No storage slots move. Existing deployments remain valid.**

---

## Grant Alignment

| vlayer Grant Expectation | SHA Deliverable |
|--------------------------|-----------------|
| Circuit implementation | `execution_proof.nr` (Phase 2) |
| On-chain verifier integration | SHA v2 `verify_receipt_with_zk()` (Phase 2) |
| Real use case | Hardware-bound DePIN receipts |
| Testnet evidence | Arbiscan TX hashes (Phase 2) |
| Gas benchmarks | ZK vs v1 comparison table (Phase 2) |
| Developer tooling | `prove_and_submit.py` + Python SDK (Phase 2) |
