#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define uint128_t __uint128_t
#define MAX 12
#define BITS 32

static const uint64_t inv = 9586122913090633727ull;
static const uint64_t modulus[6] = {
    0x8508c00000000001,
    0x170b5d4430000000,
    0x1ef3622fba094800,
    0x1a22d9f300f5138f,
    0xc63b05c06ca1493b,
    0x1ae3a4617c510ea,
};

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
void umaal96(uint32_t& o0,
             uint32_t& o1,
             uint32_t& o2,
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
void umaal96(uint32_t& o0,
             uint32_t& o1,
             uint32_t& o2,
             uint32_t a,
             uint64_t b, 
             uint32_t c)
{
    o0 = c;
    umaal96(o0, o1, o2, a, b);
}


inline
void umaal96(uint32_t& o0,
             uint32_t& o1,
             uint32_t& o2,
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
void umull96(uint32_t& o0,
             uint32_t& o1,
             uint32_t& o2,
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
        "UMLAL %[o1], %[o2], %[a0], %[b1]"
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
void umlal96(uint32_t& o0,
             uint32_t& o1,
             uint32_t& o2,
             uint32_t a,
             uint64_t b) {
// multiply a32 * b64 + o0 and store in [o0, o1, o2]
#if __arm__
    const uint32_t b0 = (uint32_t) b;
    
    o1 = 0;
    asm (
        "UMLAL %[o0], %[o1], %[a0], %[b0]"
        : [o0] "+r" (o0),
          [o1] "+r" (o1)
        : [a0] "r" (a),
          [b0] "r" (b0)
    );

    o2 = 0;
    const uint32_t b1 = (uint32_t) (b>>32);
    asm (
        "UMLAL %[o1], %[o2], %[a0], %[b1]"
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
void mul_add64(uint64_t* out,
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

    *out = ((uint64_t)o1 << 32) + (uint64_t)o0;
    *out_carry = ((uint64_t)o3 << 32) + (uint64_t)o2;
#else
    uint128_t ret = ((uint128_t)a * (uint128_t)b) + (uint128_t)c;
    *out = (uint64_t)ret;
    *out_carry = (uint64_t)(ret >> 64);
#endif
}


inline
void mul_add64(uint64_t* out,
               uint64_t* out_carry,
               uint64_t a,
               uint64_t b) {
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

    *out = ((uint64_t)o1 << 32) + (uint64_t)o0;
    *out_carry = ((uint64_t)o3 << 32) + (uint64_t)o2;
#else
    uint128_t ret = ((uint128_t)a * (uint128_t)b);
    *out = (uint64_t)ret;
    *out_carry = (uint64_t)(ret >> 64);
#endif
}

inline
uint64_t add32(uint32_t* output, const uint32_t* left, const uint32_t* right, int n) {
    uint64_t carry = 0;
    for(int i=0; i<n; i++){
        carry += (uint64_t)left[i] + (uint64_t)right[i];
        output[i] = (uint32_t) carry;
        carry = carry >> BITS;
    }
    return carry;
}

inline
uint32_t sub32(uint32_t* output, const uint32_t* left, const uint32_t* right, int n) {
    uint64_t borrow = 0;
    for(int i=0; i<n; i++){
        borrow = (uint64_t)left[i] - ((uint64_t)right[i] + (borrow >> (BITS-1)));
        output[i] = (uint32_t) borrow;
        borrow = borrow >> BITS;
    }
    return (uint32_t) borrow;
}


inline
uint32_t add32x2(uint32_t* output, const uint32_t* left, const uint32_t* right) {
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

        "ADC %[carry], %[carry], #0"
        : [carry] "+r" (carry), [t0] "+r" (t0), [t1] "+r" (t1)
        : [o] "r" (output), [l] "r" (left), [r] "r" (right)
        : "memory"
    );
    return carry;
#else
    return add32(output, left, right, 2);
#endif
}


inline
void mul32x12(uint32_t* output, const uint32_t* left, const uint32_t* right) {
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
void mul64x6(uint64_t* output, const uint64_t* left, const uint64_t* right) {
    uint64_t carry;

    mul_add64(&output[0], &carry,     left[0], right[0]);
    mul_add64(&output[1], &carry,     left[0], right[1], carry);
    mul_add64(&output[2], &carry,     left[0], right[2], carry);
    mul_add64(&output[3], &carry,     left[0], right[3], carry);
    mul_add64(&output[4], &carry,     left[0], right[4], carry);
    mul_add64(&output[5], &output[6], left[0], right[5], carry);

    for(int i=1; i<6; ++i) {
        mul_add64(&output[i],      &carry,         left[i], right[0], output[i]);
        mul_add64(&output[i + 1],  &carry,         left[i], right[1], output[i + 1], carry);
        mul_add64(&output[i + 2],  &carry,         left[i], right[2], output[i + 2], carry);
        mul_add64(&output[i + 3],  &carry,         left[i], right[3], output[i + 3], carry);
        mul_add64(&output[i + 4],  &carry,         left[i], right[4], output[i + 4], carry);
        mul_add64(&output[i + 5],  &output[i + 6], left[i], right[5], output[i + 5], carry);
    }
}

inline
void mul_hybrid(uint32_t* output, const uint64_t* left, const uint32_t* right) {
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

inline
void montgomery_step_0(uint64_t* r, const uint64_t* t, uint64_t a)
{
    uint64_t k = t[0] * inv;
    uint32_t carry[2] = {0};
    uint32_t _;

    uint32_t* r32 = (uint32_t*)r;
    const uint32_t* t32 = (const uint32_t*)t;
    const uint32_t* m32 = (const uint32_t*)modulus;

    umaal96(_,       carry[0], carry[1], m32[0],  k, t32[0]);
    umaal96(_,       carry[0], carry[1], m32[1],  k, t32[1]);
    umaal96(r32[2],  carry[0], carry[1], m32[2],  k, t32[2]);
    umaal96(r32[3],  carry[0], carry[1], m32[3],  k, t32[3]);
    umaal96(r32[4],  carry[0], carry[1], m32[4],  k, t32[4]);
    umaal96(r32[5],  carry[0], carry[1], m32[5],  k, t32[5]);
    umaal96(r32[6],  carry[0], carry[1], m32[6],  k, t32[6]);
    umaal96(r32[7],  carry[0], carry[1], m32[7],  k, t32[7]);
    umaal96(r32[8],  carry[0], carry[1], m32[8],  k, t32[8]);
    umaal96(r32[9],  carry[0], carry[1], m32[9],  k, t32[9]);
    umaal96(r32[10], carry[0], carry[1], m32[10], k, t32[10]);
    umaal96(r32[11], carry[0], carry[1], m32[11], k, t32[11]);

    r32[14] = add32x2(&r32[12], &t32[12], (const uint32_t*)&a) +
              add32x2(&r32[12], &r32[12], carry);
    r32[15] = 0;
}

inline
void montgomery_step_n(uint64_t* r, uint64_t a)
{
    uint64_t k = r[0] * inv;
    uint32_t carry[2];

    uint32_t* r32 = (uint32_t*)r;
    const uint32_t* m32 = (const uint32_t*)modulus;

    umlal96(r32[0],  carry[0], carry[1], m32[0],  k);
    umaal96(r32[1],  carry[0], carry[1], m32[1],  k);
    umaal96(r32[2],  carry[0], carry[1], m32[2],  k);
    umaal96(r32[3],  carry[0], carry[1], m32[3],  k);
    umaal96(r32[4],  carry[0], carry[1], m32[4],  k);
    umaal96(r32[5],  carry[0], carry[1], m32[5],  k);
    umaal96(r32[6],  carry[0], carry[1], m32[6],  k);
    umaal96(r32[7],  carry[0], carry[1], m32[7],  k);
    umaal96(r32[8],  carry[0], carry[1], m32[8],  k);
    umaal96(r32[9],  carry[0], carry[1], m32[9],  k);
    umaal96(r32[10], carry[0], carry[1], m32[10], k);
    umaal96(r32[11], carry[0], carry[1], m32[11], k);

    r32[14] = add32x2(&r32[12], &r32[12], (const uint32_t*)&a) +
              add32x2(&r32[12], &r32[12], carry);
    r32[15] = 0;
}


void montgomery_reduce(uint64_t* output, const uint64_t* t) {
    uint64_t r[12];
    montgomery_step_0(r, t, 0);
    montgomery_step_n(r + 1, t[7]);
    montgomery_step_n(r + 2, t[8]);
    montgomery_step_n(r + 3, t[9]);
    montgomery_step_n(r + 4, t[10]);
    montgomery_step_0(output - 1, r + 5, t[11]);
}

extern "C" void c_mul(uint64_t* output, const uint64_t* left, const uint64_t* right) {
    uint64_t tmp[12];
    //32 bit mode is much faster on arm32
    mul_hybrid((uint32_t*)tmp, left, (const uint32_t*)right);
    montgomery_reduce(output, tmp);
}

extern "C" void c_montgomry(uint64_t* output, uint64_t* tmp) {
    montgomery_reduce(output, tmp);
}

extern "C" void c_muladdadd(uint64_t* out,
                              uint64_t a,
                              uint64_t b,
                              uint64_t c,
                              uint64_t d
                              )
{
    mul_add64(out, out + 1, a, b, c, d);
}