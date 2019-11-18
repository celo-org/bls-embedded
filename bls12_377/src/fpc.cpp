#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define uint128_t __uint128_t

// #define restrict __restrict__
#define restrict



/*
typedef union { 
    uint64_t as64;
    struct
    {
        uint32_t lo;
        uint32_t hi;
    };
} u64;

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
*/

inline
void umaal96(uint32_t& restrict o0,
             uint32_t& restrict o1,
             uint32_t& restrict o2,
             uint32_t a,
             uint64_t b) {
// multiply a32 * b64 and accumulate in the 96 bit value stored in [o0, o1, o2]
// o0 is the existing 32 bit value
// o1,o2 is the 64 bit carry
#if __arm__
    const uint32_t b0 = (uint32_t) b;
    asm (
        "UMAAL %[o0], %[o1], %[a0], %[b0]"
        : [o0] "+r" (o0),
          [o1] "+r" (o1)
        : [a0] "r" (a),
          [b0] "r" (b0)
    );

    const uint32_t b1 = (uint32_t) (b>>32);
    asm (
        "UMAAL %[o1], %[o2], %[a0], %[b1]"
        : [o1] "+r" (o1),
          [o2] "+r" (o2)
        : [a0] "r" (a),
          [b1] "r" (b1)
    );
#else
    uint128_t ret = ((uint128_t)a * (uint128_t)b) + (uint128_t)o0 + (uint128_t)o1 +  + (((uint128_t)o2)<<32);
    o0 = (uint32_t)ret;
    o1 = (uint32_t)(ret >> 32);
    o2 = (uint32_t)(ret >> 64);
#endif
}

inline
void umaal96(uint32_t& restrict o0,
             uint32_t& restrict o1,
             uint32_t& restrict o2,
             uint32_t a,
             uint64_t b, 
             uint32_t c)
{
    o0 = c;
    umaal96(o0, o1, o2, a, b);
}


inline
void umaal96(uint32_t& restrict o0,
             uint32_t& restrict o1,
             uint32_t& restrict o2,
             uint32_t a,
             uint64_t b, 
             uint32_t c,
             uint32_t d)
{
    o0 = c;
    o1 = d;
    o2 = 0;
    umaal96(o0, o1, o2, a, b);
}

inline
void umull96(uint32_t& restrict o0,
             uint32_t& restrict o1,
             uint32_t& restrict o2,
             uint32_t a,
             uint64_t b) {
// multiply a32 * b64 and store in [o0, o1, o2]
#if __arm__
    const uint32_t b0 = (uint32_t) b;
    asm (
        "UMULL %[o0], %[o1], %[a0], %[b0]"
        : [o0] "+r" (o0),
          [o1] "+r" (o1)
        : [a0] "r" (a),
          [b0] "r" (b0)
    );

    o2=0;
    const uint32_t b1 = (uint32_t) (b>>32);
    asm (
        "UMAAL %[o1], %[o2], %[a0], %[b1]"
        : [o1] "+r" (o1),
          [o2] "+r" (o2)
        : [a0] "r" (a),
          [b1] "r" (b1)
    );
#else
    o0 = 0;
    o1 = 0;
    o2 = 0;
    umaal96(o0, o1, o2, a, b);
#endif
}


inline
void umlal96(uint32_t& restrict o0,
             uint32_t& restrict o1,
             uint32_t& restrict o2,
             uint32_t a,
             uint64_t b) {
// multiply a32 * b64 + o0 and store in [o0, o1, o2]
#if __arm__
    const uint32_t b0 = (uint32_t) b;
    
    o1 = 0;
    asm (
        "UMAAL %[o0], %[o1], %[a0], %[b0]"
        : [o0] "+r" (o0),
          [o1] "+r" (o1)
        : [a0] "r" (a),
          [b0] "r" (b0)
    );

    o2 = 0;
    const uint32_t b1 = (uint32_t) (b>>32);
    asm (
        "UMAAL %[o1], %[o2], %[a0], %[b1]"
        : [o1] "+r" (o1),
          [o2] "+r" (o2)
        : [a0] "r" (a),
          [b1] "r" (b1)
    );
#else
    o1 = 0;
    o2 = 0;
    umaal96(o0, o1, o2, a, b);
#endif
}

