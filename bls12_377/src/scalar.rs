//! This module provides an implementation of the BLS12-377 scalar field $\mathbb{F}_q$
//! where `q = 8444461749428370424248824938781546531375899335154063827935233455917409239041`

use core::fmt;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use byteorder::{ByteOrder, LittleEndian};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::util::{adc, mac, sbb};

/// Represents an element of the scalar field $\mathbb{F}_q$ of the BLS12-377 elliptic
/// curve construction.
// The internal representation of this type is four 64-bit unsigned
// integers in little-endian order. `Scalar` values are always in
// Montgomery form; i.e., Scalar(a) = aR mod q, with R = 2^256.
#[derive(Clone, Copy, Eq)]
pub struct Scalar(pub(crate) [u64; 4]);

impl fmt::Debug for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tmp = self.to_bytes();
        write!(f, "0x")?;
        for &b in tmp.iter().rev() {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl From<u64> for Scalar {
    fn from(val: u64) -> Scalar {
        Scalar([val, 0, 0, 0]) * r_squared()
    }
}

impl ConstantTimeEq for Scalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0[0].ct_eq(&other.0[0])
            & self.0[1].ct_eq(&other.0[1])
            & self.0[2].ct_eq(&other.0[2])
            & self.0[3].ct_eq(&other.0[3])
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).unwrap_u8() == 1
    }
}

impl ConditionallySelectable for Scalar {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Scalar([
            u64::conditional_select(&a.0[0], &b.0[0], choice),
            u64::conditional_select(&a.0[1], &b.0[1], choice),
            u64::conditional_select(&a.0[2], &b.0[2], choice),
            u64::conditional_select(&a.0[3], &b.0[3], choice),
        ])
    }
}

/// Constant representing the modulus
/// q = 8444461749428370424248824938781546531375899335154063827935233455917409239041
const fn modulus() -> Scalar {
    Scalar([
        725501752471715841u64,
        6461107452199829505u64,
        6968279316240510977u64,
        1345280370688173398u64,
    ])
}

impl<'a> Neg for &'a Scalar {
    type Output = Scalar;

    fn neg(self) -> Scalar {
        self.neg()
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Scalar {
        -&self
    }
}

impl<'a, 'b> Sub<&'b Scalar> for &'a Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &'b Scalar) -> Scalar {
        self.sub(rhs)
    }
}

impl<'a, 'b> Add<&'b Scalar> for &'a Scalar {
    type Output = Scalar;

    fn add(self, rhs: &'b Scalar) -> Scalar {
        self.add(rhs)
    }
}

impl<'a, 'b> Mul<&'b Scalar> for &'a Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &'b Scalar) -> Scalar {
        self.mul(rhs)
    }
}

impl_binops_additive!(Scalar, Scalar);
impl_binops_multiplicative!(Scalar, Scalar);

/// INV = -(q^{-1} mod 2^64) mod 2^64
const fn inv() -> u64 {
    725501752471715839u64
}

/// R = 2^256 mod q
const fn r() -> Scalar {
   Scalar([
       0x7D1C7FFFFFFFFFF3,
       0x7257F50F6FFFFFF2,
       0x16D81575512C0FEE,
       0xD4BDA322BBB9A9D,
   ]) 
}

/// R^2 = 2^512 mod q
#[inline]
const fn r_squared() -> Scalar {
    Scalar([
        0x25D577BAB861857B,
        0xCC2C27B58860591F,
        0xA7CC008FE5DC8593,
        0x11FDAE7EFF1C939,
    ]) 
}

/// R^3 = 2^768 mod q
const fn r_cubed() -> Scalar {
    Scalar([
        0x6A4295C90F65454C, 
        0x624D23FFAE271699,
        0xB1E55EF6F1C9D713,
        0x601DFA555C48DDA,
    ])
}

const fn s() -> u32 {
    47
}

/// GENERATOR^t where t * 2^s + 1 = q
/// with t odd. In other words, this
/// is a 2^s root of unity.
///
/// `GENERATOR = 7 mod q` is a generator
/// of the q - 1 order multiplicative
/// subgroup.
const fn root_of_unity() -> Scalar {
    Scalar([
        0x3c3d3ca739381fb2,
        0x9a14cda3ec99772b,
        0xd7aacc7c59724826,
        0xd1ba211c5cc349c,
    ])
}

