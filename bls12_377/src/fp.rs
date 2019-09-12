//! This module provides an implementation of the BLS12-377 base field `GF(p)` where `p != 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab`
//! TODO: Put actual modulus here

use core::fmt;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use byteorder::{BigEndian, ByteOrder};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::util::{adc, mac, sbb};

// The internal representation of this type is six 64-bit unsigned
// integers in little-endian order. `Fp` values are always in
// Montgomery form; i.e., Scalar(a) = aR mod p, with R = 2^384.
#[derive(Copy, Clone)]
pub struct Fp([u64; 6]);

impl fmt::Debug for Fp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    /*    write!(f, "{:x}", self.0[0])?;
        write!(f, "{:x}", self.0[1])?;
        write!(f, "{:x}", self.0[2])?;
        write!(f, "{:x}", self.0[3])?;
        write!(f, "{:x}", self.0[4])?;
        write!(f, "{:x}", self.0[5])?;*/
        let tmp = self.to_bytes();
        write!(f, "0x")?;
        for &b in tmp.iter() {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl Default for Fp {
    fn default() -> Self {
        Fp::zero()
    }
}

impl ConstantTimeEq for Fp {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0[0].ct_eq(&other.0[0])
            & self.0[1].ct_eq(&other.0[1])
            & self.0[2].ct_eq(&other.0[2])
            & self.0[3].ct_eq(&other.0[3])
            & self.0[4].ct_eq(&other.0[4])
            & self.0[5].ct_eq(&other.0[5])
    }
}

impl Eq for Fp {}
impl PartialEq for Fp {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).unwrap_u8() == 1
    }
}

impl ConditionallySelectable for Fp {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Fp([
            u64::conditional_select(&a.0[0], &b.0[0], choice),
            u64::conditional_select(&a.0[1], &b.0[1], choice),
            u64::conditional_select(&a.0[2], &b.0[2], choice),
            u64::conditional_select(&a.0[3], &b.0[3], choice),
            u64::conditional_select(&a.0[4], &b.0[4], choice),
            u64::conditional_select(&a.0[5], &b.0[5], choice),
        ])
    }
}

/// p = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
pub const MODULUS: [u64; 6] = [
        0x8508c00000000001,
        0x170b5d4430000000,
        0x1ef3622fba094800,
        0x1a22d9f300f5138f,
        0xc63b05c06ca1493b,
        0x1ae3a4617c510ea,
];

/// INV = -(p^{-1} mod 2^64) mod 2^64
const INV: u64 = 9586122913090633727u64;

const TWO_ADICITY: u32 = 46u32;

/// R = 2^384 mod p
const R: Fp = Fp([
    202099033278250856u64,
    5854854902718660529u64,
    11492539364873682930u64,
    8885205928937022213u64,
    5545221690922665192u64,
    39800542322357402u64,
]);

/// R2 = 2^(384*2) mod p
const R2: Fp = Fp([
    0xb786686c9400cd22,
    0x329fcaab00431b1,
    0x22a5f11162d6b46d,
    0xbfdf7d03827dc3ac,
    0x837e92f041790bf9,
    0x6dfccb1e914b88,
]);

/// zexe codebase implies this is c^t, where p - 1 = 2^s*t and t odd
const ROOT_OF_UNITY: Fp = Fp([
    2022196864061697551u64,
    17419102863309525423u64,
    8564289679875062096u64,
    17152078065055548215u64,
    17966377291017729567u64,
    68610905582439508u64,
]);

const T_MINUS_ONE_DIV_TWO: [u64; 6] = [
    0xba88600000010a11,
    0xc45f741290002e16,
    0xb3e601ea271e3de6,
    0xb80d94292763445,
    0x748c2f8a21d58c76,
    0x35c,
];

impl<'a> Neg for &'a Fp {
    type Output = Fp;

    #[inline]
    fn neg(self) -> Fp {
        self.neg()
    }
}

impl Neg for Fp {
    type Output = Fp;

    #[inline]
    fn neg(self) -> Fp {
        -&self
    }
}

impl<'a, 'b> Sub<&'b Fp> for &'a Fp {
    type Output = Fp;

    #[inline]
    fn sub(self, rhs: &'b Fp) -> Fp {
        self.sub(rhs)
    }
}

impl<'a, 'b> Add<&'b Fp> for &'a Fp {
    type Output = Fp;

    #[inline]
    fn add(self, rhs: &'b Fp) -> Fp {
        self.add(rhs)
    }
}

