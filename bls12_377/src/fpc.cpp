#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define uint128_t __uint128_t
#define MAX 12
#define BITS 32

typedef union { 
    uint64_t as64;
    struct
    {
        uint32_t lo;
        uint32_t hi;
    };
} u64; 


inline
void m(uint32_t* out, uint32_t* out_carry, uint32_t b, uint32_t c) {
#if __arm__
    uint32_t RdLo, RdHi;
    asm (
        "UMULL %[RdLo], %[RdHi], %[Rn], %[Rm]"
        : [RdLo] "=r" (RdLo), [RdHi] "=r" (RdHi)
        : [Rn] "r" (b), [Rm] "r" (c)
    );
    *out = RdLo;
    *out_carry = RdHi;
#else
    u64 rv;
    rv.as64 = (uint64_t) b * (uint64_t) c;
    *out = rv.lo;
    *out_carry = rv.hi;
#endif
}


inline
void ma(uint32_t* out, uint32_t* out_carry, uint32_t a, uint32_t b, uint32_t c) {
#if __arm__
    uint32_t RdLo=a, RdHi=0;
    asm (
        "UMLAL %[RdLo], %[RdHi], %[Rn], %[Rm]"
        : [RdLo] "+r" (RdLo), [RdHi] "+r" (RdHi)
        : [Rn] "r" (b), [Rm] "r" (c)
    );
    *out = RdLo;
    *out_carry = RdHi;
#else
   u64 rv;
   rv.as64 = ((uint64_t) b * (uint64_t) c) + (uint64_t) a;
   *out = rv.lo;
   *out_carry = rv.hi;
#endif
}

inline
void mac(uint32_t* out, uint32_t* out_carry, uint32_t a, uint32_t b, uint32_t c, uint32_t carry) {
#if __arm__
    uint32_t RdLo=(uint32_t)a, RdHi=(uint32_t)carry;
    asm (
        "UMAAL %[RdLo], %[RdHi], %[Rn], %[Rm];"
        : [RdLo] "+r" (RdLo), [RdHi] "+r" (RdHi)
        : [Rn] "r" (b), [Rm] "r" (c)
    );
    *out = RdLo;
    *out_carry = RdHi;
#else
    u64 rv;
    rv.as64 = ((uint64_t) b * (uint64_t) c) + (uint64_t) a + (uint64_t) carry;
    *out = rv.lo;
    *out_carry = rv.hi;
#endif
}

inline
uint64_t add(uint32_t* output, const uint32_t* left, const uint32_t* right, int n) {
    uint64_t carry = 0;
    for(int i=0; i<n; i++){
        carry += (uint64_t)left[i] + (uint64_t)right[i];
        output[i] = (uint32_t) carry;
        carry = carry >> BITS;
    }
    return carry;
}

inline
void sub(uint32_t* output, const uint32_t* left, const uint32_t* right, int n) {
    uint64_t borrow = 0;
    for(int i=0; i<n; i++){
        borrow = (uint64_t)left[i] - ((uint64_t)right[i] + (borrow >> (BITS-1)));
        output[i] = (uint32_t) borrow;
        borrow = borrow >> BITS;
    }
}

inline
uint32_t add12(uint32_t* output, const uint32_t* left, const uint32_t* right) {
#if __arm__
    uint32_t carry = 0, t0, t1;
    asm volatile (
        "LDR %[t0], [ %[l], #0 ]\n\t"
        "LDR %[t1], [ %[r], #0 ]\n\t"
        "ADDS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #0 ]\n\t"

        "LDR %[t0], [ %[l], #4 ]\n\t"
        "LDR %[t1], [ %[r], #4 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #4 ]\n\t"

        "LDR %[t0], [ %[l], #8 ]\n\t"
        "LDR %[t1], [ %[r], #8 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #8 ]\n\t"

        "LDR %[t0], [ %[l], #12 ]\n\t"
        "LDR %[t1], [ %[r], #12 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #12 ]\n\t"

        "LDR %[t0], [ %[l], #16 ]\n\t"
        "LDR %[t1], [ %[r], #16 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #16 ]\n\t"

        "LDR %[t0], [ %[l], #20 ]\n\t"
        "LDR %[t1], [ %[r], #20 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #20 ]\n\t"

        "LDR %[t0], [ %[l], #24 ]\n\t"
        "LDR %[t1], [ %[r], #24 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #24 ]\n\t"

        "LDR %[t0], [ %[l], #28 ]\n\t"
        "LDR %[t1], [ %[r], #28 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #28 ]\n\t"

        "LDR %[t0], [ %[l], #32 ]\n\t"
        "LDR %[t1], [ %[r], #32 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #32 ]\n\t"

        "LDR %[t0], [ %[l], #36 ]\n\t"
        "LDR %[t1], [ %[r], #36 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #36 ]\n\t"

        "LDR %[t0], [ %[l], #40 ]\n\t"
        "LDR %[t1], [ %[r], #40 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #40 ]\n\t"

        "LDR %[t0], [ %[l], #44 ]\n\t"
        "LDR %[t1], [ %[r], #44 ]\n\t"
        "ADCS %[t0], %[t0], %[t1]\n\t"
        "STR %[t0], [ %[o], #44 ]\n\t"

        "ADC %[carry], %[carry], #0"
        : [carry] "+r" (carry), [t0] "+r" (t0), [t1] "+r" (t1)
        : [o] "r" (output), [l] "r" (left), [r] "r" (right)
        : "memory"
    );
    return carry;
#else
    return add(output, left, right, 12);
#endif
}