/*
inline
void mul_add64(uint64_t* restrict out,
               uint64_t* restrict out_carry,
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
        "UMAAL %[o1], %[o2], %[a0], %[b1]\n\t"
        "UMAAL %[o2], %[o3], %[a1], %[b1]"
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
*/

#if __arm__
// not needed
#else

inline
uint64_t add32(uint32_t* output, const uint32_t* left, const uint32_t* right, int n) {
    uint64_t carry = 0;
    for(int i=0; i<n; i++){
        carry += (uint64_t)left[i] + (uint64_t)right[i];
        output[i] = (uint32_t) carry;
        carry = carry >> 32;
    }
    return carry;
}

inline
uint64_t add32(uint32_t* output, const uint32_t* a, const uint32_t* b, const uint32_t* c, int n) {
    uint64_t carry = 0;
    for(int i=0; i<n; i++){
        carry += (uint64_t)a[i] + (uint64_t)b[i] + (uint64_t)c[i];
        output[i] = (uint32_t) carry;
        carry = carry >> 32;
    }
    return carry;
}

#endif

/*
inline
uint32_t sub32(uint32_t* output, const uint32_t* left, const uint32_t* right, int n) {
    uint64_t borrow = 0;
    for(int i=0; i<n; i++){
        borrow = (uint64_t)left[i] - ((uint64_t)right[i] + (borrow >> 31));
        output[i] = (uint32_t) borrow;
        borrow = borrow >> 32;
    }
    return (uint32_t) borrow;
}
*/

inline
uint32_t acc_2_2_1(uint32_t* restrict output, const uint32_t* restrict b, uint32_t c0) {
#if __arm__
    uint32_t carry = 0;
    uint32_t t0 = output[0];
    uint32_t t1 = output[1];
    uint32_t b0 = b[0];
    uint32_t b1 = b[1];
    asm (
        "ADDS %[t0], %[b0]\n\t"
        "ADCS %[t1], %[b1]\n\t"
        "ADC %[carry], #0"
        : [carry] "+r" (carry),
          [t0] "+r" (t0),
          [t1] "+r" (t1)
        : [b0] "r" (b0),
          [b1] "r" (b1)
    );
    asm (
        "ADDS %[t0], %[c0]\n\t"
        "ADCS %[t1], #0\n\t"
        "ADC %[carry], #0"
        : [carry] "+r" (carry),
          [t0] "+r" (t0),
          [t1] "+r" (t1)
        : [c0] "r" (c0)
    );
    output[0] = t0;
    output[1] = t1;
    return carry;
#else
    uint32_t _a[] = {output[0], output[1]};
    uint32_t _b[] = {b[0], b[1]};
    uint32_t _c[] = {c0, 0};
    return add32(output, _a, _b, _c, 2);
#endif
}

inline
void add_2_2_1(uint32_t* restrict output, uint32_t* restrict a, const uint32_t* restrict b, uint32_t c0) {
#if __arm__
    uint32_t t0 = 0;
    uint32_t t1 = 0;
    uint32_t a0 = a[0];
    uint32_t a1 = a[1];
    uint32_t b0 = b[0];
    uint32_t b1 = b[1];
    asm (
        "ADDS %[t0], %[a0], %[b0]\n\t"
        "ADC %[t1], %[a1], %[b1]\n\t"
        : [t0] "+r" (t0),
          [t1] "+r" (t1)
        : [a0] "r" (a0),
          [a1] "r" (a1),
          [b0] "r" (b0),
          [b1] "r" (b1)
    );
    asm (
        "ADDS %[t0], %[c0]\n\t"
        "ADC  %[t1], #0 \n\t"
        : [t0] "+r" (t0),
          [t1] "+r" (t1)
        : [c0] "r" (c0)
    );
    output[0] = t0;
    output[1] = t1;
#else
    uint32_t _a[] = {a[0], a[1]};
    uint32_t _b[] = {b[0], b[1]};
    uint32_t _c[] = {c0, 0};
    add32(output, _a, _b, _c, 2);
#endif
}