impl<'a, 'b> Mul<&'b Fp> for &'a Fp {
    type Output = Fp;

    #[inline]
    fn mul(self, rhs: &'b Fp) -> Fp {
        self.mul(rhs)
    }
}

impl_binops_additive!(Fp, Fp);
impl_binops_multiplicative!(Fp, Fp);

impl Fp {
    /// Returns zero, the additive identity.
    #[inline]
    pub const fn zero() -> Fp {
        Fp([0, 0, 0, 0, 0, 0])
    }

    /// Returns one, the multiplicative identity.
    #[inline]
    pub const fn one() -> Fp {
        R
    }

    pub fn is_zero(&self) -> Choice {
        self.ct_eq(&Fp::zero())
    }

    pub fn is_one(&self) -> Choice {
        self.ct_eq(&Fp::one())
    }

    /// Attempts to convert a little-endian byte representation of
    /// a scalar into an `Fp`, failing if the input is not canonical.
    pub fn from_bytes(bytes: &[u8; 48]) -> CtOption<Fp> {
        let mut tmp = Fp([0, 0, 0, 0, 0, 0]);

        tmp.0[5] = BigEndian::read_u64(&bytes[0..8]);
        tmp.0[4] = BigEndian::read_u64(&bytes[8..16]);
        tmp.0[3] = BigEndian::read_u64(&bytes[16..24]);
        tmp.0[2] = BigEndian::read_u64(&bytes[24..32]);
        tmp.0[1] = BigEndian::read_u64(&bytes[32..40]);
        tmp.0[0] = BigEndian::read_u64(&bytes[40..48]);

        // Try to subtract the modulus
        let (_, borrow) = sbb(tmp.0[0], MODULUS[0], 0);
        let (_, borrow) = sbb(tmp.0[1], MODULUS[1], borrow);
        let (_, borrow) = sbb(tmp.0[2], MODULUS[2], borrow);
        let (_, borrow) = sbb(tmp.0[3], MODULUS[3], borrow);
        let (_, borrow) = sbb(tmp.0[4], MODULUS[4], borrow);
        let (_, borrow) = sbb(tmp.0[5], MODULUS[5], borrow);

        // If the element is smaller than MODULUS then the
        // subtraction will underflow, producing a borrow value
        // of 0xffff...ffff. Otherwise, it'll be zero.
        let is_some = (borrow as u8) & 1;

        // Convert to Montgomery form by computing
        // (a.R^0 * R^2) / R = a.R
        tmp *= &R2;

        CtOption::new(tmp, Choice::from(is_some))
    }

    /// Converts an element of `Fp` into a byte representation in
    /// big-endian byte order.
    pub fn to_bytes(&self) -> [u8; 48] {
        // Turn into canonical form by computing
        // (a.R) / R = a
        let tmp = Fp::montgomery_reduce(
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], 0, 0, 0, 0, 0, 0,
        );

        let mut res = [0; 48];
        BigEndian::write_u64(&mut res[0..8], tmp.0[5]);
        BigEndian::write_u64(&mut res[8..16], tmp.0[4]);
        BigEndian::write_u64(&mut res[16..24], tmp.0[3]);
        BigEndian::write_u64(&mut res[24..32], tmp.0[2]);
        BigEndian::write_u64(&mut res[32..40], tmp.0[1]);
        BigEndian::write_u64(&mut res[40..48], tmp.0[0]);

