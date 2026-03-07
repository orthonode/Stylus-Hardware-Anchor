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

RPC_URL            = os.getenv("RPC_URL", "https://sepolia-rollup.arbitrum.io/rpc")
CONTRACT_ADDRESS   = os.getenv("CONTRACT_ADDRESS", "")   # SHA v2 address (Phase 2 deploy)
PRIVATE_KEY        = os.getenv("PRIVATE_KEY", "")        # no 0x prefix
VLAYER_PROVER_URL  = os.getenv("VLAYER_PROVER_URL", "http://localhost:3000")
VLAYER_CHAIN_ID    = int(os.getenv("VLAYER_CHAIN_ID", "421614"))  # Arbitrum Sepolia
VLAYER_CIRCUIT     = Path(__file__).parent.parent / "circuits" / "execution_proof.nr"

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

    exec_data must contain a "raw_output" field: hex-encoded bytes of the
    device's raw computation result. Keccak-256(raw_output) is the public
    input that the vlayer circuit commits to.

    The exec_hash embedded in the receipt must equal this value exactly.
    The ESP32 firmware computes the same hash before building the receipt.
    """
    raw_hex = exec_data.get("raw_output", "")
    if not raw_hex:
        raise ValueError("exec_data must contain 'raw_output' (hex bytes of device output)")
    raw = bytes.fromhex(raw_hex.removeprefix("0x"))
    return keccak256(raw)


def generate_zk_proof(exec_data: dict, exec_hash: bytes, fw_hash: bytes) -> bytes:
    """
    Call the vlayer prover to generate a ZK proof via JSON-RPC.

    Sends a v_call to the vlayer prover service. The circuit proves:
        keccak256(exec_data["raw_output"]) == exec_hash
    exec_data is the private witness; exec_hash is the public input.

    Requires:
      - vlayer prover running at VLAYER_PROVER_URL (default: localhost:3000)
      - Compiled circuit artifact at VLAYER_CIRCUIT (Phase 2 deliverable)
      - VLAYER_PROVER_URL env var (override for testnet prover)

    Returns serialized proof bytes (Groth16/PLONK per vlayer backend).
    """
    import urllib.request

    witness = {
        "circuit": str(VLAYER_CIRCUIT),
        "chainId": VLAYER_CHAIN_ID,
        "publicInputs": {
            "exec_hash": "0x" + exec_hash.hex(),
        },
        "privateInputs": {
            "exec_data": exec_data.get("raw_output", "0x"),
        },
    }

    payload = json.dumps({
        "jsonrpc": "2.0",
        "method": "v_call",
        "params": [witness],
        "id": 1,
    }).encode()

    req = urllib.request.Request(
        VLAYER_PROVER_URL,
        data=payload,
        headers={"Content-Type": "application/json"},
    )

    try:
        with urllib.request.urlopen(req, timeout=120) as resp:
            result = json.load(resp)
    except Exception as e:
        raise RuntimeError(
            f"vlayer prover unreachable at {VLAYER_PROVER_URL}: {e}\n"
            f"Start local prover: vlayer server --circuit {VLAYER_CIRCUIT}"
        ) from e

    if "error" in result:
        raise RuntimeError(f"vlayer prover returned error: {result['error']}")

    proof_hex = result["result"]["proof"]
    return bytes.fromhex(proof_hex.removeprefix("0x"))


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
    Submit verify_receipt_with_zk() to SHA v2 on Arbitrum Sepolia.
    Returns transaction hash (hex string with 0x prefix).

    Requires:
      - RPC_URL env var (default: Arbitrum Sepolia RPC)
      - CONTRACT_ADDRESS env var (SHA v2 deployed address — Phase 2)
      - PRIVATE_KEY env var (submitter key, no 0x prefix)
    """
    from web3 import Web3
    from eth_account import Account

    if not CONTRACT_ADDRESS:
        raise ValueError("CONTRACT_ADDRESS not set — SHA v2 not yet deployed (Phase 2)")
    if not PRIVATE_KEY:
        raise ValueError("PRIVATE_KEY not set")

    w3 = Web3(Web3.HTTPProvider(RPC_URL))
    if not w3.is_connected():
        raise RuntimeError(f"Cannot connect to RPC: {RPC_URL}")

    account = Account.from_key(PRIVATE_KEY)
    contract = w3.eth.contract(
        address=Web3.to_checksum_address(CONTRACT_ADDRESS),
        abi=SHA_V2_ABI_STUB,
    )

    tx = contract.functions.verifyReceiptWithZk(
        hw_id,
        fw_hash,
        exec_hash,
        counter,
        claimed_digest,
        zk_proof,
    ).build_transaction({
        "from":  account.address,
        "nonce": w3.eth.get_transaction_count(account.address),
        "gas":   350_000,
        "chainId": 421614,  # Arbitrum Sepolia
    })

    signed = account.sign_transaction(tx)
    tx_hash = w3.eth.send_raw_transaction(signed.raw_transaction)
    receipt = w3.eth.wait_for_transaction_receipt(tx_hash, timeout=120)

    if receipt.status != 1:
        raise RuntimeError(f"Transaction reverted: {tx_hash.hex()}")

    return "0x" + tx_hash.hex()


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
