# ORTHONODE SYSTEMS™ — Stack Context
# Stylus-Hardware-Anchor (SHA)

> Verification Infrastructure for the Decentralized Stack.
> https://orthonode.xyz | @OrthonodeSys | Arhant Barmate

---

## Where SHA Sits in the Orthonode Stack

```
[ESP32-S3 eFuse]
      |
      v
[SHA — Stylus Hardware Anchor]   ← THIS REPO
  Keccak-256 (0x01 padding)
  4-gate on-chain verification
  117-byte receipt
  Arbitrum Sepolia
      |
      v
[oap — Orthonode Assurance Platform]   TYPE A — PROPRIETARY
  4-Layer Filter Cascade
  Ed25519 device identity
  Checkpoint anchoring
      |
      v
[nexus-core — Nexus Protocol]   TYPE A
  Verify-then-Execute
  Cloudflare Zero Trust ingress
  60/30/10 economic invariant (internal)
  Fail-closed Sentry
```

---

## Type Classification

| Repo | Type | Status |
|:-----|:-----|:-------|
| Stylus-Hardware-Anchor | Type A — Core Infrastructure | Live on Arbitrum Sepolia |
| ton-sha | Type A — Core Infrastructure | Live on TON Testnet |
| oap | Type A — Core Infrastructure (PROPRIETARY) | In development |
| nexus-core | Type A — Core Infrastructure | Phase 1.4.0 active |
| INVARIANT | Type B — Standalone (Bittensor Ideathon) | Sandboxed |
| tix-dao | Type B — Standalone (Solana Hackathon) | Sandboxed |
| orthonode.github.io | Type A — Web Presence | Live at orthonode.xyz |
| arhantbarmate.github.io | Type A — Web Presence | Live at arhantbarmate.github.io |

---

## SHA's Role

SHA is the **silicon identity layer** of the Orthonode stack on Arbitrum.

- Provides the foundational hardware-bound `HW_ID` (32-byte Keccak-256 of ESP32-S3 eFuse)
- The 4-gate on-chain verification model is the canonical template for all Orthonode verification primitives
- ton-sha ports this model to TON using SHA-256 (TON VM native) instead of Keccak-256
- oap references SHA-style identity binding for its Ed25519 device identity layer

## What SHA Does NOT Do

- SHA does not enforce the 60/30/10 economic invariant — that is internal to nexus-core
- SHA does not produce NTS trust scores — that is INVARIANT (Bittensor, Type B, sandboxed)
- SHA does not integrate with INVARIANT or tix-dao — both are Type B and sandboxed

---

## Integration Rules

- Type B repos (INVARIANT, tix-dao) must NOT be integrated with SHA without Arhant's explicit decision
- oap integration with SHA: any cross-repo dependency requires Arhant's explicit decision
- nexus-core referencing SHA: any integration requires Arhant's explicit decision

---

## Key Contacts

- Founder & Lead Engineer: Arhant Barmate
- Organization: Orthonode Infrastructure Labs
- Twitter: @OrthonodeSys
- Web: https://orthonode.xyz
- Personal: https://arhantbarmate.github.io

---

ORTHONODE SYSTEMS™ | Infrastructure Labs // Physical Verification Layer
https://orthonode.xyz | @OrthonodeSys | github.com/orthonode | Arhant Barmate