        res
    }

    /// Returns whether or not this element is strictly lexicographically
    /// larger than its negation.
    pub fn lexicographically_largest(&self) -> Choice {
        // This can be determined by checking to see if the element is
        // larger than (p - 1) // 2. If we subtract by ((p - 1) // 2) + 1
        // and there is no underflow, then the element must be larger than
        // (p - 1) // 2

        // First, because self is in Montgomery form we need to reduce it
        let tmp = Fp::montgomery_reduce(
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], 0, 0, 0, 0, 0, 0,
        );

        let (_, borrow) = sbb(tmp.0[0], 0x4284600000000001, 0);
        let (_, borrow) = sbb(tmp.0[1], 0x0B85AEA218000000, borrow);
        let (_, borrow) = sbb(tmp.0[2], 0x8F79B117DD04A400, borrow);
        let (_, borrow) = sbb(tmp.0[3], 0x8D116CF9807A89C7, borrow);
        let (_, borrow) = sbb(tmp.0[4], 0x631D82E03650A49D, borrow);
        let (_, borrow) = sbb(tmp.0[5], 0xD71D230BE28875, borrow);

        // If the element was smaller, the subtraction will underflow
        // producing a borrow value of 0xffff...ffff, otherwise it will
        // be zero. We create a Choice representing true if there was
        // overflow (and so this element is not lexicographically larger
        // than its negation) and then negate it.

        !Choice::from((borrow as u8) & 1)
    }

    /// Constructs an element of `Fp` without checking that it is
    /// canonical.
    pub const fn from_raw_unchecked(v: [u64; 6]) -> Fp {
        Fp(v)
    }

    /// Although this is labeled "vartime", it is only
    /// variable time with respect to the exponent. It
    /// is also not exposed in the public API.
    pub fn pow_vartime(&self, by: &[u64; 6]) -> Self {
        let mut res = Self::one();
        for e in by.iter().rev() {
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res *= self;
                }
            }
        }
        res
    }

    //TODO: Clearly indicate this is non-constant
    //TODO: Handle zero, quadratic nonresidue cases
    #[inline]
    pub fn sqrt(&self) -> CtOption<Self> {

        let mut z = ROOT_OF_UNITY;
        let mut w = self.pow_vartime(&T_MINUS_ONE_DIV_TWO);
        let mut x = w * self;
        let mut b = x * &w;

        let mut v = TWO_ADICITY as usize;

        // t = self^t
        {
            let mut check = b;
            for _ in 0..(v-1) {
                check = check.square();
            }
            if b != Fp::one() {
                panic!("Input is not a square root, but passed the QR test")
            }
        }

        while b != Fp::one() {
            let mut k = 0usize;

            let mut b2k = b;
            while b2k != Fp::one() {
                // invariant: b2k = b^(2^k) after entering this loop
                b2k = b2k.square();
                k += 1;
            }

            let j = v - k - 1;
            w = z;
            for _ in 0..j {
                w = w.square();
            }

            z = w.square();
            b *= &z;
            x *= &w;
            v = k;
        }

        CtOption::new(x, 1.ct_eq(&1))
    }

    #[inline]
    /// Computes the multiplicative inverse of this field
    /// element, returning None in the case that this element
    /// is zero.
    pub fn invert(&self) -> CtOption<Self> {
        // Exponentiate by p - 2 
        let t = self.pow_vartime(&[
            0x8508BFFFFFFFFFFF,
            0x170B5D4430000000,
            0x1EF3622FBA094800,
            0x1A22D9F300F5138F,
            0xC63B05C06CA1493B,
            0x1AE3A4617C510EA,
        ]);

        CtOption::new(t, !self.is_zero())
    }

    #[inline]
    const fn subtract_p(&self) -> Fp {
        let (r0, borrow) = sbb(self.0[0], MODULUS[0], 0);
        let (r1, borrow) = sbb(self.0[1], MODULUS[1], borrow);
        let (r2, borrow) = sbb(self.0[2], MODULUS[2], borrow);
        let (r3, borrow) = sbb(self.0[3], MODULUS[3], borrow);
        let (r4, borrow) = sbb(self.0[4], MODULUS[4], borrow);
        let (r5, borrow) = sbb(self.0[5], MODULUS[5], borrow);

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask!
        let r0 = (self.0[0] & borrow) | (r0 & !borrow);
        let r1 = (self.0[1] & borrow) | (r1 & !borrow);
        let r2 = (self.0[2] & borrow) | (r2 & !borrow);
        let r3 = (self.0[3] & borrow) | (r3 & !borrow);
        let r4 = (self.0[4] & borrow) | (r4 & !borrow);
        let r5 = (self.0[5] & borrow) | (r5 & !borrow);

        Fp([r0, r1, r2, r3, r4, r5])
    }

    #[inline]
    pub const fn add(&self, rhs: &Fp) -> Fp {
        let (d0, carry) = adc(self.0[0], rhs.0[0], 0);
        let (d1, carry) = adc(self.0[1], rhs.0[1], carry);
        let (d2, carry) = adc(self.0[2], rhs.0[2], carry);
        let (d3, carry) = adc(self.0[3], rhs.0[3], carry);
        let (d4, carry) = adc(self.0[4], rhs.0[4], carry);
        let (d5, _) = adc(self.0[5], rhs.0[5], carry);

        // Attempt to subtract the modulus, to ensure the value
        // is smaller than the modulus.
        (&Fp([d0, d1, d2, d3, d4, d5])).subtract_p()
    }

    #[inline]
    pub const fn neg(&self) -> Fp {
        let (d0, borrow) = sbb(MODULUS[0], self.0[0], 0);
        let (d1, borrow) = sbb(MODULUS[1], self.0[1], borrow);
        let (d2, borrow) = sbb(MODULUS[2], self.0[2], borrow);
        let (d3, borrow) = sbb(MODULUS[3], self.0[3], borrow);
        let (d4, borrow) = sbb(MODULUS[4], self.0[4], borrow);
        let (d5, _) = sbb(MODULUS[5], self.0[5], borrow);

        // Let's use a mask if `self` was zero, which would mean
        // the result of the subtraction is p.
        let mask = (((self.0[0] | self.0[1] | self.0[2] | self.0[3] | self.0[4] | self.0[5]) == 0)
            as u64)
            .wrapping_sub(1);

        Fp([
            d0 & mask,
            d1 & mask,
            d2 & mask,
            d3 & mask,
            d4 & mask,
            d5 & mask,
        ])
    }

    #[inline]
    pub const fn sub(&self, rhs: &Fp) -> Fp {
        (&rhs.neg()).add(self)
    }

    #[inline(always)]
    const fn montgomery_reduce(
        t0: u64,
        t1: u64,
        t2: u64,
        t3: u64,
        t4: u64,
        t5: u64,
        t6: u64,
        t7: u64,
        t8: u64,
        t9: u64,
        t10: u64,
        t11: u64,
    ) -> Self {
        // The Montgomery reduction here is based on Algorithm 14.32 in
        // Handbook of Applied Cryptography
        // <http://cacr.uwaterloo.ca/hac/about/chap14.pdf>.

        let k = t0.wrapping_mul(INV);
        let (_, carry) = mac(t0, k, MODULUS[0], 0);
        let (r1, carry) = mac(t1, k, MODULUS[1], carry);
        let (r2, carry) = mac(t2, k, MODULUS[2], carry);
        let (r3, carry) = mac(t3, k, MODULUS[3], carry);
        let (r4, carry) = mac(t4, k, MODULUS[4], carry);
        let (r5, carry) = mac(t5, k, MODULUS[5], carry);
        let (r6, r7) = adc(t6, 0, carry);

        let k = r1.wrapping_mul(INV);
        let (_, carry) = mac(r1, k, MODULUS[0], 0);
        let (r2, carry) = mac(r2, k, MODULUS[1], carry);
        let (r3, carry) = mac(r3, k, MODULUS[2], carry);
        let (r4, carry) = mac(r4, k, MODULUS[3], carry);
        let (r5, carry) = mac(r5, k, MODULUS[4], carry);
        let (r6, carry) = mac(r6, k, MODULUS[5], carry);
        let (r7, r8) = adc(t7, r7, carry);

        let k = r2.wrapping_mul(INV);
        let (_, carry) = mac(r2, k, MODULUS[0], 0);
        let (r3, carry) = mac(r3, k, MODULUS[1], carry);
        let (r4, carry) = mac(r4, k, MODULUS[2], carry);
        let (r5, carry) = mac(r5, k, MODULUS[3], carry);
        let (r6, carry) = mac(r6, k, MODULUS[4], carry);
        let (r7, carry) = mac(r7, k, MODULUS[5], carry);
        let (r8, r9) = adc(t8, r8, carry);

        let k = r3.wrapping_mul(INV);
        let (_, carry) = mac(r3, k, MODULUS[0], 0);
        let (r4, carry) = mac(r4, k, MODULUS[1], carry);
        let (r5, carry) = mac(r5, k, MODULUS[2], carry);
        let (r6, carry) = mac(r6, k, MODULUS[3], carry);
        let (r7, carry) = mac(r7, k, MODULUS[4], carry);
        let (r8, carry) = mac(r8, k, MODULUS[5], carry);
        let (r9, r10) = adc(t9, r9, carry);

        let k = r4.wrapping_mul(INV);
        let (_, carry) = mac(r4, k, MODULUS[0], 0);
        let (r5, carry) = mac(r5, k, MODULUS[1], carry);
        let (r6, carry) = mac(r6, k, MODULUS[2], carry);
        let (r7, carry) = mac(r7, k, MODULUS[3], carry);
        let (r8, carry) = mac(r8, k, MODULUS[4], carry);
        let (r9, carry) = mac(r9, k, MODULUS[5], carry);
        let (r10, r11) = adc(t10, r10, carry);

        let k = r5.wrapping_mul(INV);
        let (_, carry) = mac(r5, k, MODULUS[0], 0);
        let (r6, carry) = mac(r6, k, MODULUS[1], carry);
        let (r7, carry) = mac(r7, k, MODULUS[2], carry);
        let (r8, carry) = mac(r8, k, MODULUS[3], carry);
        let (r9, carry) = mac(r9, k, MODULUS[4], carry);
        let (r10, carry) = mac(r10, k, MODULUS[5], carry);
        let (r11, _) = adc(t11, r11, carry);

        // Attempt to subtract the modulus, to ensure the value
        // is smaller than the modulus.
        (&Fp([r6, r7, r8, r9, r10, r11])).subtract_p()
    }

    #[inline]
    pub const fn mul(&self, rhs: &Fp) -> Fp {
        let (t0, carry) = mac(0, self.0[0], rhs.0[0], 0);
        let (t1, carry) = mac(0, self.0[0], rhs.0[1], carry);
        let (t2, carry) = mac(0, self.0[0], rhs.0[2], carry);
        let (t3, carry) = mac(0, self.0[0], rhs.0[3], carry);
        let (t4, carry) = mac(0, self.0[0], rhs.0[4], carry);
        let (t5, t6) = mac(0, self.0[0], rhs.0[5], carry);

        let (t1, carry) = mac(t1, self.0[1], rhs.0[0], 0);
        let (t2, carry) = mac(t2, self.0[1], rhs.0[1], carry);
        let (t3, carry) = mac(t3, self.0[1], rhs.0[2], carry);
        let (t4, carry) = mac(t4, self.0[1], rhs.0[3], carry);
        let (t5, carry) = mac(t5, self.0[1], rhs.0[4], carry);
        let (t6, t7) = mac(t6, self.0[1], rhs.0[5], carry);

        let (t2, carry) = mac(t2, self.0[2], rhs.0[0], 0);
        let (t3, carry) = mac(t3, self.0[2], rhs.0[1], carry);
        let (t4, carry) = mac(t4, self.0[2], rhs.0[2], carry);
        let (t5, carry) = mac(t5, self.0[2], rhs.0[3], carry);
        let (t6, carry) = mac(t6, self.0[2], rhs.0[4], carry);
        let (t7, t8) = mac(t7, self.0[2], rhs.0[5], carry);

        let (t3, carry) = mac(t3, self.0[3], rhs.0[0], 0);
        let (t4, carry) = mac(t4, self.0[3], rhs.0[1], carry);
        let (t5, carry) = mac(t5, self.0[3], rhs.0[2], carry);
        let (t6, carry) = mac(t6, self.0[3], rhs.0[3], carry);
        let (t7, carry) = mac(t7, self.0[3], rhs.0[4], carry);
        let (t8, t9) = mac(t8, self.0[3], rhs.0[5], carry);

        let (t4, carry) = mac(t4, self.0[4], rhs.0[0], 0);
        let (t5, carry) = mac(t5, self.0[4], rhs.0[1], carry);
        let (t6, carry) = mac(t6, self.0[4], rhs.0[2], carry);
        let (t7, carry) = mac(t7, self.0[4], rhs.0[3], carry);
        let (t8, carry) = mac(t8, self.0[4], rhs.0[4], carry);
        let (t9, t10) = mac(t9, self.0[4], rhs.0[5], carry);

        let (t5, carry) = mac(t5, self.0[5], rhs.0[0], 0);
        let (t6, carry) = mac(t6, self.0[5], rhs.0[1], carry);
        let (t7, carry) = mac(t7, self.0[5], rhs.0[2], carry);
        let (t8, carry) = mac(t8, self.0[5], rhs.0[3], carry);
        let (t9, carry) = mac(t9, self.0[5], rhs.0[4], carry);
        let (t10, t11) = mac(t10, self.0[5], rhs.0[5], carry);

        Self::montgomery_reduce(t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11)
    }

    /// Squares this element.
    #[inline]
    pub const fn square(&self) -> Self {
        let (t1, carry) = mac(0, self.0[0], self.0[1], 0);
        let (t2, carry) = mac(0, self.0[0], self.0[2], carry);
        let (t3, carry) = mac(0, self.0[0], self.0[3], carry);
        let (t4, carry) = mac(0, self.0[0], self.0[4], carry);
        let (t5, t6) = mac(0, self.0[0], self.0[5], carry);

        let (t3, carry) = mac(t3, self.0[1], self.0[2], 0);
        let (t4, carry) = mac(t4, self.0[1], self.0[3], carry);
        let (t5, carry) = mac(t5, self.0[1], self.0[4], carry);
        let (t6, t7) = mac(t6, self.0[1], self.0[5], carry);

        let (t5, carry) = mac(t5, self.0[2], self.0[3], 0);
        let (t6, carry) = mac(t6, self.0[2], self.0[4], carry);
        let (t7, t8) = mac(t7, self.0[2], self.0[5], carry);

        let (t7, carry) = mac(t7, self.0[3], self.0[4], 0);
        let (t8, t9) = mac(t8, self.0[3], self.0[5], carry);

        let (t9, t10) = mac(t9, self.0[4], self.0[5], 0);

        let t11 = t10 >> 63;
        let t10 = (t10 << 1) | (t9 >> 63);
        let t9 = (t9 << 1) | (t8 >> 63);
        let t8 = (t8 << 1) | (t7 >> 63);
        let t7 = (t7 << 1) | (t6 >> 63);
        let t6 = (t6 << 1) | (t5 >> 63);
        let t5 = (t5 << 1) | (t4 >> 63);
        let t4 = (t4 << 1) | (t3 >> 63);
        let t3 = (t3 << 1) | (t2 >> 63);
        let t2 = (t2 << 1) | (t1 >> 63);
        let t1 = t1 << 1;

        let (t0, carry) = mac(0, self.0[0], self.0[0], 0);
        let (t1, carry) = adc(t1, 0, carry);
        let (t2, carry) = mac(t2, self.0[1], self.0[1], carry);
        let (t3, carry) = adc(t3, 0, carry);
        let (t4, carry) = mac(t4, self.0[2], self.0[2], carry);
        let (t5, carry) = adc(t5, 0, carry);
        let (t6, carry) = mac(t6, self.0[3], self.0[3], carry);
        let (t7, carry) = adc(t7, 0, carry);
        let (t8, carry) = mac(t8, self.0[4], self.0[4], carry);
        let (t9, carry) = adc(t9, 0, carry);
        let (t10, carry) = mac(t10, self.0[5], self.0[5], carry);
        let (t11, _) = adc(t11, 0, carry);

        Self::montgomery_reduce(t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11)
    }
}

