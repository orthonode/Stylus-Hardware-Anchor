/**
 * tiny-keccak.c - Ethereum-compatible Keccak-256 implementation for ESP32
 * 
 * Based on the Keccak-tiny implementation by coruus
 * https://github.com/coruus/keccak-tiny
 * 
 * IMPORTANT: This uses Keccak padding (0x01) NOT SHA3 padding (0x06)
 * This is required for Ethereum/Arbitrum/Stylus compatibility.
 * 
 * Copyright: 2013 Aleksey Kravchenko <rhash.admin@gmail.com>
 * License: MIT
 */

#include <string.h>
#include <stdint.h>
#include "sha3.h"

/* Keccak round constants */
static const uint64_t keccakf_rndc[24] = {
    0x0000000000000001ULL, 0x0000000000008082ULL, 0x800000000000808aULL,
    0x8000000080008000ULL, 0x000000000000808bULL, 0x0000000080000001ULL,
    0x8000000080008081ULL, 0x8000000000008009ULL, 0x000000000000008aULL,
    0x0000000000000088ULL, 0x0000000080008009ULL, 0x000000008000000aULL,
    0x000000008000808bULL, 0x800000000000008bULL, 0x8000000000008089ULL,
    0x8000000000008003ULL, 0x8000000000008002ULL, 0x8000000000000080ULL,
    0x000000000000800aULL, 0x800000008000000aULL, 0x8000000080008081ULL,
    0x8000000000008080ULL, 0x0000000080000001ULL, 0x8000000080008008ULL
};

/* Rotation offsets */
static const unsigned keccakf_rotc[24] = {
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14,
    27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44
};

/* Pi lane permutation */
static const unsigned keccakf_piln[24] = {
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4,
    15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1
};

/* Rotate left macro */
#define ROTL64(x, y) (((x) << (y)) | ((x) >> (64 - (y))))

/**
 * Keccak-f[1600] permutation
 */
static void keccakf(uint64_t st[25]) {
    for (int round = 0; round < 24; round++) {
        uint64_t bc[5], t;
        
        /* Theta step */
        for (int i = 0; i < 5; i++) {
            bc[i] = st[i] ^ st[i + 5] ^ st[i + 10] ^ st[i + 15] ^ st[i + 20];
        }
        
        for (int i = 0; i < 5; i++) {
            t = bc[(i + 4) % 5] ^ ROTL64(bc[(i + 1) % 5], 1);
            for (int j = 0; j < 25; j += 5) {
                st[j + i] ^= t;
            }
        }
        
        /* Rho Pi step */
        t = st[1];
        for (int i = 0; i < 24; i++) {
            int j = keccakf_piln[i];
            bc[0] = st[j];
            st[j] = ROTL64(t, keccakf_rotc[i]);
            t = bc[0];
        }
        
        /* Chi step */
        for (int j = 0; j < 25; j += 5) {
            for (int i = 0; i < 5; i++) {
                bc[i] = st[j + i];
            }
            for (int i = 0; i < 5; i++) {
                st[j + i] ^= (~bc[(i + 1) % 5]) & bc[(i + 2) % 5];
            }
        }
        
        /* Iota step */
        st[0] ^= keccakf_rndc[round];
    }
}

/* ============================================================================
 * SHA3 Context Initialization Functions
 * ============================================================================ */

void sha3_224_Init(SHA3_CTX *ctx) {
    memset(ctx, 0, sizeof(SHA3_CTX));
    ctx->block_size = SHA3_224_BLOCK_LENGTH;
}

void sha3_256_Init(SHA3_CTX *ctx) {
    memset(ctx, 0, sizeof(SHA3_CTX));
    ctx->block_size = SHA3_256_BLOCK_LENGTH;
}

void sha3_384_Init(SHA3_CTX *ctx) {
    memset(ctx, 0, sizeof(SHA3_CTX));
    ctx->block_size = SHA3_384_BLOCK_LENGTH;
}

void sha3_512_Init(SHA3_CTX *ctx) {
    memset(ctx, 0, sizeof(SHA3_CTX));
    ctx->block_size = SHA3_512_BLOCK_LENGTH;
}

/* ============================================================================
 * SHA3 Update Function
 * ============================================================================ */

void sha3_Update(SHA3_CTX *ctx, const unsigned char *msg, size_t size) {
    size_t idx = ctx->rest;
    ctx->rest = (ctx->rest + size) % ctx->block_size;
    
    /* Fill partial block */
    if (idx) {
        size_t left = ctx->block_size - idx;
        memcpy((uint8_t*)ctx->message + idx, msg, (size < left ? size : left));
        if (size < left) return;
        
        /* XOR block into state */
        for (size_t i = 0; i < ctx->block_size; i++) {
            ((uint8_t*)ctx->hash)[i] ^= ((uint8_t*)ctx->message)[i];
        }
        keccakf(ctx->hash);
        msg += left;
        size -= left;
    }
    
    /* Process full blocks */
    while (size >= ctx->block_size) {
        for (size_t i = 0; i < ctx->block_size; i++) {
            ((uint8_t*)ctx->hash)[i] ^= msg[i];
        }
        keccakf(ctx->hash);
        msg += ctx->block_size;
        size -= ctx->block_size;
    }
    
    /* Store remaining bytes */
    if (size) {
        memcpy(ctx->message, msg, size);
    }
}