impl Default for Scalar {
    fn default() -> Self {
        Self::zero()
    }
}

impl Scalar {
    /// Returns zero, the additive identity.
    pub const fn zero() -> Scalar {
        Scalar([0, 0, 0, 0])
    }

    /// Returns one, the multiplicative identity.
    pub const fn one() -> Scalar {
        r()
    }

    /// Doubles this field element.
    pub fn double(&self) -> Scalar {
        // TODO: This can be achieved more efficiently with a bitshift.
        self.add(self)
    }

    /// Attempts to convert a little-endian byte representation of
    /// a scalar into a `Scalar`, failing if the input is not canonical.
    pub fn from_bytes(bytes: &[u8; 32]) -> CtOption<Scalar> {
        let mut tmp = Scalar([0, 0, 0, 0]);
        let modulus = modulus();

        tmp.0[0] = LittleEndian::read_u64(&bytes[0..8]);
        tmp.0[1] = LittleEndian::read_u64(&bytes[8..16]);
        tmp.0[2] = LittleEndian::read_u64(&bytes[16..24]);
        tmp.0[3] = LittleEndian::read_u64(&bytes[24..32]);

        // Try to subtract the modulus
        let (_, borrow) = sbb(tmp.0[0], modulus.0[0], 0);
        let (_, borrow) = sbb(tmp.0[1], modulus.0[1], borrow);
        let (_, borrow) = sbb(tmp.0[2], modulus.0[2], borrow);
        let (_, borrow) = sbb(tmp.0[3], modulus.0[3], borrow);

        // If the element is smaller than MODULUS then the
        // subtraction will underflow, producing a borrow value
        // of 0xffff...ffff. Otherwise, it'll be zero.
        let is_some = (borrow as u8) & 1;

        // Convert to Montgomery form by computing
        // (a.R^0 * R^2) / R = a.R
        tmp *= &r_squared();

        CtOption::new(tmp, Choice::from(is_some))
    }

    /// Converts an element of `Scalar` into a byte representation in
    /// little-endian byte order.
    pub fn to_bytes(&self) -> [u8; 32] {
        // Turn into canonical form by computing
        // (a.R) / R = a
        let tmp = Scalar::montgomery_reduce(self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0);

        let mut res = [0; 32];
        LittleEndian::write_u64(&mut res[0..8], tmp.0[0]);
        LittleEndian::write_u64(&mut res[8..16], tmp.0[1]);
        LittleEndian::write_u64(&mut res[16..24], tmp.0[2]);
        LittleEndian::write_u64(&mut res[24..32], tmp.0[3]);

        res
    }

    /// Converts a 512-bit little endian integer into
    /// a `Scalar` by reducing by the modulus.
    pub fn from_bytes_wide(bytes: &[u8; 64]) -> Scalar {
        Scalar::from_u512([
            LittleEndian::read_u64(&bytes[0..8]),
            LittleEndian::read_u64(&bytes[8..16]),
            LittleEndian::read_u64(&bytes[16..24]),
            LittleEndian::read_u64(&bytes[24..32]),
            LittleEndian::read_u64(&bytes[32..40]),
            LittleEndian::read_u64(&bytes[40..48]),
            LittleEndian::read_u64(&bytes[48..56]),
            LittleEndian::read_u64(&bytes[56..64]),
        ])
    }

    fn from_u512(limbs: [u64; 8]) -> Scalar {
        // We reduce an arbitrary 512-bit number by decomposing it into two 256-bit digits
        // with the higher bits multiplied by 2^256. Thus, we perform two reductions
        //
        // 1. the lower bits are multiplied by R^2, as normal
        // 2. the upper bits are multiplied by R^2 * 2^256 = R^3
        //
        // and computing their sum in the field. It remains to see that arbitrary 256-bit
        // numbers can be placed into Montgomery form safely using the reduction. The
        // reduction works so long as the product is less than R=2^256 multipled by
        // the modulus. This holds because for any `c` smaller than the modulus, we have
        // that (2^256 - 1)*c is an acceptable product for the reduction. Therefore, the
        // reduction always works so long as `c` is in the field; in this case it is either the
        // constant `R2` or `R3`.
        let d0 = Scalar([limbs[0], limbs[1], limbs[2], limbs[3]]);
        let d1 = Scalar([limbs[4], limbs[5], limbs[6], limbs[7]]);
        // Convert to Montgomery form
        d0 * r_squared() + d1 * r_cubed()
    }