#[test]
fn test_conditional_selection() {
    let a = Fp([1, 2, 3, 4, 5, 6]);
    let b = Fp([7, 8, 9, 10, 11, 12]);

    assert_eq!(
        ConditionallySelectable::conditional_select(&a, &b, Choice::from(0u8)),
        a
    );
    assert_eq!(
        ConditionallySelectable::conditional_select(&a, &b, Choice::from(1u8)),
        b
    );
}

#[test]
fn test_equality() {
    fn is_equal(a: &Fp, b: &Fp) -> bool {
        let eq = a == b;
        let ct_eq = a.ct_eq(&b);

        assert_eq!(eq, ct_eq.unwrap_u8() == 1);

        eq
    }

    assert_eq!(&Fp([1, 2, 3, 4, 5, 6]), &Fp([1, 2, 3, 4, 5, 6]));

    let a = Fp([7, 2, 3, 4, 5, 6]);
    let b = Fp([1, 2, 3, 4, 5, 6]);
    assert_ne!(&a, &b);
    assert!(!is_equal(&Fp([1, 7, 3, 4, 5, 6]), &Fp([1, 2, 3, 4, 5, 6])));
    assert!(!is_equal(&Fp([1, 2, 7, 4, 5, 6]), &Fp([1, 2, 3, 4, 5, 6])));
    assert!(!is_equal(&Fp([1, 2, 3, 7, 5, 6]), &Fp([1, 2, 3, 4, 5, 6])));
    assert!(!is_equal(&Fp([1, 2, 3, 4, 7, 6]), &Fp([1, 2, 3, 4, 5, 6])));
    assert!(!is_equal(&Fp([1, 2, 3, 4, 5, 7]), &Fp([1, 2, 3, 4, 5, 6])));
}