/* ============================================================================
 * SHA3 Final Function (NIST SHA3 padding: 0x06)
 * ============================================================================ */

void sha3_Final(SHA3_CTX *ctx, unsigned char *result) {
    size_t digest_length;
    
    /* Determine digest length from block size */
    switch (ctx->block_size) {
        case SHA3_224_BLOCK_LENGTH: digest_length = sha3_224_hash_size; break;
        case SHA3_256_BLOCK_LENGTH: digest_length = sha3_256_hash_size; break;
        case SHA3_384_BLOCK_LENGTH: digest_length = sha3_384_hash_size; break;
        case SHA3_512_BLOCK_LENGTH: digest_length = sha3_512_hash_size; break;
        default: digest_length = sha3_256_hash_size;
    }
    
    /* Pad with SHA3 domain separator (0x06) */
    memset((uint8_t*)ctx->message + ctx->rest, 0, ctx->block_size - ctx->rest);
    ((uint8_t*)ctx->message)[ctx->rest] = 0x06;  /* SHA3 padding */
    ((uint8_t*)ctx->message)[ctx->block_size - 1] |= 0x80;
    
    /* XOR final block */
    for (size_t i = 0; i < ctx->block_size; i++) {
        ((uint8_t*)ctx->hash)[i] ^= ((uint8_t*)ctx->message)[i];
    }
    keccakf(ctx->hash);
    
    /* Copy result */
    memcpy(result, ctx->hash, digest_length);
}

#if USE_KECCAK
/* ============================================================================
 * Keccak Final Function (Ethereum padding: 0x01)
 * ============================================================================ */

void keccak_Final(SHA3_CTX *ctx, unsigned char *result) {
    size_t digest_length;
    
    /* Determine digest length from block size */
    switch (ctx->block_size) {
        case SHA3_224_BLOCK_LENGTH: digest_length = sha3_224_hash_size; break;
        case SHA3_256_BLOCK_LENGTH: digest_length = sha3_256_hash_size; break;
        case SHA3_384_BLOCK_LENGTH: digest_length = sha3_384_hash_size; break;
        case SHA3_512_BLOCK_LENGTH: digest_length = sha3_512_hash_size; break;
        default: digest_length = sha3_256_hash_size;
    }
    
    /* Pad with Keccak domain separator (0x01) - ETHEREUM COMPATIBLE */
    memset((uint8_t*)ctx->message + ctx->rest, 0, ctx->block_size - ctx->rest);
    ((uint8_t*)ctx->message)[ctx->rest] = 0x01;  /* Keccak padding (NOT 0x06!) */
    ((uint8_t*)ctx->message)[ctx->block_size - 1] |= 0x80;
    
    /* XOR final block */
    for (size_t i = 0; i < ctx->block_size; i++) {
        ((uint8_t*)ctx->hash)[i] ^= ((uint8_t*)ctx->message)[i];
    }
    keccakf(ctx->hash);
    
    /* Copy result */
    memcpy(result, ctx->hash, digest_length);
}

/* ============================================================================
 * Convenience Functions - Keccak (Ethereum-compatible)
 * ============================================================================ */

void keccak_256(const unsigned char *data, size_t len, unsigned char *digest) {
    SHA3_CTX ctx;
    keccak_256_Init(&ctx);
    keccak_Update(&ctx, data, len);
    keccak_Final(&ctx, digest);
}

void keccak_512(const unsigned char *data, size_t len, unsigned char *digest) {
    SHA3_CTX ctx;
    keccak_512_Init(&ctx);
    keccak_Update(&ctx, data, len);
    keccak_Final(&ctx, digest);
}
#endif /* USE_KECCAK */

/* ============================================================================
 * Convenience Functions - SHA3 (NIST standard)
 * ============================================================================ */

void sha3_256(const unsigned char *data, size_t len, unsigned char *digest) {
    SHA3_CTX ctx;
    sha3_256_Init(&ctx);
    sha3_Update(&ctx, data, len);
    sha3_Final(&ctx, digest);
}

void sha3_512(const unsigned char *data, size_t len, unsigned char *digest) {
    SHA3_CTX ctx;
    sha3_512_Init(&ctx);
    sha3_Update(&ctx, data, len);
    sha3_Final(&ctx, digest);
}
