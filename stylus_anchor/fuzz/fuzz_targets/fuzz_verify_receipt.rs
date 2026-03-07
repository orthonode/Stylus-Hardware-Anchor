#![no_main]

use libfuzzer_sys::fuzz_target;
use tiny_keccak::{Hasher, Keccak};

/// Domain tag matching on-chain constant (13 bytes: "anchor_RCT_V1")
const DOMAIN: &[u8] = b"anchor_RCT_V1";

/// Keccak-256 with 0x01 padding (Ethereum-compatible, NOT NIST SHA-3 0x06).
fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut k = Keccak::v256();
    k.update(data);
    let mut out = [0u8; 32];
    k.finalize(&mut out);
    out
}

/// Fuzz target for receipt verification logic.
///
/// Input layout (149 bytes minimum):
///   [0..32]   hw_id        — hardware identity (Keccak-256 of eFuse material)
///   [32..64]  fw_hash      — approved firmware hash
///   [64..96]  exec_hash    — execution output hash
///   [96..104] counter      — monotonic counter (big-endian u64)
///   [104..136] claimed_digest — digest submitted with receipt
///
/// Mirrors the four on-chain verification stages:
///   1. Identity check     (simulated: hw_id is non-zero)
///   2. Firmware check     (simulated: fw_hash is non-zero)
///   3. Replay protection  (counter > 0)
///   4. Digest verification (reconstruct and compare Keccak-256)
fuzz_target!(|data: &[u8]| {
    // Minimum: 32+32+32+8+32 = 136 bytes
    if data.len() < 136 {
        return;
    }

    let hw_id = &data[0..32];
    let fw_hash = &data[32..64];
    let exec_hash = &data[64..96];
    let counter_bytes: [u8; 8] = data[96..104].try_into().unwrap();
    let claimed_digest = &data[104..136];

    let counter = u64::from_be_bytes(counter_bytes);

    // Stage 1: identity check (non-zero hw_id simulates on-chain authorized_nodes lookup)
    let _authorized = hw_id.iter().any(|&b| b != 0);

    // Stage 2: firmware check (non-zero fw_hash simulates approved_firmware lookup)
    let _firmware_approved = fw_hash.iter().any(|&b| b != 0);

    // Stage 3: replay protection — counter must be strictly greater than stored value
    // Simulate stored counter = 0 (initial state)
    let _replay_safe = counter > 0;

    // Stage 4: digest verification — reconstruct receipt material and compare
    // Receipt format: DOMAIN(13) | HW_ID(32) | FW_HASH(32) | EXEC_HASH(32) | COUNTER(8) = 117 bytes
    let mut material = Vec::with_capacity(117);
    material.extend_from_slice(DOMAIN);
    material.extend_from_slice(hw_id);
    material.extend_from_slice(fw_hash);
    material.extend_from_slice(exec_hash);
    material.extend_from_slice(&counter.to_be_bytes());

    let expected_digest = keccak256(&material);

    // Constant-time-style comparison (no early exit on mismatch, matching contract intent)
    let mut diff = 0u8;
    for (a, b) in expected_digest.iter().zip(claimed_digest.iter()) {
        diff |= a ^ b;
    }
    let _digest_valid = diff == 0;
});
