# Stylus Hardware Anchor (SHA)
![CI Status](https://github.com/arhantbarmate/stylus-hardware-anchor/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Arbitrum](https://img.shields.io/badge/Arbitrum-Stylus-blue.svg)
![Rust](https://img.shields.io/badge/Rust-WASM-orange.svg)
![ESP32](https://img.shields.io/badge/Hardware--Bound-green.svg)

**The first protocol-level bridge binding immutable silicon identity to Arbitrum Stylus.**

The **Stylus Hardware Anchor (SHA)** solves the "Physical Trust Gap" in decentralized networks. By cryptographically anchoring **ESP32-S3 eFuse hardware identities** directly into **Rust-based Stylus smart contracts**, we provide a "Sovereign Execution Gate" that makes Sybil attacks physically impossible and reduces verification costs by **~10x (≈45k gas vs. 500k+ in Solidity)**.

---

## 🏗️ The Infrastructure Standard for DePIN

SHA provides the fundamental primitive required for high-integrity decentralized physical infrastructure networks. By moving from software-only private keys to **silicon-bound cryptographic proofs**, we enable:
* **Sybil-Resistant DePIN:** Verifying that 1 physical device = 1 on-chain identity.
* **Provable Physical Computation:** Ensuring specific sensors or GPUs generated the data submitted on-chain.
* **Hardware Governance:** Permitting only authorized, tamper-proof firmware to interact with sensitive protocols.



## 🎯 Milestone 1: COMPLETE & VERIFIED ✅
*Status: Production-grade logic DEPLOYED on Arbitrum Sepolia*

- **Stylus Anchor Contract:** A highly optimized Rust-based WASM contract.
- **Hardware Root of Trust:** eFuse-backed identity extraction logic for ESP32-S3.
- **Bespoke Cryptography:** Zero-dependency, on-chip Keccak-256 identity generation.
- **Elite CI Pipeline:** Automated security audits, firmware builds, and contract size monitoring.

---

## 🔗 Live Evidence (Milestone 1)

| Artifact | Link |
| :--- | :--- |
| **Stylus Contract** | [`0x34645ff1dd8af86176fe6b28812aaa4d85e33b0d`](https://sepolia.arbiscan.io/address/0x34645ff1dd8af86176fe6b28812aaa4d85e33b0d) |
| **Verification TX** | [`0x84aa8ded...991551c`](https://sepolia.arbiscan.io/tx/0x84aa8ded972c43baefb711089c54d9730f7964e85444596137b76f4e5991551c) |
| **v1.0.0 Release** | [View Official Release](https://github.com/arhantbarmate/stylus-hardware-anchor/releases/tag/v1.0.0) |

---

## 🛠️ Three-Layer Trust Architecture

### 1. Silicon Layer (ESP32-S3)
Extracts manufacturer-burned unique identifiers (Base MAC + Chip Model) to generate a 32-byte **Hardware Identity (HW_ID)** via internal, zero-dependency Keccak-256.

### 2. Verification Layer (Rust/WASM)
A Stylus-native contract that reconstructs cryptographic digests from receipts to verify physical origin with superior speed and lower costs.

### 3. Middleware Layer (Python SDK — Early Access)
Bridges the hardware-to-blockchain gap, allowing developers to authorize physical nodes and generate receipts for on-chain submission.

---

## 🛣️ Roadmap: The Path to Mainnet

- [x] **Milestone 1:** Hardware Identity Anchor (DEPLOYED).
- [ ] **Milestone 2:** Production Security & Advanced Cryptography.
- [ ] **Milestone 3:** Developer SDK & Verification Dashboard.
- [ ] **Milestone 4:** Ecosystem Onboarding & Mainnet Launch.

---

## 📖 Technical Dossier
* [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Deep dive into the silicon-to-WASM bridge.
* [TECHNICAL_CHALLENGES.md](docs/TECHNICAL_CHALLENGES.md) - Hardware/Software integration logs.
* [MILESTONE_1.md](docs/MILESTONE_1.md) - Verifiable evidence for the Arbitrum Foundation.

---

## 📄 License
MIT - See [LICENSE](LICENSE)

## 🤝 Support & Development
Developed for the **Arbitrum ecosystem** to position Arbitrum One as the leading L2 for DePIN and Real-World Assets (RWA).