    /// Converts from an integer represented in little endian
    /// into its (congruent) `Scalar` representation.
    #[inline]
    pub fn from_raw(val: [u64; 4]) -> Self {
        (&Scalar(val)).mul(&r_squared())
    }

    /// Squares this element.
    pub fn square(&self) -> Scalar {
        let (r1, carry) = mac(0, self.0[0], self.0[1], 0);
        let (r2, carry) = mac(0, self.0[0], self.0[2], carry);
        let (r3, r4) = mac(0, self.0[0], self.0[3], carry);

        let (r3, carry) = mac(r3, self.0[1], self.0[2], 0);
        let (r4, r5) = mac(r4, self.0[1], self.0[3], carry);

        let (r5, r6) = mac(r5, self.0[2], self.0[3], 0);

        let r7 = r6 >> 63;
        let r6 = (r6 << 1) | (r5 >> 63);
        let r5 = (r5 << 1) | (r4 >> 63);
        let r4 = (r4 << 1) | (r3 >> 63);
        let r3 = (r3 << 1) | (r2 >> 63);
        let r2 = (r2 << 1) | (r1 >> 63);
        let r1 = r1 << 1;

        let (r0, carry) = mac(0, self.0[0], self.0[0], 0);
        let (r1, carry) = adc(0, r1, carry);
        let (r2, carry) = mac(r2, self.0[1], self.0[1], carry);
        let (r3, carry) = adc(0, r3, carry);
        let (r4, carry) = mac(r4, self.0[2], self.0[2], carry);
        let (r5, carry) = adc(0, r5, carry);
        let (r6, carry) = mac(r6, self.0[3], self.0[3], carry);
        let (r7, _) = adc(0, r7, carry);

        Scalar::montgomery_reduce(r0, r1, r2, r3, r4, r5, r6, r7)
    }

    /// Computes the square root of this element, if it exists.
    pub fn sqrt(&self) -> CtOption<Self> {
        // Tonelli-Shank's algorithm for q mod 16 = 1
        // https://eprint.iacr.org/2012/685.pdf (page 12, algorithm 5)

        // w = self^((t - 1) // 2)
        //   = self^6104339283789297388802252303364915521546564123189034618274734669823
        let w = self.pow_vartime(&[
            0x7fff2dff7fffffff,
            0x04d0ec02a9ded201,
            0x94cebea4199cec04,
            0x0000000039f6d3a9,
        ]);

        let s = s();

        let mut v = s;
        let mut x = self * w;
        let mut b = x * w;

        // Initialize z as the 2^S root of unity.
        let mut z = root_of_unity();

        for max_v in (1..=s).rev() {
            let mut k = 1;
            let mut tmp = b.square();
            let mut j_less_than_v: Choice = 1.into();

            for j in 2..max_v {
                let tmp_is_one = tmp.ct_eq(&Scalar::one());
                let squared = Scalar::conditional_select(&tmp, &z, tmp_is_one).square();
                tmp = Scalar::conditional_select(&squared, &tmp, tmp_is_one);
                let new_z = Scalar::conditional_select(&z, &squared, tmp_is_one);
                j_less_than_v &= !j.ct_eq(&v);
                k = u32::conditional_select(&j, &k, tmp_is_one);
                z = Scalar::conditional_select(&z, &new_z, j_less_than_v);
            }

            let result = x * z;
            x = Scalar::conditional_select(&result, &x, b.ct_eq(&Scalar::one()));
            z = z.square();
            b *= z;
            v = k;
        }

        CtOption::new(
            x,
            (x * x).ct_eq(self), // Only return Some if it's the square root.
        )
    }

    /// Exponentiates `self` by `by`, where `by` is a
    /// little-endian order integer exponent.
    pub fn pow(&self, by: &[u64; 4]) -> Self {
        let mut res = Self::one();
        for e in by.iter().rev() {
            for i in (0..64).rev() {
                res = res.square();
                let mut tmp = res;
                tmp *= self;
                res.conditional_assign(&tmp, (((*e >> i) & 0x1) as u8).into());
            }
        }
        res
    }

