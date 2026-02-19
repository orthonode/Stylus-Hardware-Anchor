// zkp/contracts/sha_v2_interface.rs
//
// SHA v2 Contract Interface — Full specification with ZK extension.
// This file defines the complete public API and storage layout for SHA v2.
//
// Phase 1: Interface + stubs defined here
// Phase 2: Full implementation in contracts/src/lib.rs (on this branch)
//
// BACKWARD COMPATIBILITY GUARANTEE:
//   verify_receipt()        — SHA v1 path, NEVER modified, always available
//   verify_receipt_with_zk() — SHA v2 path, additive only
//   zk_mode_enabled         — owner-controlled flag; false by default

#![allow(dead_code)]

use stylus_sdk::prelude::*;
use alloy_primitives::{Address, Bytes, FixedBytes, U256};

// ---------------------------------------------------------------------------
// STORAGE LAYOUT — SHA v2
// ---------------------------------------------------------------------------
// Extends SHA v1 storage. All v1 slots preserved at same offsets.
// New v2 slots appended only — no slot collisions.

#[storage]
pub struct HardwareAnchorV2 {
    // ── v1 storage (DO NOT REORDER) ──────────────────────────────────────
    /// Contract owner address (set on initialize)
    owner: StorageAddress,

    /// Whether initialize() has been called
    initialized: StorageBool,

    /// Mapping: hw_id → authorized (true = device is on allowlist)
    authorized_nodes: StorageMap<FixedBytes<32>, StorageBool>,

    /// Mapping: fw_hash → approved (true = firmware version is approved)
    approved_firmware: StorageMap<FixedBytes<32>, StorageBool>,

    /// Mapping: hw_id → last counter seen (monotonic replay guard)
    counters: StorageMap<FixedBytes<32>, StorageU64>,

    // ── v2 storage (new slots only) ──────────────────────────────────────
    /// Address of the vlayer ZK verifier contract
    /// Set by owner after Phase 2 deployment
    /// Zero address = verifier not yet configured
    zk_verifier: StorageAddress,

    /// When true: verify_receipt_with_zk() requires a valid ZK proof.
    /// When false: ZK proof is verified but failure is non-blocking (audit mode).
    /// Owner-controlled. Starts false. Set to true after circuit hardening.
    zk_mode_enabled: StorageBool,

    /// Count of ZK-verified receipts (for monitoring / grant evidence)
    zk_verify_count: StorageU256,
}

// ---------------------------------------------------------------------------
// ERROR TYPES — SHA v2
// ---------------------------------------------------------------------------

pub enum ShaError {
    // ── v1 errors (preserved) ──
    NotOwner,
    AlreadyInitialized,
    NotInitialized,
    NodeNotAuthorized,
    FirmwareNotApproved,
    CounterTooLow,
    DigestMismatch,

    // ── v2 errors (new) ──
    /// ZK verifier address not set — owner must call set_zk_verifier() first
    ZkVerifierNotSet,

    /// ZK proof failed verification
    ZkProofInvalid,

    /// ZK mode is enabled but no proof was provided (empty bytes)
    ZkProofMissing,
}

// ---------------------------------------------------------------------------
// PUBLIC INTERFACE — SHA v2
// ---------------------------------------------------------------------------

#[public]
impl HardwareAnchorV2 {

    // ────────────────────────────────────────────────────────────────────────
    // ADMIN FUNCTIONS (owner-only)
    // ────────────────────────────────────────────────────────────────────────

    /// Initialize contract. Callable once. Sets owner = msg.sender.
    pub fn initialize(&mut self) -> Result<(), ShaError> {
        // Phase 2 implementation
        todo!()
    }

    /// Add a device to the authorized allowlist.
    /// Only callable by owner.
    pub fn authorize_node(&mut self, hw_id: FixedBytes<32>) -> Result<(), ShaError> {
        todo!()
    }

    /// Approve a firmware version.
    /// Only callable by owner.
    pub fn approve_firmware(&mut self, fw_hash: FixedBytes<32>) -> Result<(), ShaError> {
        todo!()
    }