/*
inline
void mul32x12(uint32_t* output, const uint32_t* left, const uint32_t* right) {
    uint32_t carry;

    mul_add32(&output[0],  &carry,      left[0], right[0],  0, 0);
    mul_add32(&output[1],  &carry,      left[0], right[1],  0, carry);
    mul_add32(&output[2],  &carry,      left[0], right[2],  0, carry);
    mul_add32(&output[3],  &carry,      left[0], right[3],  0, carry);
    mul_add32(&output[4],  &carry,      left[0], right[4],  0, carry);
    mul_add32(&output[5],  &carry,      left[0], right[5],  0, carry);
    mul_add32(&output[6],  &carry,      left[0], right[6],  0, carry);
    mul_add32(&output[7],  &carry,      left[0], right[7],  0, carry);
    mul_add32(&output[8],  &carry,      left[0], right[8],  0, carry);
    mul_add32(&output[9],  &carry,      left[0], right[9],  0, carry);
    mul_add32(&output[10], &carry,      left[0], right[10], 0, carry);
    mul_add32(&output[11], &output[12], left[0], right[11], 0, carry);

    for(int i=1; i<12; ++i) {
        mul_add32(&output[i],      &carry,          left[i], right[0],  output[i],     0);
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
void mul64x6(uint64_t* output, const uint64_t* left, const uint64_t* right) {
    uint64_t carry;

    mul_add64(&output[0], &carry,     left[0], right[0], 0, 0);
    mul_add64(&output[1], &carry,     left[0], right[1], 0, carry);
    mul_add64(&output[2], &carry,     left[0], right[2], 0, carry);
    mul_add64(&output[3], &carry,     left[0], right[3], 0, carry);
    mul_add64(&output[4], &carry,     left[0], right[4], 0, carry);
    mul_add64(&output[5], &output[6], left[0], right[5], 0, carry);

    for(int i=1; i<6; ++i) {
        mul_add64(&output[i],      &carry,         left[i], right[0], output[i], 0);
        mul_add64(&output[i + 1],  &carry,         left[i], right[1], output[i + 1], carry);
        mul_add64(&output[i + 2],  &carry,         left[i], right[2], output[i + 2], carry);
        mul_add64(&output[i + 3],  &carry,         left[i], right[3], output[i + 3], carry);
        mul_add64(&output[i + 4],  &carry,         left[i], right[4], output[i + 4], carry);
        mul_add64(&output[i + 5],  &output[i + 6], left[i], right[5], output[i + 5], carry);
    }
}
*/

inline
void mul_hybrid(uint32_t* __restrict__ output, const uint64_t* __restrict__ left, const uint32_t* __restrict__ right) {
    uint32_t carry0;
    uint32_t carry1;
    uint64_t val = left[0];

    umull96(output[0],  carry0, carry1, right[0],  val);
    umaal96(output[1],  carry0, carry1, right[1],  val, 0);
    umaal96(output[2],  carry0, carry1, right[2],  val, 0);
    umaal96(output[3],  carry0, carry1, right[3],  val, 0);
    umaal96(output[4],  carry0, carry1, right[4],  val, 0);
    umaal96(output[5],  carry0, carry1, right[5],  val, 0);
    umaal96(output[6],  carry0, carry1, right[6],  val, 0);
    umaal96(output[7],  carry0, carry1, right[7],  val, 0);
    umaal96(output[8],  carry0, carry1, right[8],  val, 0);
    umaal96(output[9],  carry0, carry1, right[9],  val, 0);
    umaal96(output[10], carry0, carry1, right[10], val, 0);
    umaal96(output[11], carry0, carry1, right[11], val, 0);
    output[12] = carry0;
    output[13] = carry1;

    for(int i=2; i<12; i+=2) {
        val = left[i/2];
        umlal96(output[i+0],  carry0, carry1, right[0],  val);
        umaal96(output[i+1],  carry0, carry1, right[1],  val);
        umaal96(output[i+2],  carry0, carry1, right[2],  val);
        umaal96(output[i+3],  carry0, carry1, right[3],  val);
        umaal96(output[i+4],  carry0, carry1, right[4],  val);
        umaal96(output[i+5],  carry0, carry1, right[5],  val);
        umaal96(output[i+6],  carry0, carry1, right[6],  val);
        umaal96(output[i+7],  carry0, carry1, right[7],  val);
        umaal96(output[i+8],  carry0, carry1, right[8],  val);
        umaal96(output[i+9],  carry0, carry1, right[9],  val);
        umaal96(output[i+10], carry0, carry1, right[10], val);
        umaal96(output[i+11], carry0, carry1, right[11], val);
        output[i+12] = carry0;
        output[i+13] = carry1;
    }
}