    /// Exponentiates `self` by `by`, where `by` is a
    /// little-endian order integer exponent.
    ///
    /// **This operation is variable time with respect
    /// to the exponent.** If the exponent is fixed,
    /// this operation is effectively constant time.
    pub fn pow_vartime(&self, by: &[u64; 4]) -> Self {
        let mut res = Self::one();
        for e in by.iter().rev() {
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res.mul_assign(self);
                }
            }
        }
        res
    }

    #[inline]
    fn montgomery_reduce(
        r0: u64,
        r1: u64,
        r2: u64,
        r3: u64,
        r4: u64,
        r5: u64,
        r6: u64,
        r7: u64,
    ) -> Self {
        // The Montgomery reduction here is based on Algorithm 14.32 in
        // Handbook of Applied Cryptography
        // <http://cacr.uwaterloo.ca/hac/about/chap14.pdf>.

        let modulus = modulus();
        let inv = inv();
        let k = r0.wrapping_mul(inv);
        let (_, carry) = mac(r0, k, modulus.0[0], 0);
        let (r1, carry) = mac(r1, k, modulus.0[1], carry);
        let (r2, carry) = mac(r2, k, modulus.0[2], carry);
        let (r3, carry) = mac(r3, k, modulus.0[3], carry);
        let (r4, carry2) = adc(r4, 0, carry);

        let k = r1.wrapping_mul(inv);
        let (_, carry) = mac(r1, k, modulus.0[0], 0);
        let (r2, carry) = mac(r2, k, modulus.0[1], carry);
        let (r3, carry) = mac(r3, k, modulus.0[2], carry);
        let (r4, carry) = mac(r4, k, modulus.0[3], carry);
        let (r5, carry2) = adc(r5, carry2, carry);

        let k = r2.wrapping_mul(inv);
        let (_, carry) = mac(r2, k, modulus.0[0], 0);
        let (r3, carry) = mac(r3, k, modulus.0[1], carry);
        let (r4, carry) = mac(r4, k, modulus.0[2], carry);
        let (r5, carry) = mac(r5, k, modulus.0[3], carry);
        let (r6, carry2) = adc(r6, carry2, carry);

        let k = r3.wrapping_mul(inv);
        let (_, carry) = mac(r3, k, modulus.0[0], 0);
        let (r4, carry) = mac(r4, k, modulus.0[1], carry);
        let (r5, carry) = mac(r5, k, modulus.0[2], carry);
        let (r6, carry) = mac(r6, k, modulus.0[3], carry);
        let (r7, _) = adc(r7, carry2, carry);
        // Result may be within MODULUS of the correct value
        (&Scalar([r4, r5, r6, r7])).sub(&modulus)
    }

    /// Multiplies `rhs` by `self`, returning the result.
    #[inline]
    pub fn mul(&self, rhs: &Self) -> Self {
        // Schoolbook multiplication

        let (r0, carry) = mac(0, self.0[0], rhs.0[0], 0);
        let (r1, carry) = mac(0, self.0[0], rhs.0[1], carry);
        let (r2, carry) = mac(0, self.0[0], rhs.0[2], carry);
        let (r3, r4) = mac(0, self.0[0], rhs.0[3], carry);

        let (r1, carry) = mac(r1, self.0[1], rhs.0[0], 0);
        let (r2, carry) = mac(r2, self.0[1], rhs.0[1], carry);
        let (r3, carry) = mac(r3, self.0[1], rhs.0[2], carry);
        let (r4, r5) = mac(r4, self.0[1], rhs.0[3], carry);

        let (r2, carry) = mac(r2, self.0[2], rhs.0[0], 0);
        let (r3, carry) = mac(r3, self.0[2], rhs.0[1], carry);
        let (r4, carry) = mac(r4, self.0[2], rhs.0[2], carry);
        let (r5, r6) = mac(r5, self.0[2], rhs.0[3], carry);

        let (r3, carry) = mac(r3, self.0[3], rhs.0[0], 0);
        let (r4, carry) = mac(r4, self.0[3], rhs.0[1], carry);
        let (r5, carry) = mac(r5, self.0[3], rhs.0[2], carry);
        let (r6, r7) = mac(r6, self.0[3], rhs.0[3], carry);

        Scalar::montgomery_reduce(r0, r1, r2, r3, r4, r5, r6, r7)
    }

    /// Subtracts `rhs` from `self`, returning the result.
    pub fn sub(&self, rhs: &Self) -> Self {
        let modulus = modulus();
        let (d0, borrow) = sbb(self.0[0], rhs.0[0], 0);
        let (d1, borrow) = sbb(self.0[1], rhs.0[1], borrow);
        let (d2, borrow) = sbb(self.0[2], rhs.0[2], borrow);
        let (d3, borrow) = sbb(self.0[3], rhs.0[3], borrow);

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the modulus.
        let (d0, carry) = adc(d0, modulus.0[0] & borrow, 0);
        let (d1, carry) = adc(d1, modulus.0[1] & borrow, carry);
        let (d2, carry) = adc(d2, modulus.0[2] & borrow, carry);
        let (d3, _) = adc(d3, modulus.0[3] & borrow, carry);

        Scalar([d0, d1, d2, d3])
    }

    /// Adds `rhs` to `self`, returning the result.
    pub fn add(&self, rhs: &Self) -> Self {
        let (d0, carry) = adc(self.0[0], rhs.0[0], 0);
        let (d1, carry) = adc(self.0[1], rhs.0[1], carry);
        let (d2, carry) = adc(self.0[2], rhs.0[2], carry);
        let (d3, _) = adc(self.0[3], rhs.0[3], carry);

        // Attempt to subtract the modulus, to ensure the value
        // is smaller than the modulus.
        (&Scalar([d0, d1, d2, d3])).sub(&modulus())
    }

    /// Negates `self`.
    pub fn neg(&self) -> Self {
        // Subtract `self` from `MODULUS` to negate. Ignore the final
        // borrow because it cannot underflow; self is guaranteed to
        // be in the field.
        let modulus = modulus();
        let (d0, borrow) = sbb(modulus.0[0], self.0[0], 0);
        let (d1, borrow) = sbb(modulus.0[1], self.0[1], borrow);
        let (d2, borrow) = sbb(modulus.0[2], self.0[2], borrow);
        let (d3, _) = sbb(modulus.0[3], self.0[3], borrow);

        // `tmp` could be `MODULUS` if `self` was zero. Create a mask that is
        // zero if `self` was zero, and `u64::max_value()` if self was nonzero.
        let mask = (((self.0[0] | self.0[1] | self.0[2] | self.0[3]) == 0) as u64).wrapping_sub(1);

        Scalar([d0 & mask, d1 & mask, d2 & mask, d3 & mask])
    }
}

