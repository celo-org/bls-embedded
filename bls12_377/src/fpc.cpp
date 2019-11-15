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
void mul_add32(uint32_t* out, uint32_t* out_carry, uint32_t b, uint32_t c) {
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
void mul_add32(uint32_t* out, uint32_t* out_carry, uint32_t b, uint32_t c, uint32_t a) {
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
void mul_add32(uint32_t* out, uint32_t* out_carry, uint32_t b, uint32_t c, uint32_t a, uint32_t carry) {
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
void m(uint32_t* out, uint32_t* out_carry, uint32_t b, uint32_t c) {
    mul_add32(out, out_carry, b, c);
}


inline
void ma(uint32_t* out, uint32_t* out_carry, uint32_t a, uint32_t b, uint32_t c) {
    mul_add32(out, out_carry, b, c, a);
}

inline
void mac(uint32_t* out, uint32_t* out_carry, uint32_t a, uint32_t b, uint32_t c, uint32_t carry) {
    mul_add32(out, out_carry, b, c, a, carry);
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
uint32_t sub(uint32_t* output, const uint32_t* left, const uint32_t* right, int n) {
    uint64_t borrow = 0;
    for(int i=0; i<n; i++){
        borrow = (uint64_t)left[i] - ((uint64_t)right[i] + (borrow >> (BITS-1)));
        output[i] = (uint32_t) borrow;
        borrow = borrow >> BITS;
    }
    return (uint32_t) borrow;
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
uint32_t add6(uint32_t* output, const uint32_t* left, const uint32_t* right) {
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

        "ADC %[carry], %[carry], #0"
        : [carry] "+r" (carry), [t0] "+r" (t0), [t1] "+r" (t1)
        : [o] "r" (output), [l] "r" (left), [r] "r" (right)
        : "memory"
    );
    return carry;
#else
    return add(output, left, right, 6);
#endif
}

/*
void mul6(uint32_t* output, const uint32_t* left, const uint32_t* right) {
    uint32_t t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, carry;

    mul_add32(&t0,  &carry, left[0], right[0],  0,  0);
    mul_add32(&t1,  &carry, left[0], right[1],  0,  carry);
    mul_add32(&t2,  &carry, left[0], right[2],  0,  carry);
    mul_add32(&t3,  &carry, left[0], right[3],  0,  carry);
    mul_add32(&t4,  &carry, left[0], right[4],  0,  carry);
    mul_add32(&t5,  &t6   , left[0], right[5],  0,  carry);
    mul_add32(&t1,  &carry, left[1], right[0],  t1, 0);
    mul_add32(&t2,  &carry, left[1], right[1],  t2, carry);
    mul_add32(&t3,  &carry, left[1], right[2],  t3, carry);
    mul_add32(&t4,  &carry, left[1], right[3],  t4, carry);
    mul_add32(&t5,  &carry, left[1], right[4],  t5, carry);
    mul_add32(&t6,  &t7   , left[1], right[5],  t6, carry);
    mul_add32(&t2,  &carry, left[2], right[0],  t2, 0);
    mul_add32(&t3,  &carry, left[2], right[1],  t3, carry);
    mul_add32(&t4,  &carry, left[2], right[2],  t4, carry);
    mul_add32(&t5,  &carry, left[2], right[3],  t5, carry);
    mul_add32(&t6,  &carry, left[2], right[4],  t6, carry);
    mul_add32(&t7,  &t8   , left[2], right[5],  t7, carry);
    mul_add32(&t3,  &carry, left[3], right[0],  t3, 0);
    mul_add32(&t4,  &carry, left[3], right[1],  t4, carry);
    mul_add32(&t5,  &carry, left[3], right[2],  t5, carry);
    mul_add32(&t6,  &carry, left[3], right[3],  t6, carry);
    mul_add32(&t7,  &carry, left[3], right[4],  t7, carry);
    mul_add32(&t8,  &t9,    left[3], right[5],  t8, carry);
    mul_add32(&t4,  &carry, left[4], right[0],  t4, 0);
    mul_add32(&t5,  &carry, left[4], right[1],  t5, carry);
    mul_add32(&t6,  &carry, left[4], right[2],  t6, carry);
    mul_add32(&t7,  &carry, left[4], right[3],  t7, carry);
    mul_add32(&t8,  &carry, left[4], right[4],  t8, carry);
    mul_add32(&t9,  &t10  , left[4], right[5],  t9, carry);
    mul_add32(&t5,  &carry, left[5], right[0],  t5, 0);
    mul_add32(&t6,  &carry, left[5], right[1],  t6, carry);
    mul_add32(&t7,  &carry, left[5], right[2],  t7, carry);
    mul_add32(&t8,  &carry, left[5], right[3],  t8, carry);
    mul_add32(&t9,  &carry, left[5], right[4],  t9, carry);
    mul_add32(&t10, &t11  , left[5], right[5], t10, carry);

    output[0] = t0;
    output[1] = t1;
    output[2] = t2;
    output[3] = t3;
    output[4] = t4;
    output[5] = t5;
    output[6] = t6;
    output[7] = t7;
    output[8] = t8;
    output[9] = t9;
    output[10] = t10;
    output[11] = t11;
} */

inline
void mul3(uint32_t* output, const uint32_t* left, const uint32_t* right) {
    uint32_t carry, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10;

    m(&output[0], &carry, left[0], right[0]);
    ma(&t1, &carry, carry, left[0], right[1]);
    ma(&t2, &t3, carry, left[0], right[2]);

    ma(&output[1], &carry, t1, left[1], right[0]);
    mac(&t2, &carry, t2, left[1], right[1], carry);
    mac(&t3, &t4, t3, left[1], right[2], carry);

    ma(&output[2], &carry, t2, left[2], right[0]);
    mac(&output[3], &carry, t3, left[2], right[1], carry);
    mac(&output[4], &output[5], t4, left[2], right[2], carry);
}

inline
void mul32_x6_rec(uint32_t* output, const uint32_t* left, const uint32_t* right) {
    const unsigned int n = 6;
    const unsigned int k = 3;
    uint32_t tmp[n + 1];

    memset(output + n, 0, n * sizeof(uint32_t));
    mul3(output, left, right);

    mul3(tmp, left, right + k);
    output[k + n] = add6(output + k, output + k, tmp);

    mul3(tmp, left + k, right);
    output[k + n] += add6(output + k, output + k, tmp);

    mul3(tmp, left + k, right + k);
    add6(output + n, output + n, tmp);
}

void mul32_x6(uint32_t* output, const uint32_t* left, const uint32_t* right) {
    uint32_t carry;

    mul_add32(&output[0],  &carry,     left[0], right[0]);
    mul_add32(&output[1],  &carry,     left[0], right[1], carry);
    mul_add32(&output[2],  &carry,     left[0], right[2], carry);
    mul_add32(&output[3],  &carry,     left[0], right[3], carry);
    mul_add32(&output[4],  &carry,     left[0], right[4], carry);
    mul_add32(&output[5],  &output[6], left[0], right[5], carry);

    for(int i=1; i<6; ++i) {
        mul_add32(&output[i],      &carry,         left[i], right[0], output[i]);
        mul_add32(&output[i + 1],  &carry,         left[i], right[1], output[i + 1], carry);
        mul_add32(&output[i + 2],  &carry,         left[i], right[2], output[i + 2], carry);
        mul_add32(&output[i + 3],  &carry,         left[i], right[3], output[i + 3], carry);
        mul_add32(&output[i + 4],  &carry,         left[i], right[4], output[i + 4], carry);
        mul_add32(&output[i + 5],  &output[i + 6], left[i], right[5], output[i + 5], carry);
    }
}

inline
void mul32_x12_rec(uint32_t* output, const uint32_t* left, const uint32_t* right) {
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

    mul32_x6(output, left, right);

    // add_carry(left_low, left_low, left_high, s2);
    // add_carry(right_low, right_low, right_high, s2);
    // karatsuba(bb, left_low, right_low, s2 + 1);
    // sub(bb, bb, ll, 2*s2);
    // sub(bb, bb, hh, 2*s2);
    // add_carry(output + k, output + k, bb, 2*s2);

    mul32_x6(tmp, left, right + k);
    output[k + n] = add12(output + k, output + k, tmp);

    mul32_x6(tmp, left + k, right);
    output[k + n] += add12(output + k, output + k, tmp);

    mul32_x6(tmp, left + k, right + k);
    add12(output + n, output + n, tmp);
}


void mul32_x12(uint32_t* output, const uint32_t* left, const uint32_t* right) {
    uint32_t carry;

    mul_add32(&output[0],  &carry,      left[0], right[0]);
    mul_add32(&output[1],  &carry,      left[0], right[1],  carry);
    mul_add32(&output[2],  &carry,      left[0], right[2],  carry);
    mul_add32(&output[3],  &carry,      left[0], right[3],  carry);
    mul_add32(&output[4],  &carry,      left[0], right[4],  carry);
    mul_add32(&output[5],  &carry,      left[0], right[5],  carry);
    mul_add32(&output[6],  &carry,      left[0], right[6],  carry);
    mul_add32(&output[7],  &carry,      left[0], right[7],  carry);
    mul_add32(&output[8],  &carry,      left[0], right[8],  carry);
    mul_add32(&output[9],  &carry,      left[0], right[9],  carry);
    mul_add32(&output[10], &carry,      left[0], right[10], carry);
    mul_add32(&output[11], &output[12], left[0], right[11], carry);

    for(int i=1; i<12; ++i) {
        mul_add32(&output[i],      &carry,          left[i], right[0],  output[i]);
        mul_add32(&output[i + 1],  &carry,          left[i], right[1],  output[i + 1], carry);
        mul_add32(&output[i + 2],  &carry,          left[i], right[2],  output[i + 2], carry);
        mul_add32(&output[i + 3],  &carry,          left[i], right[3],  output[i + 3], carry);
        mul_add32(&output[i + 4],  &carry,          left[i], right[4],  output[i + 4], carry);
        mul_add32(&output[i + 5],  &carry,          left[i], right[5],  output[i + 5], carry);
        mul_add32(&output[i + 6],  &carry,          left[i], right[6],  output[i + 6], carry);
        mul_add32(&output[i + 7],  &carry,          left[i], right[7],  output[i + 7], carry);
        mul_add32(&output[i + 8],  &carry,          left[i], right[8],  output[i + 8], carry);
        mul_add32(&output[i + 9],  &carry,          left[i], right[9],  output[i + 9], carry);
        mul_add32(&output[i + 10], &carry,          left[i], right[10], output[i + 10], carry);
        mul_add32(&output[i + 11], &output[i + 12], left[i], right[11], output[i + 11], carry);
    }
}

inline
void add32(uint32_t* out,
           uint32_t* out_carry,
           uint32_t a,
           uint32_t b,
           uint32_t c) {
    uint64_t ret = (uint64_t)a + (uint64_t)b + (uint64_t)c;
    *out = (uint32_t)ret;
    *out_carry = (uint32_t)(ret >> 32);
}

inline
void add32(uint32_t* out,
           uint32_t* out_carry,
           uint32_t a,
           uint32_t b,
           uint32_t c,
           uint32_t d) {
    uint64_t ret = (uint64_t)a + (uint64_t)b + (uint64_t)c + (uint64_t)d;
    *out = (uint32_t)ret;
    *out_carry = (uint32_t)(ret >> 32);
}


inline
void mul_add64(uint64_t* out,
               uint64_t* out_carry,
               uint64_t a,
               uint64_t b,
               uint64_t c,
               uint64_t d) {
#if __arm__
    uint32_t a0 = (uint32_t) a;
    uint32_t a1 = (uint32_t) (a >> 32);
    uint32_t b0 = (uint32_t) b;
    uint32_t b1 = (uint32_t) (b >> 32);
    uint32_t o0;
    uint32_t o1;
    uint32_t o2 = 0;
    uint32_t o3 = 0;

    asm volatile (
        "UMULL %[o0], %[o1], %[a0], %[b0]\n\t"
        "UMLAL %[o1], %[o2], %[a0], %[b1]\n\t"
        "UMLAL %[o2], %[o3], %[a1], %[b1]"
        : [o0] "+r" (o0),
          [o1] "+r" (o1),
          [o2] "+r" (o2),
          [o3] "+r" (o3)
        : [a0] "r" (a0),
          [a1] "r" (a1),
          [b0] "r" (b0),
          [b1] "r" (b1)
    );

    uint32_t t1;
    uint32_t t2;
    asm volatile (
        "UMULL %[t1], %[t2], %[a1], %[b0]\n\t"
        "ADDS %[o1], %[o1], %[t1]\n\t"
        "ADCS %[o2], %[o2], %[t2]\n\t"
        "ADC  %[o3], %[o3], #0\n\t"
        : [o0] "+r" (o0),
          [o1] "+r" (o1),
          [o2] "+r" (o2),
          [o3] "+r" (o3),
          [t1] "+r" (t1),
          [t2] "+r" (t2)
        : [a0] "r" (a0),
          [a1] "r" (a1),
          [b0] "r" (b0),
          [b1] "r" (b1)
    );
        
    uint32_t c0 = (uint32_t) c;
    uint32_t c1 = (uint32_t) (c >> 32);
    asm volatile (
        "ADDS %[o0], %[o0], %[c0]\n\t"
        "ADCS %[o1], %[o1], %[c1]\n\t"
        "ADCS %[o2], %[o2], #0\n\t"
        "ADC  %[o3], %[o3], #0\n\t"
        : [o0] "+r" (o0),
          [o1] "+r" (o1),
          [o2] "+r" (o2),
          [o3] "+r" (o3)
        : [c0] "r" (c0),
          [c1] "r" (c1)
    );

    uint32_t d0 = (uint32_t) d;
    uint32_t d1 = (uint32_t) (d >> 32);
    asm volatile (
        "ADDS %[o0], %[o0], %[d0]\n\t"
        "ADCS %[o1], %[o1], %[d1]\n\t"
        "ADCS %[o2], %[o2], #0\n\t"
        "ADC  %[o3], %[o3], #0\n\t"
        : [o0] "+r" (o0),
          [o1] "+r" (o1),
          [o2] "+r" (o2),
          [o3] "+r" (o3)
        : [d0] "r" (d0),
          [d1] "r" (d1)
    );

    *out = ((uint64_t)o1 << 32) + (uint64_t)o0;
    *out_carry = ((uint64_t)o3 << 32) + (uint64_t)o2;
#else
    uint128_t ret = ((uint128_t)a * (uint128_t)b) + (uint128_t)c + (uint128_t)d;
    *out = (uint64_t)ret;
    *out_carry = (uint64_t)(ret >> 64);
#endif
}

inline
void add64(uint64_t* out,
           uint64_t* out_carry,
           uint64_t a,
           uint64_t b,
           uint64_t c) {
#if __arm__
    uint32_t a0 = (uint32_t) a;
    uint32_t a1 = (uint32_t) (a >> 32);
    uint32_t b0 = (uint32_t) b;
    uint32_t b1 = (uint32_t) (b >> 32);
    uint32_t o0;
    uint32_t o1;
    uint32_t o2 = 0;

    asm volatile (
        "ADDS %[o0], %[a0], %[b0]\n\t"
        "ADCS %[o1], %[a1], %[b1]\n\t"
        "ADC  %[o2], %[o2], #0\n\t"
        : [o0] "+r" (o0),
          [o1] "+r" (o1),
          [o2] "+r" (o2)
        : [a0] "r" (a0),
          [a1] "r" (a1),
          [b0] "r" (b0),
          [b1] "r" (b1)
    );
 
    uint32_t c0 = (uint32_t) c;
    uint32_t c1 = (uint32_t) (c >> 32);
    asm volatile (
        "ADDS %[o0], %[o0], %[c0]\n\t"
        "ADCS %[o1], %[o1], %[c1]\n\t"
        "ADC  %[o2], %[o2], #0\n\t"
        : [o0] "+r" (o0),
          [o1] "+r" (o1),
          [o2] "+r" (o2)
        : [c0] "r" (c0),
          [c1] "r" (c1)
    );

    *out = ((uint64_t)o1 << 32) + (uint64_t)o0;
    *out_carry = o2;
#else
    uint32_t a0 = (uint32_t) a;
    uint32_t a1 = (uint32_t) (a >> 32);
    uint32_t b0 = (uint32_t) b;
    uint32_t b1 = (uint32_t) (b >> 32);
    uint32_t c0 = (uint32_t) c;
    uint32_t c1 = (uint32_t) (c >> 32);
    uint32_t o0;
    uint32_t o1;
    uint32_t o2;

    add32(&o0, &o1, a0, b0, c0);
    add32(&o1, &o2, o1, a1, b1, c1);

    *out = ((uint64_t)o1 << 32) + (uint64_t)o0;
    *out_carry = o2;
#endif
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

inline
void montgomery_step(uint64_t* r, const uint64_t* t, uint64_t a)
{
    uint64_t k, carry, _;
    k = t[0] * inv;
    mul_add64(&_,  &carry, k, modulus[0], t[0], 0);
    mul_add64(&r[1], &carry, k, modulus[1], t[1], carry);
    mul_add64(&r[2], &carry, k, modulus[2], t[2], carry);
    mul_add64(&r[3], &carry, k, modulus[3], t[3], carry);
    mul_add64(&r[4], &carry, k, modulus[4], t[4], carry);
    mul_add64(&r[5], &carry, k, modulus[5], t[5], carry);
    add64(&r[6], &r[7], a, t[6], carry);
}

void montgomery_reduce(uint64_t* output, const uint64_t* t) {
    uint64_t r[12];
    montgomery_step(r, t, 0);
    montgomery_step(r + 1, r + 1, t[7]);
    montgomery_step(r + 2, r + 2, t[8]);
    montgomery_step(r + 3, r + 3, t[9]);
    montgomery_step(r + 4, r + 4, t[10]);
    montgomery_step(output - 1, r + 5, t[11]);
}

extern "C" void c_mul(uint64_t* output, const uint64_t* left, const uint64_t* right) {
    uint64_t tmp[12];

    // cast down to 32 bit mode, which is ~30% faster than on arm32
    mul32_x12((uint32_t*)tmp, (const uint32_t*)left, (const uint32_t*)right);

    montgomery_reduce(output, tmp);
}
