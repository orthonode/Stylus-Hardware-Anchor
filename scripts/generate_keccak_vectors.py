#!/usr/bin/env python3
"""
Generate 10,000 Keccak-256 test vectors for cross-platform parity validation.

Keccak-256 uses 0x01 padding (Ethereum-compatible).
NOT NIST SHA-3 (0x06 padding) — these produce different digests.

Output: tests/keccak_vectors.json

Vector categories:
  1000  — empty and short inputs (0–15 bytes)
  2000  — 16-byte eFuse identity material format
  5000  — 117-byte full receipt format (DOMAIN|HW_ID|FW_HASH|EXEC_HASH|COUNTER)
  2000  — arbitrary random-length inputs (16–256 bytes)
"""

import json
import os
import secrets
import struct
import sys
from pathlib import Path

from eth_hash.auto import keccak

DOMAIN = b"anchor_RCT_V1"  # 13 bytes
OUTPUT_PATH = Path(__file__).parent.parent / "tests" / "keccak_vectors.json"


def make_vector(input_bytes: bytes, label: str) -> dict:
    digest = keccak(input_bytes)
    return {
        "label": label,
        "input": input_bytes.hex(),
        "expected": digest.hex(),
    }


def generate_vectors() -> list[dict]:
    vectors: list[dict] = []
    rng = secrets.token_bytes

    # Category 1: empty and short inputs (0–15 bytes), 1000 vectors
    for length in range(16):
        count = 63 if length > 0 else 1  # 1 empty + 63*15 = 946, top up below
        for _ in range(count):
            vectors.append(make_vector(rng(length), f"short_len{length}"))
    # top up to exactly 1000
    while len(vectors) < 1000:
        length = secrets.randbelow(16)
        vectors.append(make_vector(rng(length), f"short_top"))

    # Category 2: 16-byte eFuse material format, 2000 vectors
    for i in range(2000):
        material = rng(16)
        vectors.append(make_vector(material, f"efuse_material"))

    # Category 3: 117-byte full receipt format, 5000 vectors
    for i in range(5000):
        hw_id = rng(32)
        fw_hash = rng(32)
        exec_hash = rng(32)
        counter = secrets.randbelow(2**64)
        receipt = DOMAIN + hw_id + fw_hash + exec_hash + struct.pack(">Q", counter)
        assert len(receipt) == 117
        vectors.append(make_vector(receipt, "receipt_117"))

    # Category 4: random-length inputs 16–256 bytes, 2000 vectors
    for i in range(2000):
        length = 16 + secrets.randbelow(241)
        vectors.append(make_vector(rng(length), f"random_len{length}"))

    assert len(vectors) == 10000, f"Expected 10000 vectors, got {len(vectors)}"
    return vectors


def main():
    vectors = generate_vectors()
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_PATH, "w") as f:
        json.dump(
            {
                "version": "1.0",
                "algorithm": "keccak-256",
                "padding": "0x01 (Ethereum-compatible)",
                "note": "NOT NIST SHA-3 (0x06). Cross-platform parity validation for SHA receipt format.",
                "count": len(vectors),
                "vectors": vectors,
            },
            f,
            indent=2,
        )
    print(f"Wrote {len(vectors)} vectors to {OUTPUT_PATH}")


if __name__ == "__main__":
    main()
