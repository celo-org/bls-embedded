#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define MAX 12
#define BITS 32
//#define ARM

inline
void m(uint32_t* out, uint32_t* out_carry, uint32_t b, uint32_t c) {
#ifdef ARM
    uint32_t RdLo, RdHi;
    asm (
        "UMULL %[RdLo], %[RdHi], %[Rn], %[Rm]"
        : [RdLo] "=r" (RdLo), [RdHi] "=r" (RdHi)
        : [Rn] "r" (b), [Rm] "r" (c)
    );
    *out = RdLo;
    *out_carry = RdHi;
#else
    uint64_t ret = (uint64_t) b * (uint64_t) c;
    *out = (uint32_t) ret;
    *out_carry = (uint32_t)(ret >> BITS);
#endif
}

inline
void ma(uint32_t* out, uint32_t* out_carry, uint32_t a, uint32_t b, uint32_t c) {
#ifdef ARM
    uint32_t RdLo=a, RdHi=0;
    asm (
        "UMLAL %[RdLo], %[RdHi], %[Rn], %[Rm]"
        : [RdLo] "+r" (RdLo), [RdHi] "+r" (RdHi)
        : [Rn] "r" (b), [Rm] "r" (c)
    );
    *out = RdLo;
    *out_carry = RdHi;
#else
   uint64_t ret = (uint64_t) a + ((uint64_t) b * (uint64_t) c);
   *out = (uint32_t) ret;
   *out_carry = (uint32_t)(ret >> BITS);
#endif
}

inline
void mac(uint32_t* out, uint32_t* out_carry, uint32_t a, uint32_t b, uint32_t c, uint32_t carry) {
#ifdef ARM
    uint64_t t = (uint64_t)a + (uint64_t)carry;
    uint32_t RdLo=(uint32_t)t, RdHi=(uint32_t)(t>>BITS);
    asm (
        "UMLAL %[RdLo], %[RdHi], %[Rn], %[Rm];"
        : [RdLo] "+r" (RdLo), [RdHi] "+r" (RdHi)
        : [Rn] "r" (b), [Rm] "r" (c)
    );
    *out = RdLo;
    *out_carry = RdHi;
#else
    uint64_t ret = (uint64_t) a + ((uint64_t) b * (uint64_t) c) + (uint64_t) carry;
    *out = (uint32_t) ret;
    *out_carry = (uint32_t)(ret >> BITS);
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
void add_carry(uint32_t* output, const uint32_t* left, const uint32_t* right, int n) {
    output[n] = add(output, left, right, n);
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
    add_carry(output + k, output + k, tmp, n);

    tmp[n] = 0;
    mul6(tmp, left + k, right);
    add(output + k, output + k, tmp, n + 1);

    mul6(tmp, left + k, right + k);
    add(output + n, output + n, tmp, n);
}

extern "C" void c_mul(uint32_t* output, const uint32_t* left, const uint32_t* right) {
    mul12(output, left, right);
}