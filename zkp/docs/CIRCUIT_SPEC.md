# SHA × vlayer — Circuit Specification

**Version:** 0.1 (Phase 1 — Design)  
**Status:** Specification only. Implementation in Phase 2.  
**Circuit Language:** Noir (vlayer-compatible)  
**Proof System:** Groth16 / PLONK (per vlayer backend selection)

---

## 1. Purpose

This circuit proves execution correctness for SHA receipts. It answers:

> "The device produced `exec_hash` by correctly hashing its computation output `exec_data`."

This closes the security gap where `execution_hash` was opaque in SHA v1.

---

## 2. Circuit: `execution_proof` (Phase 2 Target)

### File: `zkp/circuits/execution_proof.nr`

```noir
// execution_proof.nr
// SHA × vlayer — Execution Correctness Circuit
//
// Proves: prover knows exec_data such that keccak256(exec_data) == exec_hash
// Public:  exec_hash (bytes32)
// Private: exec_data (variable length bytes)

fn main(
    // Public inputs — visible to on-chain verifier
    exec_hash: pub [u8; 32],

    // Private inputs — known only to prover (device)
    exec_data: [u8; 128],   // Phase 2: determine exact size from device output format
) {
    // Constraint: keccak256(exec_data) must equal the claimed exec_hash
    let computed_hash = std::hash::keccak256(exec_data, exec_data.len());
    assert(computed_hash == exec_hash);
}
```

**Note:** The exact size of `exec_data` will be determined in Phase 2 based on the device computation output format. Likely 64–256 bytes for sensor/oracle use cases.

---

## 3. Extended Circuit: `execution_proof_with_fw` (Phase 3 Target)

Adds firmware hash binding to the circuit, ensuring the circuit version matches the approved firmware:

```noir
// execution_proof_with_fw.nr
// Phase 3 extension — binds circuit to firmware version

fn main(
    // Public inputs
    exec_hash: pub [u8; 32],
    fw_hash:   pub [u8; 32],   // must match approved_firmware[fw_hash] on-chain

    // Private inputs
    exec_data: [u8; 128],
    fw_logic:  [u8; 64],       // committed firmware computation logic
) {
    // Constraint 1: execution hash correctness
    let computed_exec = std::hash::keccak256(exec_data, exec_data.len());
    assert(computed_exec == exec_hash);

    // Constraint 2: firmware logic commits to fw_hash
    // This binds the circuit to a specific firmware version
    let computed_fw = std::hash::keccak256(fw_logic, fw_logic.len());
    assert(computed_fw == fw_hash);
}
```

---

## 4. Recursive Circuit: `batch_execution_proof` (Phase 4 Target)

Aggregates N execution proofs into a single proof for DePIN-scale batch verification:

```noir
// batch_execution_proof.nr
// Phase 4 — Recursive aggregation
// Verifies N individual execution proofs recursively

fn main(
    // Public: array of exec_hashes (one per device receipt in batch)
    exec_hashes: pub [[u8; 32]; 50],   // N=50 batch target

    // Private: individual proofs for each receipt
    sub_proofs: [Proof; 50],
) {
    for i in 0..50 {
        // Verify each sub-proof recursively
        // This collapses N on-chain verifications to 1
        std::verify_proof(sub_proofs[i], [exec_hashes[i]]);
    }
}
```

---

## 5. Public Input Specification

The `exec_hash` public input is the bridge between SHA's receipt format and the ZK circuit:

```
On-chain: verify_receipt_with_zk(..., exec_hash, ..., zk_proof)
Circuit:  main(exec_hash=exec_hash, exec_data=<private>)

The verifier checks: does zk_proof prove knowledge of exec_data
                     such that keccak256(exec_data) == exec_hash?
```

exec_hash is already part of the SHA v1 receipt material. No changes to the receipt format are needed. The ZK layer verifies what was previously opaque.

---

## 6. Proof System Selection

| System | Proof Size | Verify Gas | Trust Setup | Recommendation |
|--------|-----------|-----------|-------------|----------------|
| Groth16 | ~200 bytes | ~150k–200k | Per-circuit trusted setup | Phase 2 (smaller proofs) |
| PLONK | ~400–800 bytes | ~200k–300k | Universal trusted setup | Alternative |
| UltraPlonk | ~400 bytes | ~180k | Universal | If vlayer supports |

**Phase 2 selection:** Follow vlayer's recommended proof system for Arbitrum Stylus compatibility. Benchmark both if the SDK supports it.

---

## 7. Phase 2 Implementation Checklist

- [ ] Finalize exec_data size and format from ESP32 computation output
- [ ] Implement `execution_proof.nr` circuit
- [ ] Generate proving/verification keys
- [ ] Deploy vlayer verifier contract to Sepolia
- [ ] Wire verifier address into SHA v2 `set_zk_verifier()`
- [ ] Implement `generate_zk_proof()` in `prove_and_submit.py`
- [ ] End-to-end test: ESP32 → prover → Sepolia → verify
- [ ] Publish gas benchmarks for ZK path vs v1 path