#[test]
fn test_squaring() {
    let a = Fp([
        0xd215d2768e83191b,
        0x5085d80f8fb28261,
        0xce9a032ddf393a56,
        0x3e9c4fff2ca0c4bb,
        0x6436b6f7f4d95dfb,
        0x10606628ad4a4d90,
    ]);
    let b = Fp([
        0xc27f4faf338e6e7, 
        0xb9363389626f355, 
        0x2677a23d5ff9b701, 
        0xaa7da7ecaa317421, 
        0xd813d973bd2c6c51, 
        0x1363906dc99b15d,
    ]);

    assert_eq!(a.square(), b);
}

#[test]
fn test_multiplication() {
    let a = Fp([
        0x397a38320170cd4,
        0x734c1b2c9e761d30,
        0x5ed255ad9a48beb5,
        0x95a3c6b22a7fcfc,
        0x2294ce75d4e26a27,
        0x13338bd870011ebb,
    ]);
    let b = Fp([
        0xb9c3c7c5b1196af7,
        0x2580e2086ce335c1,
        0xf49aed3d8a57ef42,
        0x41f281e49846e878,
        0xe0762346c38452ce,
        0x652e89326e57dc0,
    ]);
    let c = Fp([
        0x797a886e0e8e8d85, 
        0x518df0f1d1732800, 
        0xb7098a12c4a10c5, 
        0x6338f6a9ec896084, 
        0xec6b4921810a39fc, 
        0x1751097d914d4be
    ]);

    assert_eq!(a * b, c);
}