    /// Set the vlayer ZK verifier contract address.
    /// Only callable by owner. Must be called before ZK mode can be enabled.
    /// Phase 2 deliverable: address of deployed vlayer verifier on Sepolia.
    pub fn set_zk_verifier(&mut self, verifier: Address) -> Result<(), ShaError> {
        todo!()
    }

    /// Enable or disable ZK-required mode.
    /// When enabled: verify_receipt_with_zk() will revert on invalid proof.
    /// When disabled: ZK proof is checked but failure is logged, not reverted (audit mode).
    /// Owner-only. Irreversible transition to enabled recommended post-audit.
    pub fn set_zk_mode(&mut self, enabled: bool) -> Result<(), ShaError> {
        todo!()
    }

    // ────────────────────────────────────────────────────────────────────────
    // SHA v1 PATH — NEVER MODIFIED
    // ────────────────────────────────────────────────────────────────────────

    /// Original single-receipt verification. SHA v1 path.
    /// Preserved exactly as deployed at 0xD661a1aB8CEFaaCd78F4B968670C3bC438415615.
    ///
    /// Guarantees: hardware identity + firmware approval + replay protection + digest integrity.
    /// Does NOT verify execution correctness (no ZK).
    pub fn verify_receipt(
        &mut self,
        hw_id: FixedBytes<32>,
        fw_hash: FixedBytes<32>,
        exec_hash: FixedBytes<32>,
        counter: u64,
        claimed_digest: FixedBytes<32>,
    ) -> Result<(), ShaError> {
        // 1. Hardware identity check
        // require(self.authorized_nodes.get(hw_id) == true, ShaError::NodeNotAuthorized);

        // 2. Firmware approval check
        // require(self.approved_firmware.get(fw_hash) == true, ShaError::FirmwareNotApproved);

        // 3. Monotonic counter check
        // let last = self.counters.get(hw_id);
        // require(counter > last, ShaError::CounterTooLow);

        // 4. Reconstruct and compare digest
        // let material = build_receipt_material(hw_id, fw_hash, exec_hash, counter);
        // let reconstructed = keccak256(&material);
        // require(reconstructed == claimed_digest, ShaError::DigestMismatch);

        // 5. Update counter state
        // self.counters.insert(hw_id, counter);

        todo!("Phase 2: copy exact v1 logic here — do not alter semantics")
    }

    /// Batch receipt verification. SHA v1 path.
    /// Preserved exactly. Takes packed bytes blob, returns bitset of results.
    pub fn verify_receipts_batch_bitset_bytes(
        &mut self,
        packed_receipts: Bytes,
    ) -> Result<FixedBytes<32>, ShaError> {
        todo!("Phase 2: copy exact v1 batch logic here — do not alter semantics")
    }

    // ────────────────────────────────────────────────────────────────────────
    // SHA v2 PATH — ZK EXTENSION (additive)
    // ────────────────────────────────────────────────────────────────────────

