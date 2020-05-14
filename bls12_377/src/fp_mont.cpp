#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define uint128_t __uint128_t

#if __BYTE_ORDER__ != __ORDER_LITTLE_ENDIAN__
#error This code depends on little-endian word order
#endif

// multiply a32 * b64 and accumulate in the 96 bit value stored in [o0, o1, o2]
// o0 is the existing 32 bit value
// o1,o2 is the 64 bit carry

inline
void umaal96(
    uint32_t& o0,
    uint32_t& o1,
    uint32_t& o2,
    uint32_t a,
    uint64_t b
 ) {
#if __arm__
    #ifdef HAVE_UMAAL
	const uint32_t b0 = (uint32_t) b;
	const uint32_t b1 = (uint32_t) (b>>32);

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
	register uint32_t b0, b1, p0, p1;

	b0 = b;
	b1 = b >> 32;

	asm (
	    // b0:o0 = o0 + a*b0
	    "UMULL	%[p0], %[p1], %[a], %[b0]\n\t"
	    "ADDS	%[o0], %[p0]\n\t"
	    "ADC	%[b0], %[p1], #0\n\t"
	    : [p0] "=&r" (p0)
	    , [p1] "=&r" (p1)
	    , [o0]  "+r" (o0)
	    , [b0]  "+r" (b0)

	    : [a]    "r" (a)

	    : "cc"
	);

	asm (
	    // p1:p0 = o2 + a*b1
	    "UMULL	%[p0], %[p1], %[a], %[b1]\n\t"
	    "ADDS	%[p0], %[o2]\n\t"
	    "ADC	%[p1], #0\n\t"
	    : [p0] "=&r" (p0)
	    , [p1] "=&r" (p1)

	    : [a]    "r" (a)
	    , [b1]   "r" (b1)
	    , [o2]   "r" (o2)

	    : "cc"
	);

	asm (
	    // o2:o1:o0 = b0:o0 + p1:p0:o1
	    "ADDS	%[o0], %[o1]\n\t"
	    "ADCS	%[o1], %[b0], %[p0]\n\t"
	    "ADC	%[o2], %[p1], #0\n\t"

	    : [o0] "+r" (o0)
	    , [o1] "+r" (o1)
	    , [o2] "+r" (o2)

	    : [p0]  "r" (p0)
	    , [p1]  "r" (p1)
	    , [b0]  "r" (b0)

	    : "cc"
	);
    #endif
#else
    uint128_t ret = ((uint128_t)a * (uint128_t)b) + (uint128_t)o0 + (uint128_t)o1 +  (((uint128_t)o2)<<32);
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
uint32_t acc_2_2_1(uint32_t* output, const uint32_t* b, uint32_t c0) {
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
void add_2_2_1(uint32_t* output, uint32_t* a, const uint32_t* b, uint32_t c0) {
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

extern "C"
void fp_redc(uint32_t* output, uint32_t* t) {
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