#[test]
fn test_addition() {
    let a = Fp([
        0x5360bb5978678032,
        0x7dd275ae799e128e,
        0x5c5b5071ce4f4dcf,
        0xcdb21f93078dbb3e,
        0xc32365c5e73f474a,
        0x115a2a5489babe5b,
    ]);
    let b = Fp([
        0x9fd287733d23dda0,
        0xb16bf2af738b3554,
        0x3e57a75bd3cc6d1d,
        0x900bc0bd627fd6d6,
        0xd319a080efb245fe,
        0x15fdcaa4e4bb2091,
    ]);
    let c = Fp([
        0x6E2A82CCB58B5DD1,
        0x18330B19BD2947E2,
        0x7BBF959DE81272ED,
        0x439B065D69187E85,
        0xD00200866A50440E,
        0x25A9BAB356B0CE02,
    ]);

    assert_eq!(a + b, c);
}

//TODO: Use result smaller than modulus
#[test]
fn test_subtraction() {
    let a = Fp([
        0x5360bb5978678032,
        0x7dd275ae799e128e,
        0x5c5b5071ce4f4dcf,
        0xcdb21f93078dbb3e,
        0xc32365c5e73f474a,
        0x115a2a5489babe5b,
    ]);
    let b = Fp([
        0x9fd287733d23dda0,
        0xb16bf2af738b3554,
        0x3e57a75bd3cc6d1d,
        0x900bc0bd627fd6d6,
        0xd319a080efb245fe,
        0x15fdcaa4e4bb2091,
    ]);
    // ([3896f3e63b43a293, e371e0433612dd3a, 3cf70b45b48c28b1, 57c938c8a602f7f7, b644cb05642e4a87, fd0a99f5bcc4aeb4]))
    let c = Fp([
        0x3896f3e63b43a293,
        0xe371e0433612dd3a,
        0x3cf70b45b48c28b1,
        0x57c938c8a602f7f7,
        0xb644cb05642e4a87,
        0xfd0a99f5bcc4aeb4,
    ]);
    let c = c.subtract_p();

    assert_eq!(a - b, c);
}

