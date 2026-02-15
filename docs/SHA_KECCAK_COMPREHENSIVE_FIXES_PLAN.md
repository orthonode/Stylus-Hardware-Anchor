# SHA/Keccak Comprehensive Fixes Plan

## Summary
Fix all identified bugs, errors, and inconsistencies in the SHA/Keccak implementation across the codebase, with primary focus on firmware fixes. The user is using **PlatformIO (Arduino)** build system.

---

## Issues Found (Categorized by Severity)

### CRITICAL (Blocking/Build Failures)

| # | File | Issue | Impact |
|---|------|-------|--------|
| 1 | `ohr_firmware/main/tiny-keccak.c` | **Empty file (0 bytes)** | Linker errors if sha3.h functions used |
| 2 | `ohr_firmware/main/sha3.h:24` | **Missing `#include "options.h"`** | Compilation failure |
| 3 | `ohr_firmware/main/anchor_ohr.cpp` | Uses **SHA3-256 (0x06 padding)** instead of **Keccak-256 (0x01 padding)** | Digest mismatch with Ethereum/Solidity |
| 4 | `oap_witness/Cargo.toml:4` | **Invalid Rust edition "2024"** (doesn't exist) | Build failure |

### HIGH (Logic/Security Errors)

| # | File | Issue | Impact |
|---|------|-------|--------|
| 5 | `ohr_firmware/main/anchor_ohr.cpp:46-47` | Domain length mismatch: `"anchor_OHR_V1"` is 13 chars, `_LEN` says 12 | Truncated hash input |
| 6 | `ohr_firmware/main/anchor_ohr.cpp:45` | Same issue for `anchor_RCT_DOMAIN` | Truncated hash input |
| 7 | `ohr_firmware/src/main.cpp:73` | Unused variable `size_t offset = 0` | Compiler warning |
| 8 | `ohr_firmware/src/main.cpp:77-79` | Unaligned memory access via `uint64_t*` cast | Potential crashes on strict architectures |

### MEDIUM (Inconsistencies/Code Quality)

| # | File | Issue | Impact |
|---|------|-------|--------|
| 9 | `ohr_firmware/main/sha3.h:71` | `USE_KECCAK` never defined | Dead code paths |
| 10 | `ohr_firmware/src/main.cpp:136` | Hardcoded "ESP32-S3" in output | Incorrect on other chips |
| 11 | `stylus_anchor/stylus_hardware_anchor/src/lib.rs:66-80` | Missing access control on admin functions | Anyone can authorize/revoke |
| 12 | `stylus_anchor/stylus_hardware_anchor/Cargo.toml:7` | Old stylus-sdk version "0.6.0" vs "0.10.0" | Version mismatch between contracts |
| 13 | `middleware/anchor_middleware.py:97` | Domain tag comment says 12 bytes but actual tag is 13 | Documentation mismatch |

### LOW (Minor/Cosmetic)

| # | File | Issue | Impact |
|---|------|-------|--------|
| 14 | Two separate Stylus contracts | `stylus_anchor/src/lib.rs` vs `stylus_anchor/stylus_hardware_anchor/src/lib.rs` | Confusion about canonical implementation |
| 15 | `.env.example` vs actual env vars | Uses `PRIVATE_KEY` but scripts use `ANCHOR_HARDWARE_KEY` | Configuration confusion |

---

## Fixes to Implement

### Focus: PlatformIO/Arduino Firmware (`ohr_firmware/src/main.cpp`)

The user confirmed they're using PlatformIO. The good news: **`src/main.cpp` already has correct Keccak-256 with 0x01 padding!**

#### Fix 1: Remove unused variable
**File:** `ohr_firmware/src/main.cpp:73`
```c
// DELETE this line:
size_t offset = 0;  // Unused
```

#### Fix 2: Fix unaligned memory access
**File:** `ohr_firmware/src/main.cpp:76-93`
Replace direct `uint64_t*` casting with byte-by-byte XOR to avoid alignment issues:
```c
void keccak256(const uint8_t* input, size_t len, uint8_t output[32]) {
  uint64_t st[25] = {0};
  const size_t rate = 136;
  
  // Absorb - FIXED: byte-wise XOR to avoid alignment issues
  while (len >= rate) {
    for (size_t i = 0; i < rate; i++) {
      ((uint8_t*)st)[i] ^= input[i];
    }
    keccakf(st);
    input += rate;
    len -= rate;
  }
  
  // Pad
  uint8_t temp[200] = {0};
  memcpy(temp, input, len);
  temp[len] = 0x01;          // Keccak padding (correct!)
  temp[rate - 1] |= 0x80;
  
  for (size_t i = 0; i < rate; i++) {
    ((uint8_t*)st)[i] ^= temp[i];
  }
  keccakf(st);
  
  // Squeeze
  memcpy(output, st, 32);
}
```

#### Fix 3: Fix hardcoded chip model
**File:** `ohr_firmware/src/main.cpp:136`
```c
// CHANGE FROM:
Serial.printf("Chip Model:           ESP32-S3 (Model %d)\n", chip_info.model);

// TO:
Serial.printf("Chip Model:           Model %d\n", chip_info.model);
```

### ESP-IDF Fixes (Secondary - for completeness)

Even though user uses PlatformIO, fix these for future compatibility:

#### Fix 4: Implement tiny-keccak.c
**File:** `ohr_firmware/main/tiny-keccak.c`
Copy the Keccak implementation from `src/main.cpp` (lines 15-98) with proper C syntax.

#### Fix 5: Create options.h
**File:** `ohr_firmware/main/options.h`
```c
#ifndef __OPTIONS_H__
#define __OPTIONS_H__

// Enable Keccak mode (Ethereum-compatible, 0x01 padding)
#define USE_KECCAK 1

#endif
```

#### Fix 6: Fix domain tag lengths
**File:** `ohr_firmware/main/anchor_ohr.cpp:44-47`
```c
// CHANGE FROM:
#define anchor_HWI_DOMAIN_LEN 12
#define anchor_RCT_DOMAIN_LEN 12

// TO:
#define anchor_HWI_DOMAIN_LEN 13
#define anchor_RCT_DOMAIN_LEN 13
```

#### Fix 7: Replace SHA3 with Keccak in anchor_ohr.cpp
**File:** `ohr_firmware/main/anchor_ohr.cpp:72-81`
Replace mbedTLS SHA3 calls with the proper Keccak-256 implementation.

### Rust/Cargo Fixes

#### Fix 8: Fix invalid Rust edition
**File:** `oap_witness/Cargo.toml:4`
```toml
# CHANGE FROM:
edition = "2024"

# TO:
edition = "2021"
```

---

## Files to Modify

| File | Changes |
|------|---------|
| `ohr_firmware/src/main.cpp` | Remove unused var, fix alignment, fix chip model output |
| `ohr_firmware/main/tiny-keccak.c` | Implement Keccak-256 (copy from src/main.cpp) |
| `ohr_firmware/main/options.h` | Create new file with USE_KECCAK=1 |
| `ohr_firmware/main/anchor_ohr.cpp` | Fix domain lengths, add Keccak impl |
| `oap_witness/Cargo.toml` | Fix edition from "2024" to "2021" |

---

## Verification

### 1. PlatformIO Build Test
```bash
cd ohr_firmware
pio run -e esp32-s3-devkitc-1
```
Expected: Build succeeds without warnings

### 2. ESP-IDF Build Test (optional)
```bash
cd ohr_firmware/main
idf.py build
```
Expected: Build succeeds

### 3. Rust Build Test
```bash
cd oap_witness
cargo build
```
Expected: Build succeeds (currently fails due to edition "2024")

### 4. Keccak Hash Verification
Test that firmware produces same hash as Python:
```python
from eth_hash.auto import keccak
test_input = b"test"
print(keccak(test_input).hex())
# Expected: 9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb658
```

### 5. End-to-End Test
Run existing working scripts:
```bash
python3 scripts/test_contract.py
# Should still work as before
```

---

## Out of Scope (Noted but not fixing)

1. **Access control in Stylus contracts** - Would require contract redeployment
2. **Two Stylus contract versions** - User should clarify which is canonical
3. **Environment variable naming** - Scripts work as-is with current .env

---

## Risk Assessment

- **Low Risk:** Firmware changes don't affect working Python/Stylus code
- **No Breaking Changes:** Domain tags aligned to 13 bytes (matches Python)
- **Backwards Compatible:** Existing authorized nodes/digests unaffected
