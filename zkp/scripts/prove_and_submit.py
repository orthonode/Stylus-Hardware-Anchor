#!/usr/bin/env python3
"""
prove_and_submit.py
───────────────────
SHA × vlayer — Off-chain prover + on-chain submission script.

Phase 1: Scaffold with full interface defined.
Phase 2: Replace TODO sections with vlayer CLI calls and live RPC calls.

Flow:
  1. Read execution_data from device (or file)
  2. Call vlayer prover to generate zk_proof
  3. Build full receipt material and compute digest
  4. Submit verify_receipt_with_zk() to SHA v2 contract on Sepolia

Usage (Phase 2+):
  python prove_and_submit.py \
    --hw-id   0xABCD... \
    --fw-hash 0x1234... \
    --exec-data execution_output.json \
    --counter 42

Dependencies (install in venv):
  pip install web3 eth-account eth-hash[pycryptodome] click
"""

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Configuration — set via environment or .env
# ---------------------------------------------------------------------------

RPC_URL          = os.getenv("RPC_URL", "https://sepolia-rollup.arbitrum.io/rpc")
CONTRACT_ADDRESS = os.getenv("CONTRACT_ADDRESS", "")   # SHA v2 address (Phase 2 deploy)
PRIVATE_KEY      = os.getenv("PRIVATE_KEY", "")        # no 0x prefix
VLAYER_CIRCUIT   = Path(__file__).parent.parent / "circuits" / "execution_proof.nr"

# Phase 1 ABI stub — will be replaced with generated ABI in Phase 2
SHA_V2_ABI_STUB = [
    {
        "name": "verifyReceiptWithZk",
        "type": "function",
        "inputs": [
            {"name": "hwId",          "type": "bytes32"},
            {"name": "fwHash",        "type": "bytes32"},
            {"name": "execHash",      "type": "bytes32"},
            {"name": "counter",       "type": "uint64"},
            {"name": "claimedDigest", "type": "bytes32"},
            {"name": "zkProof",       "type": "bytes"},
        ],
        "outputs": [],
        "stateMutability": "nonpayable",
    },
    {
        "name": "verifyReceipt",
        "type": "function",
        "inputs": [
            {"name": "hwId",          "type": "bytes32"},
            {"name": "fwHash",        "type": "bytes32"},
            {"name": "execHash",      "type": "bytes32"},
            {"name": "counter",       "type": "uint64"},
            {"name": "claimedDigest", "type": "bytes32"},
        ],
        "outputs": [],
        "stateMutability": "nonpayable",
    },
]

# ---------------------------------------------------------------------------
# Phase 1 stubs — implemented in Phase 2
# ---------------------------------------------------------------------------

def compute_exec_hash(exec_data: dict) -> bytes:
    """
    Compute execution_hash from device execution output.
    Phase 2: implement deterministic hash over exec_data fields.
    Must match the circuit's public input computation exactly.
    """
    raise NotImplementedError("Phase 2: implement exec_hash derivation")


def generate_zk_proof(exec_data: dict, exec_hash: bytes, fw_hash: bytes) -> bytes:
    """
    Call vlayer prover CLI to generate a ZK proof.

    Phase 2 implementation:
        result = subprocess.run([
            "vlayer", "prove",
            "--circuit", str(VLAYER_CIRCUIT),
            "--input", json.dumps({
                "exec_data": exec_data,
                "exec_hash": exec_hash.hex(),
                "fw_hash":   fw_hash.hex(),
            }),
        ], capture_output=True, text=True, check=True)
        return bytes.fromhex(json.loads(result.stdout)["proof"])

    Returns serialized proof bytes (Groth16/PLONK per vlayer backend).
    """
    raise NotImplementedError("Phase 2: implement vlayer prover call")


def build_receipt_material(hw_id: bytes, fw_hash: bytes, exec_hash: bytes, counter: int) -> bytes:
    """
    Build 117-byte receipt material. Must match Rust contract exactly.

    Layout:
      [0:13]   b"anchor_RCT_V1"
      [13:45]  hw_id    (32 bytes)
      [45:77]  fw_hash  (32 bytes)
      [77:109] exec_hash (32 bytes)
      [109:117] counter (8 bytes, big-endian)
    """
    assert len(hw_id)    == 32, "hw_id must be 32 bytes"
    assert len(fw_hash)  == 32, "fw_hash must be 32 bytes"
    assert len(exec_hash)== 32, "exec_hash must be 32 bytes"

    domain = b"anchor_RCT_V1"   # 13 bytes
    assert len(domain) == 13

    material = domain + hw_id + fw_hash + exec_hash + counter.to_bytes(8, "big")
    assert len(material) == 117
    return material