void montgomery_reduce(uint32_t* restrict output, uint32_t* t) {
    const static uint64_t inv = 9586122913090633727ull;
    const static uint64_t modulus32[12] = {
         0x00000001, 0x8508c000,
         0x30000000, 0x170b5d44,
         0xba094800, 0x1ef3622f,
         0x00f5138f, 0x1a22d9f3,
         0x6ca1493b, 0xc63b05c0,
         0x17c510ea, 0x01ae3a46,
    };
    uint32_t altcarry = 0;

    for(int i=0; i<5; ++i){
        uint32_t* r = t + 2*i;
        uint64_t k = *(uint64_t*)r * inv;
        uint32_t carry[2] = {0};
        uint32_t _;

        umaal96(_,     carry[0], carry[1], modulus32[0],  k, r[0]);
        umaal96(_,     carry[0], carry[1], modulus32[1],  k, r[1]);
        umaal96(r[2],  carry[0], carry[1], modulus32[2],  k);
        umaal96(r[3],  carry[0], carry[1], modulus32[3],  k);
        umaal96(r[4],  carry[0], carry[1], modulus32[4],  k);
        umaal96(r[5],  carry[0], carry[1], modulus32[5],  k);
        umaal96(r[6],  carry[0], carry[1], modulus32[6],  k);
        umaal96(r[7],  carry[0], carry[1], modulus32[7],  k);
        umaal96(r[8],  carry[0], carry[1], modulus32[8],  k);
        umaal96(r[9],  carry[0], carry[1], modulus32[9],  k);
        umaal96(r[10], carry[0], carry[1], modulus32[10], k);
        umaal96(r[11], carry[0], carry[1], modulus32[11], k);
        altcarry = acc_2_2_1(&r[12], carry, altcarry);
    }

    {
        uint32_t* r = t + 10;
        uint64_t k = *(uint64_t*)r * inv;
        uint32_t carry[2] = {0};
        uint32_t _;

        umaal96(_,         carry[0], carry[1], modulus32[0],  k, r[0]);
        umaal96(_,         carry[0], carry[1], modulus32[1],  k, r[1]);
        umaal96(output[0], carry[0], carry[1], modulus32[2],  k, r[2]);
        umaal96(output[1], carry[0], carry[1], modulus32[3],  k, r[3]);
        umaal96(output[2], carry[0], carry[1], modulus32[4],  k, r[4]);
        umaal96(output[3], carry[0], carry[1], modulus32[5],  k, r[5]);
        umaal96(output[4], carry[0], carry[1], modulus32[6],  k, r[6]);
        umaal96(output[5], carry[0], carry[1], modulus32[7],  k, r[7]);
        umaal96(output[6], carry[0], carry[1], modulus32[8],  k, r[8]);
        umaal96(output[7], carry[0], carry[1], modulus32[9],  k, r[9]);
        umaal96(output[8], carry[0], carry[1], modulus32[10], k, r[10]);
        umaal96(output[9], carry[0], carry[1], modulus32[11], k, r[11]);
        add_2_2_1(&output[10], &r[12], carry, altcarry);
    }
}

extern "C" void c_mul(uint64_t* restrict output, const uint64_t* restrict left, const uint64_t* restrict right) {
    mul_hybrid((uint32_t*)output, left, (const uint32_t*)right);
}

extern "C" void c_montgomry(uint64_t* restrict output, uint64_t* restrict tmp) {
    montgomery_reduce((uint32_t*)output, (uint32_t*)tmp);
}

/*
extern "C" void c_muladdadd(uint64_t* restrict out,
                            uint64_t a,
                            uint64_t b,
                            uint64_t c,
                            uint64_t d)
{
    mul_add64(out, out + 1, a, b, c, d);
}
*/