#[test]
fn test_negation() {
    let a = Fp([
        0x5360bb5978678032,
        0x7dd275ae799e128e,
        0x5c5b5071ce4f4dcf,
        0xcdb21f93078dbb3e,
        0xc32365c5e73f474a,
        0x115a2a5489babe5b,
    ]);
    let b = Fp([
        0x31a804a687987fcf, 
        0x9938e795b661ed72, 
        0xc29811bdebb9fa30, 
        0x4c70ba5ff9675850, 
        0x3179ffa856201f0, 
        0xf0540ff18e0a528f,
    ]);

    assert_eq!(-a, b);
}

#[test]
fn test_debug() {
    assert_eq!(
        format!(
            "{:?}",
            Fp([0x5360bb5978678032, 0x7dd275ae799e128e, 0x5c5b5071ce4f4dcf, 0xcdb21f93078dbb3e, 0xc32365c5e73f474a, 0x115a2a5489babe5b])
        ),
        "0x104bf052ad3bc99bcb176c24a06a6c3aad4eaf2308fc4d282e106c84a757d061052630515305e59bdddf8111bfdeb704"
    );
}

#[test]
fn test_from_bytes() {
    let mut a = Fp([
        0xdc906d9be3f95dc8,
        0x8755caf7459691a1,
        0xcff1a7f4e9583ab3,
        0x9b43821f849e2284,
        0xf57554f3a2974f3f,
        0x85dbea84ed47f79,
    ]);

    for _ in 0..100 {
        a = a.square();
        let tmp = a.to_bytes();
        let b = Fp::from_bytes(&tmp).unwrap();

        assert_eq!(a, b);
    }

    assert_eq!(
        -Fp::one(),
    Fp::from_bytes(&[1, 174, 58, 70, 23, 197, 16, 234, 198, 59, 5, 192, 108, 161, 73, 59, 26, 34, 217, 243, 0, 245, 19, 143, 30, 243, 98, 47, 186, 9, 72, 0, 23, 11, 93, 68, 48, 0, 0, 0, 133, 8, 192, 0, 0, 0, 0, 0]).unwrap()
    );

    assert!(
        Fp::from_bytes(&[
            27, 1, 17, 234, 57, 127, 230, 154, 75, 27, 167, 182, 67, 75, 172, 215, 100, 119, 75,
            132, 243, 133, 18, 191, 103, 48, 210, 160, 246, 176, 246, 36, 30, 171, 255, 254, 177,
            83, 255, 255, 185, 254, 255, 255, 255, 255, 170, 170
        ])
        .is_none()
        .unwrap_u8()
            == 1
    );

    assert!(Fp::from_bytes(&[0xff; 48]).is_none().unwrap_u8() == 1);
}

