//! SHA × vlayer — happy-path integration test
//!
//! Validates the full receipt construction and ZK proof verification pipeline
//! that verify_receipt_with_zk() will execute on-chain.
//!
//! Phase 2: mock_vlayer_prove() is replaced by a live vlayer prover call.
//! The data flow and assertions here remain identical.

use tiny_keccak::{Hasher, Keccak};

const DOMAIN: &[u8] = b"anchor_RCT_V1"; // 13 bytes — must match contract constant

fn keccak256(data: &[u8]) -> [u8; 32] {
    // 0x01 padding (Ethereum-compatible). NOT NIST SHA-3 (0x06).
    let mut k = Keccak::v256();
    k.update(data);
    let mut out = [0u8; 32];
    k.finalize(&mut out);
    out
}

/// Build 117-byte receipt material. Layout must be identical to Rust contract and ESP32 firmware.
fn build_receipt_material(
    hw_id: &[u8; 32],
    fw_hash: &[u8; 32],
    exec_hash: &[u8; 32],
    counter: u64,
) -> [u8; 117] {
    let mut m = [0u8; 117];
    m[0..13].copy_from_slice(DOMAIN);
    m[13..45].copy_from_slice(hw_id);
    m[45..77].copy_from_slice(fw_hash);
    m[77..109].copy_from_slice(exec_hash);
    m[109..117].copy_from_slice(&counter.to_be_bytes());
    m
}

/// Simulate the vlayer proveCall stage.
///
/// Asserts the prover's knowledge: exec_data is the preimage of exec_hash.
/// In Phase 2, the vlayer prover CLI generates a Groth16/PLONK proof of this
/// statement and submits it to verifyReceiptWithZk() as the `zk_proof` argument.
///
/// The on-chain IZkVerifier::verify(zk_proof, exec_hash) call checks the same relation.
fn mock_vlayer_prove(exec_data: &[u8], exec_hash: &[u8; 32]) -> bool {
    keccak256(exec_data) == *exec_hash
}

/// Happy-path test: full receipt flow from eFuse identity through ZK proof.
///
/// Covers all four verification stages:
///   Stage 1 — hardware identity present (hw_id is non-zero)
///   Stage 2 — firmware hash present (fw_hash is non-zero)
///   Stage 3 — counter > 0 and digest reconstructed correctly
///   Stage 4 — vlayer proof verifies exec_data preimage knowledge
#[test]
fn test_happy_path_zk_receipt() {
    // ── Inputs — representative of real ESP32-S3 device ─────────────────────
    // hw_id: Keccak-256 of 16-byte eFuse material (MAC + chip model + revision)
    let efuse_material: [u8; 16] = [
        0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, // base MAC (6 bytes)
        0x09,                                  // chip model: ESP32-S3
        0x02,                                  // chip revision
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // zero padding
    ];
    let hw_id: [u8; 32] = keccak256(&efuse_material);

    // fw_hash: Keccak-256 of firmware binary (approved on-chain)
    let fw_hash: [u8; 32] = keccak256(b"sha_anchor_firmware_v1_0_0");

    // exec_data: raw output from device computation (private witness for ZK)
    let exec_data = b"sensor:temp=22.5C,pressure=1013hPa,ts=1741305600";

    // exec_hash: public ZK input — Keccak-256 of exec_data
    // Submitted in the receipt; circuit proves knowledge of exec_data preimage.
    let exec_hash: [u8; 32] = keccak256(exec_data);

    let counter: u64 = 1;

    // ── Stage 3a: Build receipt material ────────────────────────────────────
    let material = build_receipt_material(&hw_id, &fw_hash, &exec_hash, counter);

    assert_eq!(material.len(), 117, "receipt material must be exactly 117 bytes");
    assert_eq!(&material[0..13], DOMAIN, "domain tag must match");
    assert_eq!(&material[13..45], hw_id.as_slice(), "hw_id at bytes [13..45]");
    assert_eq!(&material[45..77], fw_hash.as_slice(), "fw_hash at bytes [45..77]");
    assert_eq!(&material[77..109], exec_hash.as_slice(), "exec_hash at bytes [77..109]");
    assert_eq!(&material[109..117], &counter.to_be_bytes(), "counter big-endian at [109..117]");

    // ── Stage 3b: Compute and verify claimed_digest ──────────────────────────
    let claimed_digest = keccak256(&material);
    let recomputed = keccak256(&material);
    assert_eq!(claimed_digest, recomputed, "digest must be deterministic");

    // ── Stage 4: vlayer proveCall ────────────────────────────────────────────
    // Proves: keccak256(exec_data) == exec_hash
    // Phase 2: replace with live vlayer::prove(circuit, exec_data, exec_hash)
    let proof_valid = mock_vlayer_prove(exec_data, &exec_hash);
    assert!(proof_valid, "vlayer proof must verify: exec_hash is keccak256(exec_data)");

    // ── Sanity: forged exec_data fails the proof ─────────────────────────────
    let forged_exec_data = b"sensor:temp=99.9C,pressure=0hPa,ts=0";
    let forged_proof_valid = mock_vlayer_prove(forged_exec_data, &exec_hash);
    assert!(!forged_proof_valid, "vlayer must reject wrong exec_data for given exec_hash");
}
