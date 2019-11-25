#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define uint128_t __uint128_t

#define restrict

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
    const uint32_t b1 = (uint32_t) (b>>32);
    #ifdef HAVE_UMAAL
        asm (
            "UMAAL %[o0], %[o1], %[a0], %[b0]"
            : [o0] "+r" (o0),
            [o1] "+r" (o1)
            : [a0] "r" (a),
            [b0] "r" (b0)
        );

        asm (
            "UMAAL %[o1], %[o2], %[a0], %[b1]"
            : [o1] "+r" (o1),
            [o2] "+r" (o2)
            : [a0] "r" (a),
            [b1] "r" (b1)
        );
    #else
        asm (
            "ADDS %[o0], %[o1]\n\t"
            "MOV  %[o1], #0\n\t"
            "UMLAL %[o0], %[o1], %[a0], %[b0]\n\t"
            "ADCS %[o1], %[o2]\n\t"
            "MOV  %[o2], #0\n\t"
            "ADC  %[o2], #0\n\t"
            "UMLAL %[o1], %[o2], %[a0], %[b1]\n\t"
            : [o0] "+r" (o0),
              [o1] "+r" (o1),
              [o2] "+r" (o2)
            : [a0] "r" (a),
              [b0] "r" (b0),
              [b1] "r" (b1)
        );
    #endif
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
    o0 = 0;
    o1 = 0;
    o2 = 0;
    umaal96(o0, o1, o2, a, b);
}


inline
void umlal96(uint32_t& restrict o0,
             uint32_t& restrict o1,
             uint32_t& restrict o2,
             uint32_t a,
             uint64_t b) {
    o1 = 0;
    o2 = 0;
    umaal96(o0, o1, o2, a, b);
}

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

