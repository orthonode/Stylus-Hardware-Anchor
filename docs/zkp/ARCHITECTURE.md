# SHA × vlayer — Architecture Document

## System Overview

SHA × vlayer integrates Zero-Knowledge Proof execution verification into the Stylus Hardware Anchor system. This adds cryptographic proof of computational correctness to the existing hardware identity and firmware governance guarantees.

## Security Architecture

### 4-Layer Verification Model

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                     │
│          (DePIN Sensors, Oracles, HW Custody)           │
└──────────────────────────┬──────────────────────────────┘
                           │
┌──────────────────────────┴──────────────────────────────┐
│                  Stylus Contract (SHA v2)                │
│                                                          │
│  Stage 1: Hardware Identity  ← verify eFuse allowlist   │
│  Stage 2: Firmware Check     ← verify fw_hash approved  │
│  Stage 3: Counter Enforce    ← monotonic replay guard   │
│  Stage 4: ZK Proof Verify    ← vlayer verifier  ◄─ NEW  │
└──────────────────────────┬──────────────────────────────┘
                           │
             ┌─────────────┴─────────────┐
             │                           │
┌────────────┴──────────┐   ┌────────────┴──────────────┐
│   Hardware (ESP32-S3)  │   │    vlayer Prover           │
│                        │   │    (off-chain)             │
│  eFuse → HW_ID         │   │                            │
│  Keccak receipt        │   │  execution_data →          │
│  Sends exec_data       │──▶│  zk_proof (Groth16/PLONK)  │
└────────────────────────┘   └───────────────────────────┘
```

### Threat Model Mitigations

| Threat | SHA v1 | SHA v2 + ZK | Mitigation |
|--------|---------|--------------|-----------|
| **Fake Device** | ✅ eFuse binding | ✅ eFuse binding | Hardware identity |
| **Malicious Firmware** | ✅ Firmware gating | ✅ Firmware gating | Approved hash check |
| **Replay Attack** | ✅ Monotonic counter | ✅ Monotonic counter | Counter enforcement |
| **Execution Tampering** | ❌ No guarantee | ✅ ZK proof | Keccak preimage resistance |
| **Hash Malleability** | ❌ Arbitrary exec_hash | ✅ Non-malleable | ZK knowledge proof |

## Contract Architecture

### Storage Layout (SHA v2)

```rust
#[storage]
pub struct HardwareAnchorV2 {
    // v1 storage (preserved)
    owner:              StorageAddress,
    initialized:        StorageBool,
    authorized_nodes:   StorageMap<FixedBytes<32>, StorageBool>,
    approved_firmware:  StorageMap<FixedBytes<32>, StorageBool>,
    counters:           StorageMap<FixedBytes<32>, StorageU64>,

    // v2 storage (new)
    zk_verifier:        StorageAddress,   // vlayer verifier address
    zk_mode_enabled:    StorageBool,      // false=audit, true=enforce
    zk_verify_count:    StorageU256,      // monitoring counter
}
```

### Verification Pipeline

#### SHA v1 Path (unchanged)
```rust
pub fn verify_receipt(hw_id, fw_hash, exec_hash, counter, claimed_digest) {
    // Stage 1: Hardware identity
    require(authorized_nodes[hw_id], NodeNotAuthorized);
    
    // Stage 2: Firmware approval
    require(approved_firmware[fw_hash], FirmwareNotApproved);
    
    // Stage 3: Replay protection + digest
    require(counter > counters[hw_id], CounterTooLow);
    require(keccak256(material) == claimed_digest, DigestMismatch);
    
    // Stage 5: Update counter
    counters[hw_id] = counter;
}
```

#### SHA v2 Path (ZK extended)
```rust
pub fn verify_receipt_with_zk(hw_id, fw_hash, exec_hash, counter, claimed_digest, zk_proof) {
    // Stages 1-3: Same as v1 (hardware, firmware, replay, digest)
    require(authorized_nodes[hw_id], NodeNotAuthorized);
    require(approved_firmware[fw_hash], FirmwareNotApproved);
    require(counter > counters[hw_id], CounterTooLow);
    require(keccak256(material) == claimed_digest, DigestMismatch);
    
    // Stage 4: ZK execution proof
    let verifier = IZkVerifier::at(zk_verifier.get());
    let proof_valid = verifier.verify(zk_proof, exec_hash);
    
    if zk_mode_enabled.get() {
        require(proof_valid, ZkProofInvalid);  // Enforcement mode
    } else {
        if !proof_valid { emit ZkProofAuditFailed(...); }  // Audit mode
    }
    
    // Finalize: Update counters
    counters[hw_id] = counter;
    zk_verify_count += U256::from(1u64);
}
```

## Circuit Architecture

### Execution Correctness Circuit

```noir
// execution_proof.nr
fn main(
    exec_hash: pub [u8; 32],    // Public: from receipt
    exec_data: [u8; 128],       // Private: from device
) {
    let computed_hash = std::hash::keccak256(exec_data, exec_data.len());
    assert(computed_hash == exec_hash);
}
```

### Security Properties

1. **Soundness**: Invalid proofs cannot be verified
2. **Zero-Knowledge**: `exec_data` remains private
3. **Completeness**: Valid `exec_data` always verifies
4. **Succinctness**: Proof size ~1KB, verification ~200ms

### Batch Aggregation (Phase 3)

```noir
// batch_execution_proof.nr
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