def keccak256(data: bytes) -> bytes:
    """Ethereum-compatible Keccak-256 (0x01 padding)."""
    from eth_hash.auto import keccak
    return keccak(data)


def submit_zk_receipt(
    hw_id: bytes,
    fw_hash: bytes,
    exec_hash: bytes,
    counter: int,
    claimed_digest: bytes,
    zk_proof: bytes,
) -> str:
    """
    Submit verify_receipt_with_zk() to SHA v2 on Sepolia.
    Returns transaction hash.

    Phase 2 implementation:
        from web3 import Web3
        from eth_account import Account

        w3 = Web3(Web3.HTTPProvider(RPC_URL))
        account = Account.from_key(PRIVATE_KEY)
        contract = w3.eth.contract(
            address=Web3.to_checksum_address(CONTRACT_ADDRESS),
            abi=SHA_V2_ABI_STUB
        )
        tx = contract.functions.verifyReceiptWithZk(
            hw_id, fw_hash, exec_hash, counter, claimed_digest, zk_proof
        ).build_transaction({
            "from":  account.address,
            "nonce": w3.eth.get_transaction_count(account.address),
            "gas":   300_000,
        })
        signed = account.sign_transaction(tx)
        tx_hash = w3.eth.send_raw_transaction(signed.rawTransaction)
        return tx_hash.hex()
    """
    raise NotImplementedError("Phase 2: implement on-chain submission")


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(
        description="SHA × vlayer: generate ZK proof and submit to Sepolia"
    )
    parser.add_argument("--hw-id",    required=True, help="Hardware ID (0x hex, 32 bytes)")
    parser.add_argument("--fw-hash",  required=True, help="Firmware hash (0x hex, 32 bytes)")
    parser.add_argument("--exec-data", required=True, help="Path to execution_data.json")
    parser.add_argument("--counter",  required=True, type=int, help="Receipt counter")
    parser.add_argument("--v1-only",  action="store_true", help="Skip ZK, use SHA v1 path")
    args = parser.parse_args()

    hw_id   = bytes.fromhex(args.hw_id.removeprefix("0x"))
    fw_hash = bytes.fromhex(args.fw_hash.removeprefix("0x"))

    with open(args.exec_data) as f:
        exec_data = json.load(f)

    print(f"[SHA×vlayer] hw_id:   {hw_id.hex()}")
    print(f"[SHA×vlayer] fw_hash: {fw_hash.hex()}")
    print(f"[SHA×vlayer] counter: {args.counter}")

    # Phase 2: these will be real calls
    print("[SHA×vlayer] Computing exec_hash...")
    exec_hash = compute_exec_hash(exec_data)         # Phase 2

    print("[SHA×vlayer] Building receipt material...")
    material = build_receipt_material(hw_id, fw_hash, exec_hash, args.counter)
    claimed_digest = keccak256(material)
    print(f"[SHA×vlayer] digest:  {claimed_digest.hex()}")

    if args.v1_only:
        print("[SHA×vlayer] Submitting SHA v1 receipt (no ZK)...")
        # Phase 2: submit_v1_receipt(...)
        raise NotImplementedError("Phase 2")
    else:
        print("[SHA×vlayer] Generating ZK proof via vlayer prover...")
        zk_proof = generate_zk_proof(exec_data, exec_hash, fw_hash)   # Phase 2
        print(f"[SHA×vlayer] Proof size: {len(zk_proof)} bytes")

        print("[SHA×vlayer] Submitting ZK-verified receipt...")
        tx_hash = submit_zk_receipt(                                    # Phase 2
            hw_id, fw_hash, exec_hash,
            args.counter, claimed_digest, zk_proof
        )
        print(f"[SHA×vlayer] ✅ TX submitted: {tx_hash}")
        print(f"[SHA×vlayer] Arbiscan: https://sepolia.arbiscan.io/tx/{tx_hash}")


if __name__ == "__main__":
    main()