inline
void mul6(uint32_t* output, const uint32_t* left, const uint32_t* right) {
    uint32_t carry, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10;

    m(&output[0], &carry, left[0], right[0]);
    ma(&t1, &carry, carry, left[0], right[1]);
    ma(&t2, &carry, carry, left[0], right[2]);
    ma(&t3, &carry, carry, left[0], right[3]);
    ma(&t4, &carry, carry, left[0], right[4]);
    ma(&t5, &t6, carry, left[0], right[5]);

    ma(&output[1], &carry, t1, left[1], right[0]);
    mac(&t2, &carry, t2, left[1], right[1], carry);
    mac(&t3, &carry, t3, left[1], right[2], carry);
    mac(&t4, &carry, t4, left[1], right[3], carry);
    mac(&t5, &carry, t5, left[1], right[4], carry);
    mac(&t6, &t7, t6, left[1], right[5], carry);

    ma(&output[2], &carry, t2, left[2], right[0]);
    mac(&t3, &carry, t3, left[2], right[1], carry);
    mac(&t4, &carry, t4, left[2], right[2], carry);
    mac(&t5, &carry, t5, left[2], right[3], carry);
    mac(&t6, &carry, t6, left[2], right[4], carry);
    mac(&t7, &t8, t7, left[2], right[5], carry);

    ma(&output[3], &carry, t3, left[3], right[0]);
    mac(&t4, &carry, t4, left[3], right[1], carry);
    mac(&t5, &carry, t5, left[3], right[2], carry);
    mac(&t6, &carry, t6, left[3], right[3], carry);
    mac(&t7, &carry, t7, left[3], right[4], carry);
    mac(&t8, &t9, t8, left[3], right[5], carry);

    ma(&output[4], &carry, t4, left[4], right[0]);
    mac(&t5, &carry, t5, left[4], right[1], carry);
    mac(&t6, &carry, t6, left[4], right[2], carry);
    mac(&t7, &carry, t7, left[4], right[3], carry);
    mac(&t8, &carry, t8, left[4], right[4], carry);
    mac(&t9, &t10, t9, left[4], right[5], carry);

    ma(&output[5], &carry, t5, left[5], right[0]);
    mac(&output[6], &carry, t6, left[5], right[1], carry);
    mac(&output[7], &carry, t7, left[5], right[2], carry);
    mac(&output[8], &carry, t8, left[5], right[3], carry);
    mac(&output[9], &carry, t9, left[5], right[4], carry);
    mac(&output[10], &output[11], t10, left[5], right[5], carry);
}

inline
void mul12(uint32_t* output, const uint32_t* left, const uint32_t* right) {
    // uint32_t left_low[MAX/2];
    // uint32_t left_high[MAX/2];
    // uint32_t right_low[MAX/2];
    // uint32_t right_high[MAX/2];
    //uint32_t ll[MAX];
    //uint32_t hh[MAX];
    //uint32_t lh[MAX];
    //uint32_t hl[MAX];

    const unsigned int n = 12;
    const unsigned int k = 6;

    uint32_t tmp[n + 1];


    // uint32_t bb[MAX + 2] = {0};

    memset(output + n, 0, n * sizeof(uint32_t));
    // const unsigned int s2 = n - k;

    // memcpy(left_low, left, k * sizeof(uint32_t));
    // memcpy(left_high, left + k, s2 * sizeof(uint32_t));
    // memcpy(right_low, right, k * sizeof(uint32_t));
    // memcpy(right_high, right + k, s2 * sizeof(uint32_t));

    mul6(output, left, right);

    // add_carry(left_low, left_low, left_high, s2);
    // add_carry(right_low, right_low, right_high, s2);
    // karatsuba(bb, left_low, right_low, s2 + 1);
    // sub(bb, bb, ll, 2*s2);
    // sub(bb, bb, hh, 2*s2);
    // add_carry(output + k, output + k, bb, 2*s2);

    mul6(tmp, left, right + k);
    output[k + 12] = add12(output + k, output + k, tmp);

    mul6(tmp, left + k, right);
    output[k + 12] += add12(output + k, output + k, tmp);

    mul6(tmp, left + k, right + k);
    add12(output + n, output + n, tmp);
}

inline
void mul_add64(uint64_t* out,
               uint64_t* out_carry,
               uint64_t a,
               uint64_t b,
               uint64_t c,
               uint64_t d) {
    uint128_t ret = ((uint128_t)a * (uint128_t)b) + (uint128_t)c + (uint128_t)d;
    *out = (uint64_t)ret;
    *out_carry = (uint64_t)(ret >> 64);
}

