# SHA × vlayer — Integration Guide

**Audience:** Developers integrating SHA v2 with ZK verification  
**Prerequisites:** SHA v1 setup complete (see SETUP_GUIDE.md)

---

## Overview

SHA v2 adds a single new function to the existing contract:

```
verify_receipt()          → SHA v1 (no ZK, always available)
verify_receipt_with_zk()  → SHA v2 (ZK required, same hardware checks)
```

Your existing integration using `verify_receipt()` continues to work unchanged.
To upgrade to ZK-verified receipts, follow this guide.

---

## Phase 2 Integration Steps (when Phase 2 is complete)

### Step 1 — Install vlayer prover

```bash
# Install vlayer CLI (Phase 2: exact install command from vlayer docs)
curl -L https://install.vlayer.xyz | bash
vlayer --version
```

### Step 2 — Deploy SHA v2 to Sepolia

```bash
# On the feat/zkp-vlayer-integration branch
cd contracts
cargo stylus deploy \
  --wasm-file-path ./target/wasm32-unknown-unknown/release/sha_v2.wasm \
  --private-key-path <your-key-file>

# Note the new contract address
export SHA_V2_ADDRESS=0x...
```

### Step 3 — Set ZK verifier address

The vlayer verifier is deployed separately by vlayer. Get the Sepolia address from vlayer docs.

```bash
cast send $SHA_V2_ADDRESS \
  "setZkVerifier(address)" \
  $VLAYER_VERIFIER_ADDRESS \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY

# Confirm:
cast call $SHA_V2_ADDRESS "getZkVerifier()(address)" --rpc-url $RPC_URL
```

### Step 4 — Generate a ZK proof for your device

```bash
# Collect execution data from your ESP32 device
# (the raw computation output before hashing)
python zkp/scripts/prove_and_submit.py \
  --hw-id   0x<your-hw-id> \
  --fw-hash 0x<your-fw-hash> \
  --exec-data execution_output.json \
  --counter <next-counter>
```

This will:
1. Compute `exec_hash = keccak256(exec_data)`
2. Call vlayer prover → get `zk_proof`
3. Build receipt digest
4. Submit `verify_receipt_with_zk()` to Sepolia

### Step 5 — Enable ZK enforcement (optional)

After validating the ZK path works in audit mode:

```bash
# Enable enforcement — after this, invalid ZK proofs revert
cast send $SHA_V2_ADDRESS \
  "setZkMode(bool)" true \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY
```

---

## Calling verify_receipt_with_zk() Directly

ABI signature:

```
verifyReceiptWithZk(
    bytes32 hwId,
    bytes32 fwHash,
    bytes32 execHash,
    uint64  counter,
    bytes32 claimedDigest,
    bytes   zkProof
)
```

Cast example:

```bash
cast send $SHA_V2_ADDRESS \
  "verifyReceiptWithZk(bytes32,bytes32,bytes32,uint64,bytes32,bytes)" \
  $HW_ID $FW_HASH $EXEC_HASH $COUNTER $DIGEST $ZK_PROOF \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY
```

---

## Checking ZK Proof Count (monitoring)

```bash
cast call $SHA_V2_ADDRESS "getZkVerifyCount()(uint256)" --rpc-url $RPC_URL
```

---

## Error Reference

| Error | Cause | Fix |
|-------|-------|-----|
| `ZkVerifierNotSet` | `set_zk_verifier()` not called | Owner calls `setZkVerifier(address)` |
| `ZkProofInvalid` | Proof doesn't verify | Regenerate proof; check exec_data matches exec_hash |
| `NodeNotAuthorized` | hw_id not on allowlist | Owner calls `authorizeNode(bytes32)` |
| `FirmwareNotApproved` | fw_hash not approved | Owner calls `approveFirmware(bytes32)` |
| `CounterTooLow` | Counter not monotonic | Increment counter; check device state |
| `DigestMismatch` | Receipt material mismatch | Check domain tag is `anchor_RCT_V1` (13 bytes), material is 117 bytes |
