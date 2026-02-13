/**
 * options.h - Build configuration for SHA3/Keccak implementation
 * 
 * This file provides compile-time options for the Keccak/SHA3 implementation.
 */

#ifndef __OPTIONS_H__
#define __OPTIONS_H__

/**
 * USE_KECCAK: Enable Ethereum-compatible Keccak-256
 * 
 * When enabled (1): Uses Keccak padding (0x01) - Ethereum compatible
 * When disabled (0): Uses SHA3 padding (0x06) - NIST standard
 * 
 * For Ethereum/Arbitrum/Stylus compatibility, this MUST be set to 1.
 */
#define USE_KECCAK 1

#endif /* __OPTIONS_H__ */
