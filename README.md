# Stylus Hardware Anchor (SHA)
![CI Status](https://github.com/arhantbarmate/stylus-hardware-anchor/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Arbitrum](https://img.shields.io/badge/Arbitrum-Stylus-blue.svg)
![Rust](https://img.shields.io/badge/Rust-WASM-orange.svg)
![ESP32](https://img.shields.io/badge/Hardware-Bound-green.svg)

**The first protocol-level bridge binding immutable silicon identity to Arbitrum Stylus.**

The **Stylus Hardware Anchor (SHA)** solves the "Physical Trust Gap" in decentralized networks. By cryptographically anchoring **ESP32-S3 eFuse hardware identities** directly into **Rust-based Stylus smart contracts**, we provide a "Sovereign Execution Gate" that makes Sybil attacks physically impossible.

---

## 🏗️ The Infrastructure Standard

Unlike software-only identities (private keys) which can be cloned or leaked, the Stylus Hardware Anchor is bound to the physical silicon. This is critical for:
* **DePIN:** Verifying that a physical node actually exists and is performing work.
* **Sybil Resistance:** Ensuring 1 physical device = 1 on-chain identity.
* **Hardware Governance:** Permitting only authorized, tamper-proof firmware to interact with your protocol.

## 🎯 Milestone 1: COMPLETE ✅
*Status: Verified on Arbitrum Sepolia*

- **Stylus Anchor Contract:** Deployed and functional on-chain.
- **Hardware Root of Trust:** eFuse-backed identity extraction for ESP32-S3.
- **Cryptographic Parity:** 1:1 matching between hardware-generated receipts and Stylus verification logic.
- **Elite CI Pipeline:** Automated security audits, firmware builds, and contract size monitoring.

---

## 🔗 Live Evidence (Milestone 1)

| Artifact | Link |
| :--- | :--- |
| **Stylus Contract** | [`0x34645ff1dd8af86176fe6b28812aaa4d85e33b0d`](https://sepolia.arbiscan.io/address/0x34645ff1dd8af86176fe6b28812aaa4d85e33b0d) |
| **Verification TX** | [`0x84aa8ded...991551c`](https://sepolia.arbiscan.io/tx/0x84aa8ded972c43baefb711089c54d9730f7964e85444596137b76f4e5991551c) |
| **v1.0.0 Release** | [View Release](https://github.com/arhantbarmate/stylus-hardware-anchor/releases/tag/v1.0.0) |

---

## 🛠️ Technical Architecture

### 1. Silicon Layer (ESP32-S3)
Extracts unique, manufacturer-burned eFuse IDs to create a 32-byte **Hardware Identity (HW_ID)**. No two chips are the same.

### 2. Verification Layer (Rust/WASM)
A Stylus-native contract that reconstructs cryptographic digests from a 116-byte receipt format to verify physical origin.

### 3. Governance Layer (Monotonic Counters)
Prevents replay attacks by tracking unique execution counts on-chain for every authorized device.

---

## 🛣️ Roadmap: The Path to Production

- [x] **Milestone 1:** Silicon-to-Stylus Identity Binding (Identity Anchor).
- [ ] **Milestone 2:** Production Hardening (Keccak-256 Integration & Secure Boot).
- [ ] **Milestone 3:** Developer SDK & Verification Dashboard (Public Utility).
- [ ] **Milestone 4:** DePIN Integration Templates (Real-world use cases).

---

## 📖 Technical Dossier
* [ARCHITECTURE.md](docs/ARCHITECTURE.md) - How the silicon-to-WASM bridge works.
* [TECHNICAL_CHALLENGES.md](docs/TECHNICAL_CHALLENGES.md) - Resolution of WSL/Rust cross-compilation hurdles.
* [MILESTONE_1.md](docs/MILESTONE_1.md) - Evidence for the Arbitrum Foundation.

---

## 📄 License
MIT - See [LICENSE](LICENSE)

## 🤝 Support
Developed for the **Arbitrum ecosystem** to enable the next generation of secure, hardware-bound decentralized applications.