#[test]
fn test_sqrt() {
    // a = 4
    let a = Fp::from_raw_unchecked([
        0xaa270000000cfff3,
        0x53cc0032fc34000a,
        0x478fe97a6b0a807f,
        0xb1d37ebee6ba24d7,
        0x8ec9733bbf78ab2f,
        0x9d645513d83de7e,
    ]);
    // [b7365bc1527cc225, 80c4410c13dad980, 405a608866ec9af9, bae77f06775d9e86, 631d7a2378887188, 24475d61e565d7]

    assert_eq!(
        // sqrt(4) = -2
        a.sqrt().unwrap(),
        // 2
        Fp::from_raw_unchecked([
            0xb7365bc1527cc225,
            0x80c4410c13dad980, 
            0x405a608866ec9af9, 
            0xbae77f06775d9e86, 
            0x631d7a2378887188, 
            0x24475d61e565d7,
        ])
    );
}

#[test]
fn test_inversion() {
    let a = Fp([
        0x43b43a5078ac2076,
        0x1ce0763046f8962b,
        0x724a5276486d735c,
        0x6f05c2a6282d48fd,
        0x2095bd5bb4ca9331,
        0x3b35b3894b0f7da,
    ]);
    let b = Fp([
        0x46e62daa07fc3fba,
        0x7a3ba1598ea4f941, 
        0x675f586198cad5e3, 
        0xd3c06c64199ca906, 
        0x61617cc7f1012816, 
        0xefb2f069ef448e,
    ]);

    assert_eq!(a.invert().unwrap(), b);
    assert!(Fp::zero().invert().is_none().unwrap_u8() == 1);
}

#[test]
fn test_lexicographic_largest() {
    assert!(!bool::from(Fp::zero().lexicographically_largest()));
    assert!(!bool::from(Fp::one().lexicographically_largest()));
    assert!(!bool::from(
        Fp::from_raw_unchecked([
            0xa1fafffffffe5557,
            0x995bfff976a3fffe,
            0x3f41d24d174ceb4,
            0xf6547998c1995dbd,
            0x778a468f507a6034,
            0x20559931f7f8103
        ])
        .lexicographically_largest()
    ));
    assert!(bool::from(
        Fp::from_raw_unchecked([
            0x1804000000015554,
            0x855000053ab00001,
            0x633cb57c253c276f,
            0x6e22d1ec31ebb502,
            0xd3916126f2d14ca2,
            0x17fbb8571a006596
        ])
        .lexicographically_largest()
    ));
    assert!(bool::from(
        Fp::from_raw_unchecked([
            0x43f5fffffffcaaae,
            0x32b7fff2ed47fffd,
            0x7e83a49a2e99d69,
            0xeca8f3318332bb7a,
            0xef148d1ea0f4c069,
            0x40ab3263eff0206
        ])
        .lexicographically_largest()
    ));
}