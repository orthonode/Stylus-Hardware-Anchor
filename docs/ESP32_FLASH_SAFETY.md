# ESP32-S3 Flashing Safety Notes (DevKit / Prototype)

This project’s on-chain authorization is based on a **hardware identity** derived from immutable chip properties (eFuse-backed identifiers) and a hash function.

## What is safe (non-destructive)

- Flashing firmware via USB using `esptool.py`/PlatformIO is a normal development workflow.
- Reflashing the same device does **not** physically damage the chip.
- Reading chip identifiers (MAC/chip info) is read-only and safe.

## What can permanently change a chip (avoid unless you know exactly why)

ESP32-S3 has one-time programmable security features. Enabling these incorrectly can permanently lock you out of future flashing/debugging:

- Secure Boot (burning related eFuses)
- Flash Encryption (burning related eFuses)
- Disabling JTAG / UART download mode via eFuses

If you are doing prototype development, keep these **disabled** unless you have a tested production provisioning plan.

## Authorization cannot harm the chip

On-chain authorization (`authorizeNode(bytes32)`) only writes to **smart contract storage**. It does not send anything to the ESP32, and it cannot modify chip state.

The only way the chip changes is via:

- flashing firmware
- writing to NVS/flash from firmware
- burning eFuses (should not happen in this repo’s default flashing flow)

## Recommended safe workflow

- Keep firmware flashing in development mode.
- Do not run any tooling that burns eFuses.
- Treat keys as disposable testnet keys.
- Verify identity stability by rebooting and confirming the Hardware ID stays constant.

## Operational note

If you change the identity derivation logic (or the hash implementation), the computed `HW_ID` can change. That is not chip damage — it’s a software identity format change. In that case you must re-authorize the new `HW_ID` on-chain.
