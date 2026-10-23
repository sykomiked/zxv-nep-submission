/* near_rt.c — the compiler-rt helpers the wasm32 freestanding build needs.
 *
 * clang lowers 128-bit multiplication and division to __multi3, __udivti3
 * and __divti3. Homebrew's clang ships no wasm32 libclang_rt.builtins, so
 * the contract carries its own — the same algorithms compiler-rt uses,
 * written over uint64 halves so they need no 128-bit lowering of their
 * own. The kernel already leans on libgcc helpers for 64-bit division on
 * the 32-bit targets (see surplus.c's own notes); this is that pattern for
 * the on-chain build.
 *
 * Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
 * SPDX-License-Identifier: Apache-2.0
 */
#include <stdint.h>

/* 128-bit multiply (compiler-rt's __multi3), built over 32-bit halves.
 * No 128-bit temporaries: on wasm32 they would lower back into __multi3
 * and recurse forever. Returns the low 128 bits of the product (the
 * a_hi*b_hi term lands at bit 128 and is dropped, which is the correct
 * truncation for a 128-bit result). */
__uint128_t __multi3(__uint128_t a, __uint128_t b)
{
    uint64_t a_lo = (uint32_t) (uint64_t) a, a_hi = (uint64_t) a >> 32;
    uint64_t b_lo = (uint32_t) (uint64_t) b, b_hi = (uint64_t) b >> 32;
    uint64_t a_lo_b_lo = a_lo * b_lo;
    uint64_t a_lo_b_hi = a_lo * b_hi;
    uint64_t a_hi_b_lo = a_hi * b_lo;
    uint64_t a_hi_b_hi = a_hi * b_hi;
    uint64_t t = a_hi_b_lo + (a_lo_b_lo >> 32);
    uint64_t w1 = a_lo_b_hi + (uint32_t) t;
    uint64_t w2 = (uint32_t) w1;
    uint64_t w3 = w1 >> 32;
    uint64_t hi = a_hi_b_hi + (t >> 32) + w3;
    uint64_t lo = (w2 << 32) | (uint32_t) a_lo_b_lo;
    return ((__uint128_t) hi << 64) | lo;
}

/* Unsigned 128-bit division: restoring long division over 64-bit halves. */
__uint128_t __udivti3(__uint128_t a, __uint128_t b)
{
    uint64_t n_lo = (uint64_t) a, n_hi = (uint64_t) (a >> 64);
    uint64_t d_lo = (uint64_t) b, d_hi = (uint64_t) (b >> 64);
    uint64_t q_lo = 0, q_hi = 0;
    unsigned shift = 0;
    if (d_hi == 0 && d_lo == 0) return (__uint128_t) 0 - 1; /* div by 0 */
    /* normalize so the divisor's top bit is set */
    while (!(d_hi & 0x8000000000000000ull)) {
        d_hi = (d_hi << 1) | (d_lo >> 63);
        d_lo <<= 1;
        shift++;
    }
    for (unsigned i = 0; i < 128; i++) {
        /* q <<= 1; n <<= 1 (into the 128-bit carry of q) */
        q_hi = (q_hi << 1) | (q_lo >> 63);
        q_lo <<= 1;
        {
            uint64_t ncarry = n_hi >> 63;
            n_hi = (n_hi << 1) | (n_lo >> 63);
            n_lo <<= 1;
            if (ncarry || n_hi > d_hi || (n_hi == d_hi && n_lo >= d_lo)) {
                /* n -= d (uint64 wrap is the borrow); set the quotient bit */
                uint64_t borrow = 0;
                if (n_lo < d_lo) borrow = 1;
                n_lo -= d_lo;
                n_hi = n_hi - d_hi - borrow;
                q_lo |= 1;
            }
        }
    }
    (void) shift;
    return ((__uint128_t) q_hi << 64) | q_lo;
}

__int128 __divti3(__int128 a, __int128 b)
{
    int neg = ((a < 0) != (b < 0));
    __uint128_t ua = a < 0 ? (__uint128_t) -a : (__uint128_t) a;
    __uint128_t ub = b < 0 ? (__uint128_t) -b : (__uint128_t) b;
    __uint128_t q = __udivti3(ua, ub);
    return neg ? (__int128) 0 - (__int128) q : (__int128) q;
}

/* scp144.c expects plain shake256(in, inlen, out, outlen); the in-tree
 * fips202 ships its shake256 under the dilithium namespace with the
 * (out, outlen, in, inlen) order, so the contract adapts the two. */
extern void pqcrystals_dilithium_fips202_ref_shake256(uint8_t *out,
                                                      size_t outlen,
                                                      const uint8_t *in,
                                                      size_t inlen);

void shake256(const uint8_t *in, size_t inlen, uint8_t *out, size_t outlen)
{
    pqcrystals_dilithium_fips202_ref_shake256(out, outlen, in, inlen);
}