impl<'a> From<&'a Scalar> for [u8; 32] {
    fn from(value: &'a Scalar) -> [u8; 32] {
        value.to_bytes()
    }
}

#[test]
fn test_inv() {
    // Compute -(q^{-1} mod 2^64) mod 2^64 by exponentiating
    // by totient(2**64) - 1

    let true_inv = inv();
    let mut inv = 1u64;
    for _ in 0..63 {
        inv = inv.wrapping_mul(inv);
        inv = inv.wrapping_mul(modulus().0[0]);
    }
    inv = inv.wrapping_neg();

    assert_eq!(inv, true_inv);
}

#[cfg(feature = "std")]
#[test]
fn test_debug() {
    assert_eq!(
        format!("{:?}", Scalar::zero()),
        "0x0000000000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(
        format!("{:?}", Scalar::one()),
        "0x0000000000000000000000000000000000000000000000000000000000000001"
    );
    assert_eq!(
        format!("{:?}", r_squared()),
        "0x1824b159acc5056f998c4fefecbc4ff55884b7fa0003480200000001fffffffe"
    );
}

#[test]
fn test_equality() {
    assert_eq!(Scalar::zero(), Scalar::zero());
    assert_eq!(Scalar::one(), Scalar::one());
    assert_eq!(r_squared(), r_squared());

    assert!(Scalar::zero() != Scalar::one());
    assert!(Scalar::one() != r_squared());
}