inline
void add64(uint64_t* out,
           uint64_t* out_carry,
           uint64_t a,
           uint64_t b,
           uint64_t c) {
    uint128_t ret = (uint128_t)a + (uint128_t)b + (uint128_t)c;
    *out = (uint64_t)ret;
    *out_carry = (uint64_t)(ret >> 64);
}

static const uint64_t inv = 9586122913090633727ull;
static const uint64_t modulus[6] = {
    0x8508c00000000001,
    0x170b5d4430000000,
    0x1ef3622fba094800,
    0x1a22d9f300f5138f,
    0xc63b05c06ca1493b,
    0x1ae3a4617c510ea,
};

void montgomery_reduce(uint64_t* output, const uint64_t* t) {
        uint64_t carry, k, _, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11;

        const uint64_t t0 = t[0];
        const uint64_t t1 = t[1];
        const uint64_t t2 = t[2];
        const uint64_t t3 = t[3];
        const uint64_t t4 = t[4];
        const uint64_t t5 = t[5];
        const uint64_t t6 = t[6];
        const uint64_t t7 = t[7];
        const uint64_t t8 = t[8];
        const uint64_t t9 = t[9];
        const uint64_t t10 = t[10];
        const uint64_t t11 = t[11];

        k = t[0] * inv;
        mul_add64(&_,  &carry, k, modulus[0], t0, 0);
        mul_add64(&r1, &carry, k, modulus[1], t1, carry);
        mul_add64(&r2, &carry, k, modulus[2], t2, carry);
        mul_add64(&r3, &carry, k, modulus[3], t3, carry);
        mul_add64(&r4, &carry, k, modulus[4], t4, carry);
        mul_add64(&r5, &carry, k, modulus[5], t5, carry);
        add64(&r6, &r7, t6, 0, carry);

        k = r1 * inv;
        mul_add64(&_,  &carry, k, modulus[0], r1, 0);
        mul_add64(&r2, &carry, k, modulus[1], r2, carry);
        mul_add64(&r3, &carry, k, modulus[2], r3, carry);
        mul_add64(&r4, &carry, k, modulus[3], r4, carry);
        mul_add64(&r5, &carry, k, modulus[4], r5, carry);
        mul_add64(&r6, &carry, k, modulus[5], r6, carry);
        add64(&r7, &r8, t7, r7, carry);

        k = r2 * inv;
        mul_add64(&_,  &carry, k, modulus[0], r2, 0);
        mul_add64(&r3, &carry, k, modulus[1], r3, carry);
        mul_add64(&r4, &carry, k, modulus[2], r4, carry);
        mul_add64(&r5, &carry, k, modulus[3], r5, carry);
        mul_add64(&r6, &carry, k, modulus[4], r6, carry);
        mul_add64(&r7, &carry, k, modulus[5], r7, carry);
        add64(&r8, &r9, t8, r8, carry);

        k = r3 * inv;
        mul_add64(&_,  &carry, k, modulus[0], r3, 0);
        mul_add64(&r4, &carry, k, modulus[1], r4, carry);
        mul_add64(&r5, &carry, k, modulus[2], r5, carry);
        mul_add64(&r6, &carry, k, modulus[3], r6, carry);
        mul_add64(&r7, &carry, k, modulus[4], r7, carry);
        mul_add64(&r8, &carry, k, modulus[5], r8, carry);
        add64(&r9, &r10, t9, r9, carry);

        k = r4 * inv;
        mul_add64(&_,  &carry, k, modulus[0], r4, 0);
        mul_add64(&r5, &carry, k, modulus[1], r5, carry);
        mul_add64(&r6, &carry, k, modulus[2], r6, carry);
        mul_add64(&r7, &carry, k, modulus[3], r7, carry);
        mul_add64(&r8, &carry, k, modulus[4], r8, carry);
        mul_add64(&r9, &carry, k, modulus[5], r9, carry);
        add64(&r10, &r11, t10, r10, carry);

        k = r5 * inv;
        mul_add64(&_,   &carry, k, modulus[0], r5, 0);
        mul_add64(&r6,  &carry, k, modulus[1], r6, carry);
        mul_add64(&r7,  &carry, k, modulus[2], r7, carry);
        mul_add64(&r8,  &carry, k, modulus[3], r8, carry);
        mul_add64(&r9,  &carry, k, modulus[4], r9, carry);
        mul_add64(&r10, &carry, k, modulus[5], r10, carry);
        add64(&r11, &_, t11, r11, carry);

        output[0] = r6;
        output[1] = r7;
        output[2] = r8;
        output[3] = r9;
        output[4] = r10;
        output[5] = r11;
    }

extern "C" void c_mul(uint64_t* output, const uint64_t* left, const uint64_t* right) {
    uint64_t tmp[24];
    mul12((uint32_t*)tmp, (const uint32_t*)left, (const uint32_t*)right);
    montgomery_reduce(output, tmp);
}
