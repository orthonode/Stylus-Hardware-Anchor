# SHA × vlayer — Circuit Specification

## Overview

This document specifies the Noir ZK circuits for SHA × vlayer integration. The circuits prove execution correctness of ESP32-S3 device computations.

## Phase 1 Circuit: `execution_proof.nr`

### Purpose

Proves that the prover knows `exec_data` such that `keccak256(exec_data) == exec_hash`. This makes the `exec_hash` field in SHA receipts non-malleable — devices cannot claim arbitrary hash values without knowing the corresponding execution data.

### Security Model

- **Public Input**: `exec_hash` (32 bytes) - visible to on-chain verifier
- **Private Input**: `exec_data` (128 bytes) - known only to prover/device
- **Constraint**: `keccak256(exec_data) == exec_hash`

### Circuit Interface

```noir
// execution_proof.nr
fn main(
    exec_hash: pub [u8; 32],
    exec_data: [u8; 128],
) {
    let computed_hash = std::hash::keccak256(exec_data, exec_data.len());
    assert(computed_hash == exec_hash);
}
```

### Design Rationale

1. **Simplicity**: Single hash constraint minimizes circuit size and proving time
2. **Compatibility**: Uses existing `exec_hash` field, no receipt format changes
3. **Flexibility**: `exec_data` size can be adjusted based on actual device output
4. **Security**: Keccak256 matches Ethereum/Stylus hash function used in SHA v1

### Integration Points

#### Receipt Material (unchanged from SHA v1)
```
[0:13]   "anchor_RCT_V1"     (domain tag)
[13:45]  hw_id              (32 bytes)
[45:77]  fw_hash            (32 bytes)
[77:109] exec_hash          (32 bytes) ← ZK public input
[109:117] counter           (8 bytes)
```

#### ZK Verification Pipeline
1. Device outputs `exec_data` (private)
2. Prover computes `exec_hash = keccak256(exec_data)`
3. Prover generates ZK proof using `execution_proof.nr`
4. SHA v2 verifies proof against stored `exec_hash` from receipt

### Phase 2 Circuit: `batch_execution_proof.nr`

#### Purpose (Future)

Aggregate N individual execution proofs into a single verifiable proof for gas optimization.

#### Interface (Planned)
```noir
fn main(
    exec_hashes: pub [[u8; 32]; 50],
    exec_data_batch: [[u8; 128]; 50],
) {
    for i in 0..50 {
        let computed = std::hash::keccak256(exec_data_batch[i], exec_data_batch[i].len());
        assert(computed == exec_hashes[i]);
    }
}
```

### Implementation Notes

#### vlayer Integration
- Circuit compiled with `nargo compile`
- Prover called via `vlayer prove --circuit execution_proof.nr --input ...`
- Proof format: Groth16 or PLONK (vlayer backend dependent)

#### Gas Considerations
- Single proof: ~200-300k gas (estimated)
- Batch proof (N=50): ~500-800k gas total (~10-16k per receipt)
- Target: <20k gas per receipt with aggregation

#### Security Assumptions
- Keccak256 preimage resistance: computationally infeasible to find `exec_data` for arbitrary `exec_hash`
- vlayer prover honesty: assumes correct vlayer CLI usage
- Circuit soundness: vlayer's trusted setup guarantees

## Testing Strategy

### Unit Tests
```bash
# Test vector generation
nargo test execution_proof

# Expected: proof verifies for valid exec_data
# Expected: proof fails for mismatched exec_hash
```

### Integration Tests
1. Generate test `exec_data` from ESP32 firmware
2. Compute `exec_hash` using same Keccak256 implementation
3. Generate proof with vlayer prover
4. Verify proof on-chain with SHA v2 contract

### Test Vectors
| Test | exec_data | exec_hash | Expected |
|------|-----------|-----------|----------|
| Valid | Random 128 bytes | keccak256(data) | ✅ Verify |
| Invalid | Random 128 bytes | Wrong hash | ❌ Reject |
| Edge | All zeros | 0xc5d246... | ✅ Verify |

## Future Extensions

### Phase 3: Recursive Composition
- Aggregate batch proofs into recursive proof
- Target: 1000+ receipts in single verification
- Gas optimization: O(log N) verification cost

### Phase 4: Custom Circuits
- Domain-specific execution proofs (e.g., sensor data validation)
- Computation-specific constraints beyond hash equality
- Integration with device-specific execution models

## Dependencies

- **Noir**: v1.0+ (circuit language)
- **vlayer**: CLI prover (backend)
- **Keccak256**: Standard Ethereum hash function
- **SHA v2**: On-chain verifier contract

## Deliverables

### Phase 1
- [x] `execution_proof.nr` circuit specification
- [x] Integration interface defined
- [x] Test vectors and validation strategy

### Phase 2
- [ ] Working circuit implementation
- [ ] vlayer prover integration
- [ ] End-to-end testnet verification

### Phase 3
- [ ] Batch aggregation circuit
- [ ] Gas benchmarking
- [ ] Production optimization

---

**Status**: Phase 1 specification complete  
**Next**: Phase 2 implementation and deployment