#[test]
fn test_to_bytes() {
    assert_eq!(
        Scalar::zero().to_bytes(),
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ]
    );

    assert_eq!(
        Scalar::one().to_bytes(),
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ]
    );

    assert_eq!(
        r_squared().to_bytes(),
        [
            243, 255, 255, 255, 255, 127, 28, 125, 242, 255, 255, 111, 15, 245, 87, 114, 238, 15, 
            44, 81, 117, 21, 216, 22, 157, 154, 187, 43, 50, 218, 75, 13
        ]
    );

    assert_eq!(
        (-&Scalar::one()).to_bytes(),
        [
        0, 0, 0, 0, 0, 128, 17, 10, 1, 0, 0, 208, 254, 118, 170, 89, 1, 176, 55, 92, 30, 77, 180, 
        96, 86, 165, 44, 154, 94, 101, 171, 18
        ]
    );
}

#[test]
fn test_from_bytes() {
    assert_eq!(
        Scalar::from_bytes(&[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ])
        .unwrap(),
        Scalar::zero()
    );

    assert_eq!(
        Scalar::from_bytes(&[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ])
        .unwrap(),
        Scalar::one()
    );

    assert_eq!(
        Scalar::from_bytes(&[
            243, 255, 255, 255, 255, 127, 28, 125, 242, 255, 255, 111, 15, 245, 87, 114, 238, 15, 
            44, 81, 117, 21, 216, 22, 157, 154, 187, 43, 50, 218, 75, 13
        ])
        .unwrap(),
        r_squared()
    );

    // -1 should work
    assert!(
        Scalar::from_bytes(&[
            0, 0, 0, 0, 0, 128, 17, 10, 1, 0, 0, 208, 254, 118, 170, 89, 1, 176, 55, 92, 30, 77, 180, 
            96, 86, 165, 44, 154, 94, 101, 171, 18
        ])
        .is_some()
        .unwrap_u8()
            == 1
    );

    // modulus is invalid
    assert!(
        Scalar::from_bytes(&[
            1, 0, 0, 0, 0, 128, 17, 10, 1, 0, 0, 208, 254, 118, 170, 89, 1, 176, 55, 92, 30, 77, 180, 
            96, 86, 165, 44, 154, 94, 101, 171, 18
        ])
        .is_none()
        .unwrap_u8()
            == 1
    );

    // Anything larger than the modulus is invalid
    assert!(
        Scalar::from_bytes(&[
            2, 0, 0, 0, 0, 128, 17, 10, 1, 0, 0, 208, 254, 118, 170, 89, 1, 176, 55, 92, 30, 77, 180, 
            96, 86, 165, 44, 154, 94, 101, 171, 18
        ])
        .is_none()
        .unwrap_u8()
            == 1
    );
    assert!(
        Scalar::from_bytes(&[
            1, 0, 0, 0, 0, 128, 17, 10, 1, 0, 0, 208, 254, 118, 170, 89, 1, 177, 55, 92, 30, 77, 180, 
            96, 86, 165, 44, 154, 94, 101, 171, 18
        ])
        .is_none()
        .unwrap_u8()
            == 1
    );
    assert!(
        Scalar::from_bytes(&[
            1, 0, 0, 0, 255, 255, 255, 255, 254, 91, 254, 255, 2, 164, 189, 83, 5, 216, 161, 9, 8,
            216, 57, 51, 72, 125, 157, 41, 83, 167, 237, 117
        ])
        .is_none()
        .unwrap_u8()
            == 1
    );
}

#[test]
fn test_from_u512_zero() {
    let modulus = modulus();
    assert_eq!(
        Scalar::zero(),
        Scalar::from_u512([
            modulus.0[0],
            modulus.0[1],
            modulus.0[2],
            modulus.0[3],
            0,
            0,
            0,
            0
        ])
    );
}