    /// ZK-extended receipt verification. SHA v2 path.
    ///
    /// Runs full SHA v1 verification pipeline FIRST.
    /// Then verifies the vlayer ZK proof as Stage 4.
    ///
    /// # Arguments
    /// * `hw_id`          — 32-byte hardware identity (eFuse-derived)
    /// * `fw_hash`        — Firmware version hash
    /// * `exec_hash`      — Hash of computation output (public ZK input)
    /// * `counter`        — Monotonic receipt counter
    /// * `claimed_digest` — Keccak receipt digest (SHA v1 digest)
    /// * `zk_proof`       — Serialized vlayer proof bytes
    ///
    /// # Verification Pipeline
    /// Stage 1: require(authorized_nodes[hw_id] == true)
    /// Stage 2: require(approved_firmware[fw_hash] == true)
    /// Stage 3: require(counter > counters[hw_id])
    /// Stage 3: require(keccak(material) == claimed_digest)
    /// Stage 4: require(zk_verifier.verify(zk_proof, exec_hash) == true)
    ///          ↑ This is the new vlayer call
    ///
    /// # Backward Compatibility
    /// If zk_mode_enabled == false: Stage 4 runs but invalid proof only emits
    /// an event (non-blocking). This allows audit/monitoring before hard enforcement.
    pub fn verify_receipt_with_zk(
        &mut self,
        hw_id: FixedBytes<32>,
        fw_hash: FixedBytes<32>,
        exec_hash: FixedBytes<32>,
        counter: u64,
        claimed_digest: FixedBytes<32>,
        zk_proof: Bytes,
    ) -> Result<(), ShaError> {
        // ── Stage 1: Hardware identity ─────────────────────────────────────
        // require(
        //     self.authorized_nodes.get(hw_id),
        //     ShaError::NodeNotAuthorized
        // );

        // ── Stage 2: Firmware approval ─────────────────────────────────────
        // require(
        //     self.approved_firmware.get(fw_hash),
        //     ShaError::FirmwareNotApproved
        // );

        // ── Stage 3: Replay protection + digest ────────────────────────────
        // let last = self.counters.get(hw_id);
        // require(counter > last, ShaError::CounterTooLow);

        // let material = build_receipt_material(hw_id, fw_hash, exec_hash, counter);
        // let reconstructed = keccak256(&material);
        // require(reconstructed == claimed_digest, ShaError::DigestMismatch);

        // ── Stage 4: ZK execution proof ────────────────────────────────────
        // let verifier_addr = self.zk_verifier.get();
        // require(verifier_addr != Address::ZERO, ShaError::ZkVerifierNotSet);
        //
        // let verifier = IZkVerifier::at(verifier_addr);
        // let proof_valid = verifier.verify(zk_proof, exec_hash);
        //
        // if self.zk_mode_enabled.get() {
        //     require(proof_valid, ShaError::ZkProofInvalid);
        // } else {
        //     // Audit mode: emit event but don't revert
        //     if !proof_valid {
        //         // emit ZkProofAuditFailed { hw_id, exec_hash };
        //     }
        // }

        // ── Finalize ───────────────────────────────────────────────────────
        // self.counters.insert(hw_id, counter);
        // self.zk_verify_count += U256::from(1u64);

        todo!("Phase 2: implement full ZK-extended verification")
    }

    // ────────────────────────────────────────────────────────────────────────
    // VIEW FUNCTIONS
    // ────────────────────────────────────────────────────────────────────────

    pub fn get_owner(&self) -> Address {
        todo!()
    }

    pub fn is_node_authorized(&self, hw_id: FixedBytes<32>) -> bool {
        todo!()
    }

    pub fn is_firmware_approved(&self, fw_hash: FixedBytes<32>) -> bool {
        todo!()
    }

    pub fn get_counter(&self, hw_id: FixedBytes<32>) -> u64 {
        todo!()
    }

    /// Returns the configured vlayer verifier address.
    /// Zero address means not yet configured.
    pub fn get_zk_verifier(&self) -> Address {
        todo!()
    }

    /// Returns whether ZK enforcement mode is active.
    pub fn is_zk_mode_enabled(&self) -> bool {
        todo!()
    }

    /// Returns total count of ZK-verified receipts (for monitoring).
    pub fn get_zk_verify_count(&self) -> U256 {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// RECEIPT MATERIAL BUILDER
// ---------------------------------------------------------------------------
// Preserved from v1. Domain tag and layout must match ESP32 firmware exactly.
//
// Layout (117 bytes total):
//   [0..13]   domain tag: "anchor_RCT_V1" (13 bytes, ASCII)
//   [13..45]  hw_id       (32 bytes)
//   [45..77]  fw_hash     (32 bytes)
//   [77..109] exec_hash   (32 bytes)
//   [109..117] counter    (8 bytes, big-endian u64)
//
// keccak256(material) == claimed_digest  ← this is what verify_receipt checks

fn build_receipt_material(
    hw_id: FixedBytes<32>,
    fw_hash: FixedBytes<32>,
    exec_hash: FixedBytes<32>,
    counter: u64,
) -> [u8; 117] {
    let mut material = [0u8; 117];
    material[0..13].copy_from_slice(b"anchor_RCT_V1");
    material[13..45].copy_from_slice(hw_id.as_slice());
    material[45..77].copy_from_slice(fw_hash.as_slice());
    material[77..109].copy_from_slice(exec_hash.as_slice());
    material[109..117].copy_from_slice(&counter.to_be_bytes());
    material
}
