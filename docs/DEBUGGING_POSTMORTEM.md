# Debugging Postmortem — ABI Export + Empty `0x` Reverts

This document captures the key deployment/debugging loop we hit and the exact resolution.

## Symptoms

- Contract deployed successfully.
- `initialize()` succeeded.
- Many other calls reverted with **empty revert data**: `data: "0x"`.
- `getOwner()` reverted when called with an incorrect signature/selector, but the raw selector call returned the correct owner.

## Root cause

There were **two overlapping issues**:

### 1) ABI export was not wired correctly for `stylus-sdk 0.6.x`

- The contract crate had no ABI-export entrypoint that `cargo stylus export-abi` could run.
- Attempting to use the newer `print_from_args()` pattern (Stylus 0.10.x) failed on 0.6.x.

**Fix:** use the 0.6.x ABI printer:

- `stylus_sdk::abi::export::print_abi::<YourEntrypoint>(license, pragma)`

and ensure the crate has a `[[bin]]` target so `cargo stylus export-abi` can execute it.

### 2) Function name mismatch (snake_case vs exported camelCase)

Stylus 0.6.x exports external method names as **camelCase**.

So the exported ABI contains:

- `getOwner()` not `get_owner()`
- `authorizeNode(bytes32)` not `authorize_node(bytes32)`

When `cast` was given the snake_case signature, it computed a **different selector**, the router did not find a matching method, and the call reverted with empty `0x`.

## Resolution

- Generate ABI via:

```bash
cd stylus_anchor/stylus_hardware_anchor
cargo stylus export-abi --rust-features export-abi > abi.json
```

- Use the **exported** method names for all `cast call/send` operations.

- Ensure your shell variables are set (empty `$RPC/$CONTRACT/$PK` caused additional confusing parsing errors).

## Confirmed working proof

- `getOwner()` returned the expected owner.
- Owner key matched sender.
- `authorizeNode(bytes32)` succeeded (`status 1`), and `isNodeAuthorized(bytes32)` returned `true`.

## Notes on legacy deployments

If an earlier contract was deployed without owner gating, it cannot be “deleted” from the chain. The correct approach is:

- Treat it as **deprecated**
- Point all tooling/docs to the **current** contract
- Avoid publishing admin keys
- Optionally publish a warning in docs that the old address is not supported