#[test]
fn test_from_u512_r() {
    assert_eq!(r(), Scalar::from_u512([1, 0, 0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_from_u512_r_squared() {
    assert_eq!(r_squared(), Scalar::from_u512([0, 0, 0, 0, 1, 0, 0, 0]));
}

#[test]
fn test_from_u512_max() {
    let max_u64 = 0xffffffffffffffff;
    assert_eq!(
        r_cubed() - r(),
        Scalar::from_u512([max_u64, max_u64, max_u64, max_u64, max_u64, max_u64, max_u64, max_u64])
    );
}

#[test]
fn test_from_bytes_wide_r_squared() {
    assert_eq!(
        r_squared(),
        Scalar::from_bytes_wide(&[
            243, 255, 255, 255, 255, 127, 28, 125, 242, 255, 255, 111, 15, 245, 87, 114, 238, 15, 
            44, 81, 117, 21, 216, 22, 157, 154, 187, 43, 50, 218, 75, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
    );
}

#[test]
fn test_from_bytes_wide_negative_one() {
    assert_eq!(
        -&Scalar::one(),
        Scalar::from_bytes_wide(&[
            0, 0, 0, 0, 0, 128, 17, 10, 1, 0, 0, 208, 254, 118, 170, 89, 1, 176, 55, 92, 30, 77, 180, 96, 86, 
            165, 44, 154, 94, 101, 171, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
    );
}

#[test]
fn test_zero() {
    assert_eq!(Scalar::zero(), -&Scalar::zero());
    assert_eq!(Scalar::zero(), Scalar::zero() + Scalar::zero());
    assert_eq!(Scalar::zero(), Scalar::zero() - Scalar::zero());
    assert_eq!(Scalar::zero(), Scalar::zero() * Scalar::zero());
}

#[cfg(test)]
const LARGEST: Scalar = Scalar([
    725501752471715840u64,
    6461107452199829505u64,
    6968279316240510977u64,
    1345280370688173398u64,
]);

#[test]
fn test_addition() {
    let mut tmp = LARGEST;
    tmp += &LARGEST;
    println!("{:x?}", tmp);

    assert_eq!(
        tmp,
        Scalar([
            0xa117fffffffffff, 
            0x59aa76fed0000001, 
            0x60b44d1e5c37b001, 
            0x12ab655e9a2ca556,
        ])
    );

    let mut tmp = LARGEST;
    tmp += &Scalar([1, 0, 0, 0]);

    assert_eq!(tmp, Scalar::zero());
}

#[test]
fn test_negation() {
    let tmp = -&LARGEST;

    assert_eq!(tmp, Scalar([1, 0, 0, 0]));

    let tmp = -&Scalar::zero();
    assert_eq!(tmp, Scalar::zero());
    let tmp = -&Scalar([1, 0, 0, 0]);
    assert_eq!(tmp, LARGEST);
}

#[test]
fn test_subtraction() {
    let mut tmp = LARGEST;
    tmp -= &LARGEST;

    assert_eq!(tmp, Scalar::zero());

    let mut tmp = Scalar::zero();
    tmp -= &LARGEST;

    let mut tmp2 = modulus();
    tmp2 -= &LARGEST;

    assert_eq!(tmp, tmp2);
}

#[test]
fn test_multiplication() {
    let mut cur = LARGEST;

    for _ in 0..100 {
        let mut tmp = cur;
        tmp *= &cur;

        let mut tmp2 = Scalar::zero();
        for b in cur
            .to_bytes()
            .iter()
            .rev()
            .flat_map(|byte| (0..8).rev().map(move |i| ((byte >> i) & 1u8) == 1u8))
        {
            let tmp3 = tmp2;
            tmp2.add_assign(&tmp3);

            if b {
                tmp2.add_assign(&cur);
            }
        }

        assert_eq!(tmp, tmp2);

        cur.add_assign(&LARGEST);
    }
}

#[test]
fn test_squaring() {
    let mut cur = LARGEST;

    for _ in 0..100 {
        let mut tmp = cur;
        tmp = tmp.square();

        let mut tmp2 = Scalar::zero();
        for b in cur
            .to_bytes()
            .iter()
            .rev()
            .flat_map(|byte| (0..8).rev().map(move |i| ((byte >> i) & 1u8) == 1u8))
        {
            let tmp3 = tmp2;
            tmp2.add_assign(&tmp3);

            if b {
                tmp2.add_assign(&cur);
            }
        }

        assert_eq!(tmp, tmp2);

        cur.add_assign(&LARGEST);
    }
}

#[test]
fn test_from_raw() {
    assert_eq!(Scalar::from_raw(modulus().0), Scalar::zero());

    assert_eq!(Scalar::from_raw([1, 0, 0, 0]), r());
}

#[test]
fn test_double() {
    let a = Scalar::from_raw([
        0x1fff3231233ffffd,
        0x4884b7fa00034802,
        0x998c4fefecbc4ff3,
        0x1824b159acc50562,
    ]);

    assert_eq!(a.double(), a + a);
}
