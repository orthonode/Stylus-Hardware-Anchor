// zkp/contracts/IZkVerifier.rs
//
// Interface trait for the vlayer ZK verifier contract.
// SHA v2 calls this to verify execution correctness proofs.
//
// Phase: 1 (interface only — implementation in Phase 2)
// Deployed verifier address stored in: sha_v2.zk_verifier (StorageAddress)

use stylus_sdk::prelude::*;
use alloy_primitives::{Bytes, FixedBytes};

/// Trait representing the on-chain vlayer verifier contract interface.
///
/// The concrete implementation is deployed by vlayer and its address
/// is stored in SHA v2 storage as `zk_verifier: StorageAddress`.
///
/// SHA v2 calls `verify()` as Stage 4 of receipt verification.
/// If this returns false, the entire verify_receipt_with_zk() call reverts.
#[interface]
pub trait IZkVerifier {
    /// Verify a ZK proof against a public input.
    ///
    /// # Arguments
    /// * `proof`        — Serialized proof bytes (Groth16 or PLONK depending on vlayer backend)
    /// * `public_input` — The execution_hash that the circuit publicly commits to
    ///
    /// # Returns
    /// * `true`  — Proof is valid: the prover correctly computed execution_hash from exec_data
    /// * `false` — Proof is invalid: revert upstream
    ///
    /// # Security Note
    /// This function must be called AFTER SHA hardware/firmware/counter checks pass.
    /// ZK validity alone does not authorize a receipt — hardware identity must be verified first.
    fn verify(proof: Bytes, public_input: FixedBytes<32>) -> bool;
}

// ---------------------------------------------------------------------------
// PHASE 2 IMPLEMENTATION NOTE
// ---------------------------------------------------------------------------
// In Phase 2, the vlayer-deployed verifier contract will implement this trait.
// SHA v2 will store its address and call it via:
//
//   let verifier = IZkVerifier::at(self.zk_verifier.get());
//   let valid = verifier.verify(zk_proof, exec_hash);
//   require(valid, Error::ZkProofInvalid);
//
// The verifier contract itself is NOT written by this project.
// vlayer provides it as part of their SDK.
// SHA only needs the interface above.
// ---------------------------------------------------------------------------
