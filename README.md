# Stylus Hardware Anchor (SHA)

![CI Status](https://github.com/arhantbarmate/stylus-hardware-anchor/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Arbitrum](https://img.shields.io/badge/Arbitrum-Stylus-blue.svg)
![Rust](https://img.shields.io/badge/Rust-WASM-orange.svg)
![ESP32](https://img.shields.io/badge/Hardware--Bound-green.svg)

**A protocol-level bridge binding immutable silicon identity to Arbitrum Stylus.**

Stylus Hardware Anchor (SHA) is a reusable hardware identity verification primitive designed for Stylus smart contracts. It cryptographically binds immutable ESP32-S3 eFuse identifiers to on-chain execution logic, enabling deterministic device identity, replay-safe receipts, and firmware-governed access control.

Preliminary benchmark: ~12.5k–29.7k gas per receipt in batched verification on Arbitrum Sepolia (amortization improves with batch size; see BENCHMARKS.md). Stylus enables high-throughput hardware receipt verification via WASM batch execution when applications need to verify many receipts in a single transaction; otherwise, single verification remains available.

---

## 🏗️ Hardware Identity Primitive for Stylus

SHA provides a structured approach to hardware-bound execution in decentralized systems. By moving from software-only private keys to silicon-bound cryptographic proofs, it enables:

- **Device-Bound Identity:** 1 physical device ↔ 1 on-chain identity  
- **Replay-Safe Receipts:** Monotonic counter enforcement  
- **Firmware Governance:** Execution restricted to approved firmware versions  
- **Deterministic Verification:** Ethereum-compatible Keccak-256 (0x01 padding)

This project is focused on providing reusable infrastructure, not a standalone application.

---

## Current Status

- Prototype deployed on Arbitrum Sepolia  
- Firmware ↔ Python ↔ Stylus cryptographic parity achieved  
- ≥10,000 deterministic test vectors validated  
- Security hardening (Phase 1 — Milestone 1) pending grant approval  

No professional third-party audit has been conducted. Mainnet deployment is not part of the current scope.

---

## Start Here (Docs Index)

This repo is intentionally **Stylus-first (Rust/WASM)**. For tooling like `cast`, always call the **exported ABI names** (Stylus SDK 0.6.x exports camelCase, e.g. `getOwner`, `authorizeNode`).

Recommended reading order:

1. [`SETUP_GUIDE.md`](SETUP_GUIDE.md)
2. [`docs/DEPLOYMENT_GUIDE.md`](docs/DEPLOYMENT_GUIDE.md)
3. [`docs/CAST_CHEATSHEET.md`](docs/CAST_CHEATSHEET.md)
4. [`docs/ESP32_FLASH_SAFETY.md`](docs/ESP32_FLASH_SAFETY.md)
5. [`docs/DEBUGGING_POSTMORTEM.md`](docs/DEBUGGING_POSTMORTEM.md)

Configuration is environment-driven:

- `RPC_URL`
- `CONTRACT_ADDRESS`
- `PRIVATE_KEY` (never commit; prefer local key files)

## 🔗 Live Evidence (Sepolia)

| Artifact | Link |
|----------|------|
| Stylus Contract | [`0xD661a1aB8CEFaaCd78F4B968670C3bC438415615`](https://sepolia.arbiscan.io/address/0xD661a1aB8CEFaaCd78F4B968670C3bC438415615) |
| Verification TX | Use your local transaction hashes (not committed) |
| Gas Benchmarks | See `BENCHMARKS.md` for batch verification results |
| v1.0.0 Prototype Release | [View Release](https://github.com/arhantbarmate/stylus-hardware-anchor/releases/tag/v1.0.0) |

---

## 🧼 Optional (Cleaner Dev Workflow)

For a cleaner development environment with isolated Python dependencies:

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
pip install -r requirements-dev.txt
```

`requirements-dev.txt` includes `black` for code formatting and additional testing tools.

---

## 🛠️ Architecture Overview

### 1️⃣ Silicon Layer (ESP32-S3)

- Extracts manufacturer-burned identifiers (Base MAC + Chip Model)  
- Generates 32-byte Hardware Identity (HW_ID)  
- On-device Keccak-256 (Ethereum-compatible, 0x01 padding)

### 2️⃣ Verification Layer (Stylus / Rust / WASM)

- Reconstructs receipt digests  
- Enforces monotonic counter replay protection  
- Validates firmware hash gating  

### 3️⃣ Middleware Layer (Python SDK — Early Access)

- Device authorization flows  
- Receipt generation utilities  
- On-chain submission helpers  

---

## 🛣️ Roadmap

### Phase 1 — Current Grant Scope  
**$49,000 USD — 6 months — Testnet Only (Arbitrum Sepolia)**

- Milestone 1: Security Hardening  
- Milestone 2: Developer SDK & Integration Tooling  

No professional audit, mainnet deployment, or ecosystem expansion is included in Phase 1.

### Phase 2 — Future Grant (Not in Current Scope)

- Professional third-party security audit  
- Mainnet deployment  
- Hardware reference design  
- Broader ecosystem integrations  

See [ROADMAP.md](ROADMAP.md) for full details.

---

## 📖 Technical Documentation

- [ARCHITECTURE.md](docs/ARCHITECTURE.md) — Detailed system design  
- [TECHNICAL_CHALLENGES.md](docs/TECHNICAL_CHALLENGES.md) — Integration logs  
- [MILESTONE_1.md](docs/MILESTONE_1.md) — Prototype validation evidence  

---

## ⚖️ Scope Clarification

SHA is currently a testnet-deployed prototype.  

Phase 1 focuses on:

- Internal security hardening  
- Reproducible validation artifacts  
- Developer tooling  

Professional audit and mainnet deployment are explicitly deferred to Phase 2.

---

## � ZKP Integration (vlayer Grant)

**Branch**: [`feat/zkp-vlayer-integration`](https://github.com/arhantbarmate/stylus-hardware-anchor/tree/feat/zkp-vlayer-integration)  
**Grant Applied**: 2026-02-19  
**Status**: Phase 1 Complete ✅ — Architecture & Interface Design

SHA × vlayer adds **cryptographic execution correctness** to hardware identity verification. This creates a 4-layer security model:

| Layer | Guarantee | Status |
|-------|-----------|--------|
| Silicon Identity | eFuse → Keccak → on-chain allowlist | ✅ Live on Sepolia |
| Firmware Governance | Approved firmware hash gating | ✅ Live on Sepolia |
| Replay Protection | Monotonic counter enforcement | ✅ Live on Sepolia |
| **ZK Execution Proof** | **vlayer circuit + Stylus verifier** | 🔄 Phase 2 |

### Key Features
- **Additive ZK**: Stage 4 verification, preserves SHA v1 guarantees
- **Off-chain Prover**: ESP32 too weak for ZK generation; prover runs separately
- **Audit Mode**: Safe migration with `zk_mode_enabled` flag
- **No Format Changes**: Uses existing `exec_hash` as ZK public input

### Quick Start (Phase 1)
```bash
# Checkout ZKP branch
git checkout feat/zkp-vlayer-integration

# Explore architecture
cat docs/zkp/ARCHITECTURE.md
cat docs/zkp/CIRCUIT_SPEC.md
cat docs/zkp/ZK_ROADMAP.md

# Review interfaces
cat zkp/contracts/IZkVerifier.rs
cat zkp/contracts/sha_v2_interface.rs
```

### Phase Roadmap
- **Phase 1** ✅: Architecture, interfaces, documentation (current)
- **Phase 2** ⏳: Noir circuit + SHA v2 contract + end-to-end testnet
- **Phase 3** ⏳: Batch proof aggregation (N=50 → ~16k gas/receipt)
- **Phase 4** ⏳: Recursive ZK for 1000+ device networks

**Details**: See [`docs/zkp/`](docs/zkp/) directory and [`PROGRESS.md`](PROGRESS.md)

---

## �📄 License

MIT — See [LICENSE](LICENSE)

---

## 🤝 Development Focus

SHA is being developed as reusable infrastructure for the Arbitrum ecosystem, with a disciplined roadmap toward audit preparation and eventual mainnet deployment, contingent on successful Phase 1 completion.