inline
void mul_hybrid(uint32_t* restrict output, const uint64_t* restrict left, const uint32_t* restrict right) {
    register uint32_t carry0;
    register uint32_t carry1;
    uint32_t o0;
    uint32_t o1;
    uint32_t o2;
    uint32_t o3;
    uint32_t o4;
    uint32_t o5;
    uint32_t o6;
    uint32_t o7;
    uint32_t o8;
    uint32_t o9;
    uint32_t o10;
    uint32_t o11;

    register uint64_t val = left[0];
    umull96(o0,  carry0, carry1, right[0],  val);
    output[0] = o0;
    umaal96(o1,  carry0, carry1, right[1],  val, 0);
    output[1] = o1;
    umaal96(o2,  carry0, carry1, right[2],  val, 0);
    umaal96(o3,  carry0, carry1, right[3],  val, 0);
    umaal96(o4,  carry0, carry1, right[4],  val, 0);
    umaal96(o5,  carry0, carry1, right[5],  val, 0);
    umaal96(o6,  carry0, carry1, right[6],  val, 0);
    umaal96(o7,  carry0, carry1, right[7],  val, 0);
    umaal96(o8,  carry0, carry1, right[8],  val, 0);
    umaal96(o9,  carry0, carry1, right[9],  val, 0);
    umaal96(o10, carry0, carry1, right[10], val, 0);
    umaal96(o11, carry0, carry1, right[11], val, 0);
    uint32_t o12 = carry0;
    uint32_t o13 = carry1;

    val = left[1];
    umlal96(o2,  carry0, carry1, right[0],  val);
    output[2] = o2;
    umaal96(o3,  carry0, carry1, right[1],  val);
    output[3] = o3;
    umaal96(o4,  carry0, carry1, right[2],  val);
    umaal96(o5,  carry0, carry1, right[3],  val);
    umaal96(o6,  carry0, carry1, right[4],  val);
    umaal96(o7,  carry0, carry1, right[5],  val);
    umaal96(o8,  carry0, carry1, right[6],  val);
    umaal96(o9,  carry0, carry1, right[7],  val);
    umaal96(o10, carry0, carry1, right[8],  val);
    umaal96(o11, carry0, carry1, right[9],  val);
    umaal96(o12, carry0, carry1, right[10], val);
    umaal96(o13, carry0, carry1, right[11], val);
    uint32_t o14 = carry0;
    uint32_t o15 = carry1;

    val = left[2];
    umlal96(o4,  carry0, carry1, right[0],  val);
    output[4] = o4;
    umaal96(o5,  carry0, carry1, right[1],  val);
    output[5] = o5;
    umaal96(o6,  carry0, carry1, right[2],  val);
    umaal96(o7,  carry0, carry1, right[3],  val);
    umaal96(o8,  carry0, carry1, right[4],  val);
    umaal96(o9,  carry0, carry1, right[5],  val);
    umaal96(o10, carry0, carry1, right[6],  val);
    umaal96(o11, carry0, carry1, right[7],  val);
    umaal96(o12, carry0, carry1, right[8],  val);
    umaal96(o13, carry0, carry1, right[9],  val);
    umaal96(o14, carry0, carry1, right[10], val);
    umaal96(o15, carry0, carry1, right[11], val);
    uint32_t o16 = carry0;
    uint32_t o17 = carry1;


    val = left[3];
    umlal96(o6,  carry0, carry1, right[0],  val);
    output[6] = o6;
    umaal96(o7,  carry0, carry1, right[1],  val);
    output[7] = o7;
    umaal96(o8,  carry0, carry1, right[2],  val);
    umaal96(o9,  carry0, carry1, right[3],  val);
    umaal96(o10, carry0, carry1, right[4],  val);
    umaal96(o11, carry0, carry1, right[5],  val);
    umaal96(o12, carry0, carry1, right[6],  val);
    umaal96(o13, carry0, carry1, right[7],  val);
    umaal96(o14, carry0, carry1, right[8],  val);
    umaal96(o15, carry0, carry1, right[9],  val);
    umaal96(o16, carry0, carry1, right[10], val);
    umaal96(o17, carry0, carry1, right[11], val);
    uint32_t o18 = carry0;
    uint32_t o19 = carry1;

    val = left[4];
    umlal96(o8,  carry0, carry1, right[0],  val);
    output[8] = o8;
    umaal96(o9,  carry0, carry1, right[1],  val);
    output[9] = o9;
    umaal96(o10, carry0, carry1, right[2],  val);
    umaal96(o11, carry0, carry1, right[3],  val);
    umaal96(o12, carry0, carry1, right[4],  val);
    umaal96(o13, carry0, carry1, right[5],  val);
    umaal96(o14, carry0, carry1, right[6],  val);
    umaal96(o15, carry0, carry1, right[7],  val);
    umaal96(o16, carry0, carry1, right[8],  val);
    umaal96(o17, carry0, carry1, right[9],  val);
    umaal96(o18, carry0, carry1, right[10], val);
    umaal96(o19, carry0, carry1, right[11], val);
    uint32_t o20 = carry0;
    uint32_t o21 = carry1;

    val = left[5];
    umlal96(o10, carry0, carry1, right[0],  val);
    output[10] = o10;
    umaal96(o11, carry0, carry1, right[1],  val);
    output[11] = o11;
    umaal96(o12, carry0, carry1, right[2],  val);
    output[12] = o12;
    umaal96(o13, carry0, carry1, right[3],  val);
    output[13] = o13;
    umaal96(o14, carry0, carry1, right[4],  val);
    output[14] = o14;
    umaal96(o15, carry0, carry1, right[5],  val);
    output[15] = o15;
    umaal96(o16, carry0, carry1, right[6],  val);
    output[16] = o16;
    umaal96(o17, carry0, carry1, right[7],  val);
    output[17] = o17;
    umaal96(o18, carry0, carry1, right[8],  val);
    output[18] = o18;
    umaal96(o19, carry0, carry1, right[9],  val);
    output[19] = o19;
    umaal96(o20, carry0, carry1, right[10], val);
    output[20] = o20;
    umaal96(o21, carry0, carry1, right[11], val);
    output[21] = o21;
    output[22] = carry0;
    output[23] = carry1;
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
    register uint32_t altcarry = 0;

    for(int i=0; i<5; ++i){
        uint32_t* r = t + 2*i;
        register uint64_t k = *(uint64_t*)r * inv;
        register uint32_t carry[2] = {0};
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
