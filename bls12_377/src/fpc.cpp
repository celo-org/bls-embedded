#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define uint128_t __uint128_t

void mac(uint64_t* out, uint64_t* out_carry, uint64_t a, uint64_t b, uint64_t c, uint64_t carry) {
    uint128_t ret = (uint128_t) a + ((uint128_t) b * (uint128_t) c) + (uint128_t) carry;
    *out = (uint64_t) ret;
    *out_carry = (uint64_t)(ret >> 64);
}

void ma(uint64_t* out, uint64_t* out_carry, uint64_t a, uint64_t b, uint64_t c) {
    uint128_t ret = (uint128_t) a + ((uint128_t) b * (uint128_t) c);
    *out = (uint64_t) ret;
    *out_carry = (uint64_t)(ret >> 64);
}

void add_carry(uint64_t* output, const uint64_t* left, const uint64_t* right, int n) {
    uint128_t carry = 0;
    while(n-- > 0) {
        carry = carry + (uint128_t) *left + (uint128_t) *right;
        *output = (uint64_t) carry;
        carry = carry >> 64;
        output++;
        left++;
        right++;
    }
    *output = carry;
}

void add(uint64_t* output, const uint64_t* left, const uint64_t* right, int n) {
    uint128_t carry = 0;
    for(int i=0; i<n; i++){
        carry += (uint128_t)left[i] + (uint128_t)right[i];
        output[i] = (uint64_t) carry;
        carry = carry >> 64;
    }
}

void karatsuba(uint64_t* output, const uint64_t* left, const uint64_t* right, unsigned int n) {
    if(n==1){
        uint128_t ret = (uint128_t) left[0] * (uint128_t) right[0];
        output[0] = (uint64_t) ret;
        output[1] = (uint64_t)(ret >> 64);
        return;
    }

    uint64_t left_low[6] = {0};
    uint64_t left_high[6] = {0};
    uint64_t right_low[6] = {0};
    uint64_t right_high[6] = {0};
    uint64_t ll[12] = {0};
    uint64_t lh[12] = {0};
    uint64_t hl[12] = {0};
    uint64_t hh[12] = {0};

    const unsigned int k = n / 2;
    const unsigned int s2 = n - k;

    memcpy(left_low, left, k * sizeof(uint64_t));
    memcpy(left_high, left + k, s2 * sizeof(uint64_t));
    memcpy(right_low, right, k * sizeof(uint64_t));
    memcpy(right_high, right + k, s2 * sizeof(uint64_t));

    karatsuba(ll, left_low, right_low, k);
    karatsuba(lh, left_low, right_high, s2);
    karatsuba(hl, left_high, right_low, s2);
    karatsuba(hh, left_high, right_high, s2);

    memset(output, 0, 2 * n * sizeof(uint64_t));
    memcpy(output, ll, 2 * k * sizeof(uint64_t));
    add(output + k, output + k, lh, 2*n - k);
    add(output + k, output + k, hl, 2*n - k);
    add(output + 2*k, output + 2*k, hh, 2*n - 2*k);
}

extern "C" void c_mul(uint64_t* output, const uint64_t* left, const uint64_t* right) {
    karatsuba(output, left, right, 6);
}



extern "C" void c_mul_basic(uint64_t* output, const uint64_t* left, const uint64_t* right) {
    uint64_t carry, t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11;

    mac(&t0, &carry, 0, left[0], right[0], 0);
    mac(&t1, &carry, 0, left[0], right[1], carry);
    mac(&t2, &carry, 0, left[0], right[2], carry);
    mac(&t3, &carry, 0, left[0], right[3], carry);
    mac(&t4, &carry, 0, left[0], right[4], carry);
    mac(&t5, &t6, 0, left[0], right[5], carry);

    mac(&t1, &carry, t1, left[1], right[0], 0);
    mac(&t2, &carry, t2, left[1], right[1], carry);
    mac(&t3, &carry, t3, left[1], right[2], carry);
    mac(&t4, &carry, t4, left[1], right[3], carry);
    mac(&t5, &carry, t5, left[1], right[4], carry);
    mac(&t6, &t7, t6, left[1], right[5], carry);

    mac(&t2, &carry, t2, left[2], right[0], 0);
    mac(&t3, &carry, t3, left[2], right[1], carry);
    mac(&t4, &carry, t4, left[2], right[2], carry);
    mac(&t5, &carry, t5, left[2], right[3], carry);
    mac(&t6, &carry, t6, left[2], right[4], carry);
    mac(&t7, &t8, t7, left[2], right[5], carry);

    mac(&t3, &carry, t3, left[3], right[0], 0);
    mac(&t4, &carry, t4, left[3], right[1], carry);
    mac(&t5, &carry, t5, left[3], right[2], carry);
    mac(&t6, &carry, t6, left[3], right[3], carry);
    mac(&t7, &carry, t7, left[3], right[4], carry);
    mac(&t8, &t9, t8, left[3], right[5], carry);

    mac(&t4, &carry, t4, left[4], right[0], 0);
    mac(&t5, &carry, t5, left[4], right[1], carry);
    mac(&t6, &carry, t6, left[4], right[2], carry);
    mac(&t7, &carry, t7, left[4], right[3], carry);
    mac(&t8, &carry, t8, left[4], right[4], carry);
    mac(&t9, &t10, t9, left[4], right[5], carry);

    mac(&t5, &carry, t5, left[5], right[0], 0);
    mac(&t6, &carry, t6, left[5], right[1], carry);
    mac(&t7, &carry, t7, left[5], right[2], carry);
    mac(&t8, &carry, t8, left[5], right[3], carry);
    mac(&t9, &carry, t9, left[5], right[4], carry);
    mac(&t10, &t11, t10, left[5], right[5], carry);

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
}