## Prover Architecture

### Off-chain Prover Flow

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   ESP32-S3      │───▶│  Prover Script   │───▶│  vlayer CLI     │
│                 │    │                  │    │                  │
│ exec_data.json  │    │ build_material() │    │ nargo compile    │
│ serial output  │    │ keccak256()      │    │ vlayer prove     │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                                        │
                                                        ▼
┌─────────────────────────────────────────────────────────┐
│                Stylus Contract (SHA v2)                │
│                                                        │
│ verify_receipt_with_zk(hw_id, fw_hash, exec_hash,     │
│                        counter, digest, zk_proof)      │
└─────────────────────────────────────────────────────────┘
```

### Prover Components

1. **Data Collection**: `exec_data` from ESP32-S3
2. **Hash Computation**: Keccak256 of execution output
3. **Proof Generation**: vlayer CLI with Noir circuit
4. **Submission**: On-chain `verify_receipt_with_zk()` call

### Gas Optimization Strategy

| Verification Type | Gas Cost | Receipts | Gas/Receipt |
|------------------|----------|-----------|-------------|
| SHA v1 (single)  | 118,935  | 1         | 118,935     |
| SHA v1 (batch)   | 628,200  | 50        | 12,564      |
| SHA v2 + ZK      | ~300,000 | 1         | ~300,000    |
| SHA v2 + ZK Batch | ~800,000 | 50        | ~16,000     |

## Integration Architecture

### Backward Compatibility

- **SHA v1 Path**: `verify_receipt()` never modified
- **Additive Only**: `verify_receipt_with_zk()` is new function
- **Feature Flag**: `zk_mode_enabled` controls enforcement
- **Storage Safe**: v1 slots preserved, v2 slots appended

### Migration Path

```
Phase 1: Architecture design (current)
├── Interfaces defined
├── Storage layout planned
└── Migration strategy documented

Phase 2: ZK verification (next)
├── Deploy vlayer verifier
├── Deploy SHA v2 contract
├── Enable audit mode (zk_mode_enabled = false)
└── Monitor ZK proof failures

Phase 3: Enforcement transition
├── Audit results review
├── Enable enforcement mode (zk_mode_enabled = true)
└── Decommission SHA v1 path (optional)
```

### Deployment Architecture

#### Contract Addresses
- **SHA v1**: `0xD661a1aB8CEFaaCd78F4B968670C3bC438415615` (Sepolia)
- **SHA v2**: TBD (Phase 2 deployment)
- **vlayer Verifier**: TBD (vlayer deployment)

#### Network Configuration
- **Testnet**: Arbitrum Sepolia (current)
- **Mainnet**: Arbitrum One (future)
- **RPC**: `https://sepolia-rollup.arbitrum.io/rpc`

## Security Considerations

### Cryptographic Assumptions

1. **Keccak256 Preimage Resistance**: 2^128 security level
2. **vlayer Trusted Setup**: Groth16 setup ceremony security
3. **Noir Soundness**: Circuit compiler correctness
4. **Stylus VM**: Correct contract execution

### Risk Mitigations

| Risk | Mitigation |
|------|------------|
| **Circuit Bug** | Audit mode before enforcement |
| **Prover Compromise** | Off-chain model, no on-chain secrets |
| **Verifier Failure** | Multiple verifier implementations |
| **Gas DoS** | Proof size limits, gas caps |

### Compliance & Audits

- **Phase 1**: Architecture review ✅
- **Phase 2**: Circuit audit + penetration testing
- **Phase 3**: Formal verification of batch aggregation
- **Phase 4**: Mainnet readiness assessment

## Performance Architecture

### Latency Breakdown

| Step | Time | Location |
|------|------|----------|
| ESP32 Execution | 10-100ms | Device |
| Data Transfer | 100-500ms | Serial/Network |
| Hash Computation | <1ms | Prover |
| Proof Generation | 1-5s | vlayer |
| On-chain Verify | 200-500ms | Stylus |

| Total | 1.5-6s | End-to-end |

### Throughput Targets

| Metric | SHA v1 | SHA v2 + ZK | Target |
|--------|---------|--------------|--------|
| Single Receipt | 118k gas | ~300k gas | <350k |
| Batch (N=50) | 12.5k/receipt | ~16k/receipt | <20k |
| Prover Rate | N/A | 10-20/min | 30+/min |

## Future Architecture

### Phase 4: Recursive ZK

```
┌─────────────────────────────────────────────────────────┐
│                Recursive Proof Tree                     │
│                                                        │
│    Layer 1: 50 execution proofs                       │
│    Layer 2: 10 aggregated proofs                      │
│    Layer 3: 1 final recursive proof                   │
│                                                        │
│    Gas: O(log N) instead of O(N)                      │
└─────────────────────────────────────────────────────────┘
```

### Scaling Architecture

- **Device Networks**: 1000+ ESP32-S3 devices
- **Proof Aggregation**: Hierarchical aggregation trees
- **Verification**: Single on-chain call for entire network
- **Gas Optimization**: Sub-linear verification cost

---

**Status**: Phase 1 architecture complete  
**Next**: Phase 2 circuit implementation and deployment
