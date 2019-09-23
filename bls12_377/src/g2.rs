//! This module provides an implementation of the $\mathbb{G}_2$ group of BLS12-381.

use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::fp::Fp;
use crate::fp2::Fp2;
use crate::Scalar;

/// This is an element of $\mathbb{G}_2$ represented in the affine coordinate space.
/// It is ideal to keep elements in this representation to reduce memory usage and
/// improve performance through the use of mixed curve model arithmetic.
///
/// Values of `G2Affine` are guaranteed to be in the $q$-order subgroup unless an
/// "unchecked" API was misused.
#[derive(Copy, Clone, Debug)]
pub struct G2Affine {
    pub(crate) x: Fp2,
    pub(crate) y: Fp2,
    infinity: Choice,
}

impl Default for G2Affine {
    fn default() -> G2Affine {
        G2Affine::identity()
    }
}

impl<'a> From<&'a G2Projective> for G2Affine {
    fn from(p: &'a G2Projective) -> G2Affine {
       let zinv = p.z.invert().unwrap_or(Fp2::zero());
       let zinv2 = zinv.square();
       let x = p.x * zinv2;
       let zinv3 = zinv2 * zinv;
       let y = p.y * zinv3;

        let tmp = G2Affine {
            x,
            y,
            infinity: Choice::from(0u8),
        };

        G2Affine::conditional_select(&tmp, &G2Affine::identity(), zinv.is_zero())
    }
}

impl From<G2Projective> for G2Affine {
    fn from(p: G2Projective) -> G2Affine {
        G2Affine::from(&p)
    }
}

impl ConstantTimeEq for G2Affine {
    fn ct_eq(&self, other: &Self) -> Choice {
        // The only cases in which two points are equal are
        // 1. infinity is set on both
        // 2. infinity is not set on both, and their coordinates are equal

        (self.infinity & other.infinity)
            | ((!self.infinity)
                & (!other.infinity)
                & self.x.ct_eq(&other.x)
                & self.y.ct_eq(&other.y))
    }
}

impl ConditionallySelectable for G2Affine {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        G2Affine {
            x: Fp2::conditional_select(&a.x, &b.x, choice),
            y: Fp2::conditional_select(&a.y, &b.y, choice),
            infinity: Choice::conditional_select(&a.infinity, &b.infinity, choice),
        }
    }
}

impl Eq for G2Affine {}
impl PartialEq for G2Affine {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        bool::from(self.ct_eq(other))
    }
}

impl<'a> Neg for &'a G2Affine {
    type Output = G2Affine;

    #[inline]
    fn neg(self) -> G2Affine {
        G2Affine {
            x: self.x,
            y: Fp2::conditional_select(&-self.y, &Fp2::one(), self.infinity),
            infinity: self.infinity,
        }
    }
}

impl Neg for G2Affine {
    type Output = G2Affine;

    #[inline]
    fn neg(self) -> G2Affine {
        -&self
    }
}

impl<'a, 'b> Add<&'b G2Projective> for &'a G2Affine {
    type Output = G2Projective;

    #[inline]
    fn add(self, rhs: &'b G2Projective) -> G2Projective {
        rhs.add_mixed(self)
    }
}

impl<'a, 'b> Add<&'b G2Affine> for &'a G2Projective {
    type Output = G2Projective;

    #[inline]
    fn add(self, rhs: &'b G2Affine) -> G2Projective {
        self.add_mixed(rhs)
    }
}

impl<'a, 'b> Sub<&'b G2Projective> for &'a G2Affine {
    type Output = G2Projective;

    #[inline]
    fn sub(self, rhs: &'b G2Projective) -> G2Projective {
        self + (-rhs)
    }
}

impl<'a, 'b> Sub<&'b G2Affine> for &'a G2Projective {
    type Output = G2Projective;

    #[inline]
    fn sub(self, rhs: &'b G2Affine) -> G2Projective {
        self + (-rhs)
    }
}

impl_binops_additive!(G2Projective, G2Affine);
impl_binops_additive_specify_output!(G2Affine, G2Projective, G2Projective);

const B: Fp2 = Fp2 {
    c0: Fp::from_raw_unchecked([
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
    ]),
    c1: Fp::from_raw_unchecked([
        9255502405446297221u64,
        10229180150694123945u64,
        9215585410771530959u64,
        13357015519562362907u64,
        5437107869987383107u64,
        16259554076827459u64,
    ]),
};

const G2_GEN_X_C0: Fp = Fp::from_raw_unchecked([
    0x68904082f268725b,
    0x668f2ea74f45328b,
    0xebca7a65802be84f,
    0x1e1850f4c1ada3e6,
    0x830dc22d588ef1e9,
    0x1862a81767c0982, 
]);

const G2_GEN_X_C1: Fp =  Fp::from_raw_unchecked([
    0x5f02a915c91c7f39,
    0xf8c553ba388da2a7,
    0xd51a416dbd198850,
    0xe943c6f38ae3073a,
    0xffe24aa8259a4981,
    0x11853391e73dfdd,  
]);

const G2_GEN_X: Fp2 = Fp2 { c0: G2_GEN_X_C0, c1: G2_GEN_X_C1 };

const G2_GEN_Y_C0: Fp =  Fp::from_raw_unchecked([
    0xd5b19b897881430f,
    0x5be9118a5b371ed,
    0x6063f91f86c131ee,
    0x3244a61be8f4ec19,
    0xa02e425b9f9a3a12,
    0x18af8c04f3360d2, 
]);

const G2_GEN_Y_C1: Fp = Fp::from_raw_unchecked([
    0x57601ac71a5b96f5,
    0xe99acc1714f2440e,
    0x2339612f10118ea9,
    0x8321e68a3b1cd722,
    0x2b543b050cc74917,
    0x590182b396c112,
]); 

const G2_GEN_Y: Fp2 = Fp2 { c0: G2_GEN_Y_C0, c1: G2_GEN_Y_C1 };

impl G2Affine {
    /// Returns the identity of the group: the point at infinity.
    pub fn identity() -> G2Affine {
        G2Affine {
            x: Fp2::zero(),
            y: Fp2::one(),
            infinity: Choice::from(1u8),
        }
    }

    /// Returns a fixed generator of the group. See [`notes::design`](notes/design/index.html#fixed-generators)
    /// for how this generator is chosen.
    pub fn generator() -> G2Affine {
        G2Affine {
            x: G2_GEN_X, 
            y: G2_GEN_Y, 
            infinity: Choice::from(0u8),
        }
    }

    /// Serializes this element into compressed form. See [`notes::serialization`](crate::notes::serialization)
    /// for details about how group elements are serialized.
    pub fn to_compressed(&self) -> [u8; 96] {
        // Strictly speaking, self.x is zero already when self.infinity is true, but
        // to guard against implementation mistakes we do not assume this.
        let x = Fp2::conditional_select(&self.x, &Fp2::zero(), self.infinity);

        let mut res = [0; 96];

        (&mut res[0..48]).copy_from_slice(&x.c1.to_bytes()[..]);
        (&mut res[48..96]).copy_from_slice(&x.c0.to_bytes()[..]);

        // This point is in compressed form, so we set the most significant bit.
        res[0] |= 1u8 << 7;

        // Is this point at infinity? If so, set the second-most significant bit.
        res[0] |= u8::conditional_select(&0u8, &(1u8 << 6), self.infinity);

        // Is the y-coordinate the lexicographically largest of the two associated with the
        // x-coordinate? If so, set the third-most significant bit so long as this is not
        // the point at infinity.
        res[0] |= u8::conditional_select(
            &0u8,
            &(1u8 << 5),
            (!self.infinity) & self.y.lexicographically_largest(),
        );

        res
    }

    /// Serializes this element into uncompressed form. See [`notes::serialization`](crate::notes::serialization)
    /// for details about how group elements are serialized.
    pub fn to_uncompressed(&self) -> [u8; 192] {
        let mut res = [0; 192];

        let x = Fp2::conditional_select(&self.x, &Fp2::zero(), self.infinity);
        let y = Fp2::conditional_select(&self.y, &Fp2::zero(), self.infinity);

        res[0..48].copy_from_slice(&x.c1.to_bytes()[..]);
        res[48..96].copy_from_slice(&x.c0.to_bytes()[..]);
        res[96..144].copy_from_slice(&y.c1.to_bytes()[..]);
        res[144..192].copy_from_slice(&y.c0.to_bytes()[..]);

        // Is this point at infinity? If so, set the second-most significant bit.
        res[0] |= u8::conditional_select(&0u8, &(1u8 << 6), self.infinity);

        res
    }

    /// Attempts to deserialize an uncompressed element. See [`notes::serialization`](crate::notes::serialization)
    /// for details about how group elements are serialized.
    pub fn from_uncompressed(bytes: &[u8; 192]) -> CtOption<Self> {
        Self::from_uncompressed_unchecked(bytes)
            .and_then(|p| CtOption::new(p, p.is_on_curve() & p.is_torsion_free()))
    }

    /// Attempts to deserialize an uncompressed element, not checking if the
    /// element is on the curve and not checking if it is in the correct subgroup.
    /// **This is dangerous to call unless you trust the bytes you are reading; otherwise,
    /// API invariants may be broken.** Please consider using `from_uncompressed()` instead.
    pub fn from_uncompressed_unchecked(bytes: &[u8; 192]) -> CtOption<Self> {
        // Obtain the three flags from the start of the byte sequence
        let compression_flag_set = Choice::from((bytes[0] >> 7) & 1);
        let infinity_flag_set = Choice::from((bytes[0] >> 6) & 1);
        let sort_flag_set = Choice::from((bytes[0] >> 5) & 1);

        // Attempt to obtain the x-coordinate
        let xc1 = {
            let mut tmp = [0; 48];
            tmp.copy_from_slice(&bytes[0..48]);

            // Mask away the flag bits
            tmp[0] &= 0b0001_1111;

            Fp::from_bytes(&tmp)
        };
        let xc0 = {
            let mut tmp = [0; 48];
            tmp.copy_from_slice(&bytes[48..96]);

            Fp::from_bytes(&tmp)
        };

        // Attempt to obtain the y-coordinate
        let yc1 = {
            let mut tmp = [0; 48];
            tmp.copy_from_slice(&bytes[96..144]);

            Fp::from_bytes(&tmp)
        };
        let yc0 = {
            let mut tmp = [0; 48];
            tmp.copy_from_slice(&bytes[144..192]);

            Fp::from_bytes(&tmp)
        };

        xc1.and_then(|xc1| {
            xc0.and_then(|xc0| {
                yc1.and_then(|yc1| {
                    yc0.and_then(|yc0| {
                        let x = Fp2 {
                            c0: xc0,
                            c1: xc1
                        };
                        let y = Fp2 {
                            c0: yc0,
                            c1: yc1
                        };

                        // Create a point representing this value
                        let p = G2Affine::conditional_select(
                            &G2Affine {
                                x,
                                y,
                                infinity: infinity_flag_set,
                            },
                            &G2Affine::identity(),
                            infinity_flag_set,
                        );

                        CtOption::new(
                            p,
                            // If the infinity flag is set, the x and y coordinates should have been zero.
                            ((!infinity_flag_set) | (infinity_flag_set & x.is_zero() & y.is_zero())) &
                            // The compression flag should not have been set, as this is an uncompressed element
                            (!compression_flag_set) &
                            // The sort flag should not have been set, as this is an uncompressed element
                            (!sort_flag_set),
                        )
                    })
                })
            })
        })
    }

    /// Attempts to deserialize a compressed element. See [`notes::serialization`](crate::notes::serialization)
    /// for details about how group elements are serialized.
    pub fn from_compressed_vartime(bytes: &[u8; 96]) -> Option<Self> {
        // We already know the point is on the curve because this is established
        // by the y-coordinate recovery procedure in from_compressed_unchecked().

        Self::from_compressed_unchecked_vartime(bytes).and_then(|p| 
            match bool::from(p.is_torsion_free()) {
                true => Some(p),
                _ => None,
            }
        )
    }

    /// Attempts to deserialize an uncompressed element, not checking if the
    /// element is in the correct subgroup.
    /// **This is dangerous to call unless you trust the bytes you are reading; otherwise,
    /// API invariants may be broken.** Please consider using `from_compressed()` instead.
    pub fn from_compressed_unchecked_vartime(bytes: &[u8; 96]) -> Option<Self> {
        // Obtain the three flags from the start of the byte sequence
        let compression_flag_set = Choice::from((bytes[0] >> 7) & 1);
        let infinity_flag_set = Choice::from((bytes[0] >> 6) & 1);
        let sort_flag_set = Choice::from((bytes[0] >> 5) & 1);

        // Attempt to obtain the x-coordinate
        let xc1 = {
            let mut tmp = [0; 48];
            tmp.copy_from_slice(&bytes[0..48]);

            // Mask away the flag bits
            tmp[0] &= 0b0001_1111;

            Fp::from_bytes(&tmp)
        };
        let xc0 = {
            let mut tmp = [0; 48];
            tmp.copy_from_slice(&bytes[48..96]);

            Fp::from_bytes(&tmp)
        };

        match bool::from(xc0.is_some()) {
            false => None,
            _ => 
            {
                match bool::from(xc1.is_some()) {
                    false => None,
                    _ =>
                    {
                        let xc0 = xc0.unwrap();
                        let xc1 = xc1.unwrap();
                        let x = Fp2 { c0: xc0, c1: xc1 };

                        // If the infinity flag is set, return the value assuming
                        // the x-coordinate is zero and the sort bit is not set.
                        //
                        // Otherwise, return a recovered point (assuming the correct
                        // y-coordinate can be found) so long as the infinity flag
                        // was not set.
                        match bool::from(infinity_flag_set & // Infinity flag should be set
                            compression_flag_set & // Compression flag should be set
                            (!sort_flag_set) & // Sort flag should not be set
                            x.is_zero(), // The x-coordinate should be zero
                        )
                        {
                            true => Some(G2Affine::identity()),
                            _ => None,
                        }.or_else(|| {
                            // Recover a y-coordinate given x by y = sqrt_vartime(x^3 + 4)
                            ((x.square() * x) + B).sqrt_vartime().and_then(|y| {
                                // Switch to the correct y-coordinate if necessary.
                                let y = Fp2::conditional_select(
                                    &y,
                                    &-y,
                                    y.lexicographically_largest() ^ sort_flag_set,
                                );

                                match bool::from((!infinity_flag_set) & compression_flag_set) {
                                    true => Some(G2Affine { x, y, infinity: infinity_flag_set, }),
                                    _ => None,
                                }
                            })
                        })
                    }
                }
            }
        }
    }

    /// Returns true if this element is the identity (the point at infinity).
    #[inline]
    pub fn is_identity(&self) -> Choice {
        self.infinity
    }

    /// Returns true if this point is free of an $h$-torsion component, and so it
    /// exists within the $q$-order subgroup $\mathbb{G}_2$. This should always return true
    /// unless an "unchecked" API was used.
    pub fn is_torsion_free(&self) -> Choice {
        const FQ_MODULUS_BYTES: [u8; 32] = [
    1, 0, 0, 0, 0, 128, 17, 10, 1, 0, 0, 208, 254, 118, 170, 89, 1, 176, 55, 92, 30, 77, 180, 96, 86, 165, 44, 154, 94, 101, 171, 18
        ];

        // Clear the r-torsion from the point and check if it is the identity
        G2Projective::from(*self)
            .multiply(&FQ_MODULUS_BYTES)
            .is_identity()
    }

    /// Returns true if this point is on the curve. This should always return
    /// true unless an "unchecked" API was used.
    pub fn is_on_curve(&self) -> Choice {
        // y^2 - x^3 ?= 4(u + 1)
        (self.y.square() - (self.x.square() * self.x)).ct_eq(&B) | self.infinity
    }
}

/// This is an element of $\mathbb{G}_2$ represented in the projective coordinate space.
#[derive(Copy, Clone, Debug)]
pub struct G2Projective {
    pub(crate) x: Fp2,
    pub(crate) y: Fp2,
    pub(crate) z: Fp2,
}

impl<'a> From<&'a G2Affine> for G2Projective {
    fn from(p: &'a G2Affine) -> G2Projective {
        G2Projective {
            x: p.x,
            y: p.y,
            z: Fp2::conditional_select(&Fp2::one(), &Fp2::zero(), p.infinity),
        }
    }
}

impl From<G2Affine> for G2Projective {
    fn from(p: G2Affine) -> G2Projective {
        G2Projective::from(&p)
    }
}

impl ConstantTimeEq for G2Projective {
    fn ct_eq(&self, other: &Self) -> Choice {
        // Is (xz^2, yz^3, z) equal to (x'z'^2, yz'^3, z') when converted to affine?

        let z = other.z.square();
        let x1 = self.x * z;
        let z = z * other.z;
        let y1 = self.y * z;
        let z = self.z.square();
        let x2 = other.x * z;
        let z = z * self.z;
        let y2 = other.y * z;

        let self_is_zero = self.z.is_zero();
        let other_is_zero = other.z.is_zero();

        (self_is_zero & other_is_zero) // Both point at infinity
            | ((!self_is_zero) & (!other_is_zero) & x1.ct_eq(&x2) & y1.ct_eq(&y2)) // Neither point at infinity, coordinates are the same
    }
}

impl ConditionallySelectable for G2Projective {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        G2Projective {
            x: Fp2::conditional_select(&a.x, &b.x, choice),
            y: Fp2::conditional_select(&a.y, &b.y, choice),
            z: Fp2::conditional_select(&a.z, &b.z, choice),
        }
    }
}

impl Eq for G2Projective {}
impl PartialEq for G2Projective {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        bool::from(self.ct_eq(other))
    }
}

impl<'a> Neg for &'a G2Projective {
    type Output = G2Projective;

    #[inline]
    fn neg(self) -> G2Projective {
        G2Projective {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}

impl Neg for G2Projective {
    type Output = G2Projective;

    #[inline]
    fn neg(self) -> G2Projective {
        -&self
    }
}

impl<'a, 'b> Add<&'b G2Projective> for &'a G2Projective {
    type Output = G2Projective;

    #[inline]
    fn add(self, rhs: &'b G2Projective) -> G2Projective {
        self.add(rhs)
    }
}

impl<'a, 'b> Sub<&'b G2Projective> for &'a G2Projective {
    type Output = G2Projective;

    #[inline]
    fn sub(self, rhs: &'b G2Projective) -> G2Projective {
        self + (-rhs)
    }
}

impl<'a, 'b> Mul<&'b Scalar> for &'a G2Projective {
    type Output = G2Projective;

    fn mul(self, other: &'b Scalar) -> Self::Output {
        self.multiply(&other.to_bytes())
    }
}

impl<'a, 'b> Mul<&'b Scalar> for &'a G2Affine {
    type Output = G2Projective;

    fn mul(self, other: &'b Scalar) -> Self::Output {
        G2Projective::from(self).multiply(&other.to_bytes())
    }
}

impl_binops_additive!(G2Projective, G2Projective);
impl_binops_multiplicative!(G2Projective, Scalar);
impl_binops_multiplicative_mixed!(G2Affine, Scalar, G2Projective);

impl G2Projective {
    /// Returns the identity of the group: the point at infinity.
    pub fn identity() -> G2Projective {
        G2Projective {
            x: Fp2::zero(),
            y: Fp2::one(),
            z: Fp2::zero(),
        }
    }

    /// Returns a fixed generator of the group. See [`notes::design`](notes/design/index.html#fixed-generators)
    /// for how this generator is chosen.
    pub fn generator() -> G2Projective {
        G2Projective {
            x: G2_GEN_X,  
            y: G2_GEN_Y,
            z: Fp2::one(),
        }
    }

    /// Computes the doubling of this point.
    pub fn double(&self) -> G2Projective {
        // http://www.hyperelliptic.org/EFD/g2p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l
        //
        // There are no points of order 2.
        // TODO: Is this true for 377?

        let a = self.x.square();
        let b = self.y.square();
        let c = b.square();
        let d = self.x + b;
        let d = d.square();
        let d = d - a - c;
        let d = d + d;
        let e = a + a + a;
        let f = e.square();
        let z3 = self.z * self.y;
        let z3 = z3 + z3;
        let x3 = f - (d + d);
        let c = c + c;
        let c = c + c;
        let c = c + c;
        let y3 = e * (d - x3) - c;

        let tmp = G2Projective {
            x: x3,
            y: y3,
            z: z3,
        };

        G2Projective::conditional_select(&tmp, &G2Projective::identity(), self.is_identity())
    }

    /// Adds this point to another point.
    pub fn add(&self, rhs: &G2Projective) -> G2Projective {
        // This Jacobian point addition technique is based on the implementation in libsecp256k1,
        // which assumes that rhs has z=1. Let's address the case of zero z-coordinates generally.

        // If self is the identity, return rhs. Otherwise, return self. The other cases will be
        // predicated on neither self nor rhs being the identity.
        let f1 = self.is_identity();
        let res = G2Projective::conditional_select(self, rhs, f1);
        let f2 = rhs.is_identity();

        // If neither are the identity but x1 = x2 and y1 != y2, then return the identity
        let z = rhs.z.square();
        let u1 = self.x * z;
        let z = z * rhs.z;
        let s1 = self.y * z;
        let z = self.z.square();
        let u2 = rhs.x * z;
        let z = z * self.z;
        let s2 = rhs.y * z;
        let f3 = u1.ct_eq(&u2) & (!s1.ct_eq(&s2));
        let res =
            G2Projective::conditional_select(&res, &G2Projective::identity(), (!f1) & (!f2) & f3);

        let t = u1 + u2;
        let m = s1 + s2;
        let rr = t.square();
        let m_alt = -u2;
        let tt = u1 * m_alt;
        let rr = rr + tt;

        // Correct for x1 != x2 but y1 = -y2, which can occur because p - 1 is divisible by 3.
        // libsecp256k1 does this by substituting in an alternative (defined) expression for lambda.
        let degenerate = m.is_zero() & rr.is_zero();
        let rr_alt = s1 + s1;
        let m_alt = m_alt + u1;
        let rr_alt = Fp2::conditional_select(&rr_alt, &rr, !degenerate);
        let m_alt = Fp2::conditional_select(&m_alt, &m, !degenerate);

        let n = m_alt.square();
        let q = n * t;

        let n = n.square();
        let n = Fp2::conditional_select(&n, &m, degenerate);
        let t = rr_alt.square();
        let z3 = m_alt * self.z * rhs.z; // We allow rhs.z != 1, so we must account for this.
        let z3 = z3 + z3;
        let q = -q;
        let t = t + q;
        let x3 = t;
        let t = t + t;
        let t = t + q;
        let t = t * rr_alt;
        let t = t + n;
        let y3 = -t;
        let x3 = x3 + x3;
        let x3 = x3 + x3;
        let y3 = y3 + y3;
        let y3 = y3 + y3;

        let tmp = G2Projective {
            x: x3,
            y: y3,
            z: z3,
        };

        G2Projective::conditional_select(&res, &tmp, (!f1) & (!f2) & (!f3))
    }

    /// Adds this point to another point in the affine model.
    pub fn add_mixed(&self, rhs: &G2Affine) -> G2Projective {
        // This Jacobian point addition technique is based on the implementation in libsecp256k1,
        // which assumes that rhs has z=1. Let's address the case of zero z-coordinates generally.

        // If self is the identity, return rhs. Otherwise, return self. The other cases will be
        // predicated on neither self nor rhs being the identity.
        let f1 = self.is_identity();
        let res = G2Projective::conditional_select(self, &G2Projective::from(rhs), f1);
        let f2 = rhs.is_identity();

        // If neither are the identity but x1 = x2 and y1 != y2, then return the identity
        let u1 = self.x;
        let s1 = self.y;
        let z = self.z.square();
        let u2 = rhs.x * z;
        let z = z * self.z;
        let s2 = rhs.y * z;
        let f3 = u1.ct_eq(&u2) & (!s1.ct_eq(&s2));
        let res =
            G2Projective::conditional_select(&res, &G2Projective::identity(), (!f1) & (!f2) & f3);

        let t = u1 + u2;
        let m = s1 + s2;
        let rr = t.square();
        let m_alt = -u2;
        let tt = u1 * m_alt;
        let rr = rr + tt;

        // Correct for x1 != x2 but y1 = -y2, which can occur because p - 1 is divisible by 3.
        // libsecp256k1 does this by substituting in an alternative (defined) expression for lambda.
        let degenerate = m.is_zero() & rr.is_zero();
        let rr_alt = s1 + s1;
        let m_alt = m_alt + u1;
        let rr_alt = Fp2::conditional_select(&rr_alt, &rr, !degenerate);
        let m_alt = Fp2::conditional_select(&m_alt, &m, !degenerate);

        let n = m_alt.square();
        let q = n * t;

        let n = n.square();
        let n = Fp2::conditional_select(&n, &m, degenerate);
        let t = rr_alt.square();
        let z3 = m_alt * self.z;
        let z3 = z3 + z3;
        let q = -q;
        let t = t + q;
        let x3 = t;
        let t = t + t;
        let t = t + q;
        let t = t * rr_alt;
        let t = t + n;
        let y3 = -t;
        let x3 = x3 + x3;
        let x3 = x3 + x3;
        let y3 = y3 + y3;
        let y3 = y3 + y3;

        let tmp = G2Projective {
            x: x3,
            y: y3,
            z: z3,
        };

        G2Projective::conditional_select(&res, &tmp, (!f1) & (!f2) & (!f3))
    }

    fn multiply(&self, by: &[u8; 32]) -> G2Projective {
        let mut acc = G2Projective::identity();

        // This is a simple double-and-add implementation of point
        // multiplication, moving from most significant to least
        // significant bit of the scalar.
        //
        // We skip the leading bit because it's always unset for Fq
        // elements.
        for bit in by
            .iter()
            .rev()
            .flat_map(|byte| (0..8).rev().map(move |i| Choice::from((byte >> i) & 1u8)))
            .skip(1)
        {
            acc = acc.double();
            acc = G2Projective::conditional_select(&acc, &(acc + self), bit);
        }

        acc
    }

    /// Converts a batch of `G2Projective` elements into `G2Affine` elements. This
    /// function will panic if `p.len() != q.len()`.
    pub fn batch_normalize(p: &[Self], q: &mut [G2Affine]) {
        assert_eq!(p.len(), q.len());

        let mut acc = Fp2::one();
        for (p, q) in p.iter().zip(q.iter_mut()) {
            // We use the `x` field of `G2Affine` to store the product
            // of previous z-coordinates seen.
            q.x = acc;

            // We will end up skipping all identities in p
            acc = Fp2::conditional_select(&(acc * p.z), &acc, p.is_identity());
        }

        // This is the inverse, as all z-coordinates are nonzero and the ones
        // that are not are skipped.
        acc = acc.invert().unwrap();

        for (p, q) in p.iter().rev().zip(q.iter_mut().rev()) {
            let skip = p.is_identity();

            // Compute tmp = 1/z
            let tmp = q.x * acc;

            // Cancel out z-coordinate in denominator of `acc`
            acc = Fp2::conditional_select(&(acc * p.z), &acc, skip);

            // Set the coordinates to the correct value
            let tmp2 = tmp.square();
            let tmp3 = tmp2 * tmp;

            q.x = p.x * tmp2;
            q.y = p.y * tmp3;
            q.infinity = Choice::from(0u8);

            *q = G2Affine::conditional_select(&q, &G2Affine::identity(), skip);
        }
    }

    /// Returns true if this element is the identity (the point at infinity).
    #[inline]
    pub fn is_identity(&self) -> Choice {
        self.z.is_zero()
    }

    /// Returns true if this point is on the curve. This should always return
    /// true unless an "unchecked" API was used.
    pub fn is_on_curve(&self) -> Choice {
        // Y^2 - X^3 = 4(u + 1)(Z^6)

        (self.y.square() - (self.x.square() * self.x))
            .ct_eq(&((self.z.square() * self.z).square() * B))
            | self.z.is_zero()
    }
}

#[test]
fn test_is_on_curve() {
    assert!(bool::from(G2Affine::identity().is_on_curve()));
    assert!(bool::from(G2Affine::generator().is_on_curve()));
    assert!(bool::from(G2Projective::identity().is_on_curve()));
    assert!(bool::from(G2Projective::generator().is_on_curve()));

    let mut pt = G2Projective::generator();
    for _i in 0..100 {
        pt = pt.double();
        assert!(bool::from(pt.is_on_curve()));
    }
}

#[test]
fn test_affine_point_equality() {
    let a = G2Affine::generator();
    let b = G2Affine::identity();

    assert!(a == a);
    assert!(b == b);
    assert!(a != b);
    assert!(b != a);
}

#[test]
fn test_projective_point_equality() {
    let a = G2Projective::generator();
    let b = G2Projective::identity();

    assert!(a == a);
    assert!(b == b);
    assert!(a != b);
    assert!(b != a);

    let a2 = a.double();
    let a4 = a2.double();
    let a4_prime = a2 + a + a;
    assert_eq!(a4, a4_prime);
}

#[test]
fn test_conditionally_select_affine() {
    let a = G2Affine::generator();
    let b = G2Affine::identity();

    assert_eq!(G2Affine::conditional_select(&a, &b, Choice::from(0u8)), a);
    assert_eq!(G2Affine::conditional_select(&a, &b, Choice::from(1u8)), b);
}

#[test]
fn test_conditionally_select_projective() {
    let a = G2Projective::generator();
    let b = G2Projective::identity();

    assert_eq!(
        G2Projective::conditional_select(&a, &b, Choice::from(0u8)),
        a
    );
    assert_eq!(
        G2Projective::conditional_select(&a, &b, Choice::from(1u8)),
        b
    );
}

#[test]
fn test_projective_to_affine() {
    let a = G2Projective::generator();
    let b = G2Projective::identity();

    assert!(bool::from(G2Affine::from(a).is_on_curve()));
    assert!(!bool::from(G2Affine::from(a).is_identity()));
    let b = G2Affine::from(b);
    assert!(bool::from(b.is_on_curve()));
    assert!(bool::from(G2Affine::from(b).is_identity()));
}

#[test]
fn test_affine_to_projective() {
    let a = G2Affine::generator();
    let b = G2Affine::identity();

    assert!(bool::from(G2Projective::from(a).is_on_curve()));
    assert!(!bool::from(G2Projective::from(a).is_identity()));
    assert!(bool::from(G2Projective::from(b).is_on_curve()));
    assert!(bool::from(G2Projective::from(b).is_identity()));
}

#[test]
fn test_doubling() {
    {
        let tmp = G2Projective::identity().double();
        assert!(bool::from(tmp.is_identity()));
        assert!(bool::from(tmp.is_on_curve()));
    }
    {
        let tmp = G2Projective::generator().double();
        assert!(!bool::from(tmp.is_identity()));
        assert!(bool::from(tmp.is_on_curve()));

        assert_eq!(
            G2Affine::from(tmp),
            G2Affine {
                x: Fp2 {
                    c0: Fp::from_raw_unchecked([
                        0xb7bb9a26fb72697a, 
                        0x59e88dfa302f5e0c, 
                        0x89849dc62d07d624, 
                        0x83d4c44ae291b4ba, 
                        0x6b7eaf596b73a0c5, 
                        0xdda97ba8280944,
                    ]),
                    c1: Fp::from_raw_unchecked([
                        0x1b6a166ce673cd07, 
                        0x31e0c506cb0db98f, 
                        0x18cb95a5acb8df95, 
                        0xcc39f4a3d83cab36, 
                        0x5312c87ab58e6344, 
                        0xe7508915955a38,
                    ]),
                },
                y: Fp2 {
                    c0: Fp::from_raw_unchecked([
                        0x376c611156c8bce1, 
                        0xa277d39ac4448ef, 
                        0x15c841eafd60bc7a, 
                        0x6273ecbddcafead0, 
                        0x8b19ec1a24a43c0b, 
                        0x12402b7b65e9c74,
                    ]),
                    c1: Fp::from_raw_unchecked([
                        0x9c85713196b4d134, 
                        0xf1c7f3ebe43a28d2, 
                        0xffbadf706f7094da, 
                        0x9ad1c308f942a59a, 
                        0xfb1decdcb0423bbf, 
                        0x168a86463e56bee,
                    ]),
                },
                infinity: Choice::from(0u8)
            }
        );
    }
}

#[test]
fn test_projective_addition() {
    {
        let a = G2Projective::identity();
        let b = G2Projective::identity();
        let c = a + b;
        assert!(bool::from(c.is_identity()));
        assert!(bool::from(c.is_on_curve()));
    }
    {
        let a = G2Projective::generator().double().double(); // 4P
        let b = G2Projective::generator().double(); // 2P
        let c = a + b;

        let mut d = G2Projective::generator();
        for _ in 0..5 {
            d = d + G2Projective::generator();
        }
        assert!(!bool::from(c.is_identity()));
        assert!(bool::from(c.is_on_curve()));
        assert!(!bool::from(d.is_identity()));
        assert!(bool::from(d.is_on_curve()));
        assert_eq!(c, d);
    }

    // Degenerate case
   /* {
        let beta = Fp2 {
            c0: Fp::from_raw_unchecked([
                0xcd03c9e48671f071,
                0x5dab22461fcda5d2,
                0x587042afd3851b95,
                0x8eb60ebe01bacb9e,
                0x3f97d6e83d050d2,
                0x18f0206554638741,
            ]),
            c1: Fp::zero(),
        };
        let beta = beta.square();
        let a = G2Projective::generator().double().double();
        let b = G2Projective {
            x: a.x * beta,
            y: -a.y,
            z: a.z,
        };
        assert!(bool::from(a.is_on_curve()));
        assert!(bool::from(b.is_on_curve()));

        let c = a + b;
        assert_eq!(
            G2Affine::from(c),
            G2Affine::from(G2Projective {
                x: Fp2 {
                    c0: Fp::from_raw_unchecked([
                        0x705abc799ca773d3,
                        0xfe132292c1d4bf08,
                        0xf37ece3e07b2b466,
                        0x887e1c43f447e301,
                        0x1e0970d033bc77e8,
                        0x1985c81e20a693f2
                    ]),
                    c1: Fp::from_raw_unchecked([
                        0x1d79b25db36ab924,
                        0x23948e4d529639d3,
                        0x471ba7fb0d006297,
                        0x2c36d4b4465dc4c0,
                        0x82bbc3cfec67f538,
                        0x51d2728b67bf952
                    ])
                },
                y: Fp2 {
                    c0: Fp::from_raw_unchecked([
                        0x41b1bbf6576c0abf,
                        0xb6cc93713f7a0f9a,
                        0x6b65b43e48f3f01f,
                        0xfb7a4cfcaf81be4f,
                        0x3e32dadc6ec22cb6,
                        0xbb0fc49d79807e3
                    ]),
                    c1: Fp::from_raw_unchecked([
                        0x7d1397788f5f2ddf,
                        0xab2907144ff0d8e8,
                        0x5b7573e0cdb91f92,
                        0x4cb8932dd31daf28,
                        0x62bbfac6db052a54,
                        0x11f95c16d14c3bbe
                    ])
                },
                z: Fp2::one()
            })
        );
        assert!(!bool::from(c.is_identity()));
        assert!(bool::from(c.is_on_curve()));
    }*/
}

#[test]
fn test_mixed_addition() {
    {
        let a = G2Affine::identity();
        let b = G2Projective::identity();
        let c = a + b;
        assert!(bool::from(c.is_identity()));
        assert!(bool::from(c.is_on_curve()));
    }
    {
        let a = G2Projective::generator().double().double(); // 4P
        let b = G2Projective::generator().double(); // 2P
        let c = a + b;

        let mut d = G2Projective::generator();
        for _ in 0..5 {
            d = d + G2Affine::generator();
        }
        assert!(!bool::from(c.is_identity()));
        assert!(bool::from(c.is_on_curve()));
        assert!(!bool::from(d.is_identity()));
        assert!(bool::from(d.is_on_curve()));
        assert_eq!(c, d);
    }

    // Degenerate case
    // TODO: Adapt for bls-377
   /* {
        let beta = Fp2 {
            c0: Fp::from_raw_unchecked([
                0xcd03c9e48671f071,
                0x5dab22461fcda5d2,
                0x587042afd3851b95,
                0x8eb60ebe01bacb9e,
                0x3f97d6e83d050d2,
                0x18f0206554638741,
            ]),
            c1: Fp::zero(),
        };
        let beta = beta.square();
        let a = G2Projective::generator().double().double();
        let b = G2Projective {
            x: a.x * beta,
            y: -a.y,
            z: a.z,
        };
        let a = G2Affine::from(a);
        assert!(bool::from(a.is_on_curve()));
        assert!(bool::from(b.is_on_curve()));

        let c = a + b;
        assert_eq!(
            G2Affine::from(c),
            G2Affine::from(G2Projective {
                x: Fp2 {
                    c0: Fp::from_raw_unchecked([
                        0x705abc799ca773d3,
                        0xfe132292c1d4bf08,
                        0xf37ece3e07b2b466,
                        0x887e1c43f447e301,
                        0x1e0970d033bc77e8,
                        0x1985c81e20a693f2
                    ]),
                    c1: Fp::from_raw_unchecked([
                        0x1d79b25db36ab924,
                        0x23948e4d529639d3,
                        0x471ba7fb0d006297,
                        0x2c36d4b4465dc4c0,
                        0x82bbc3cfec67f538,
                        0x51d2728b67bf952
                    ])
                },
                y: Fp2 {
                    c0: Fp::from_raw_unchecked([
                        0x41b1bbf6576c0abf,
                        0xb6cc93713f7a0f9a,
                        0x6b65b43e48f3f01f,
                        0xfb7a4cfcaf81be4f,
                        0x3e32dadc6ec22cb6,
                        0xbb0fc49d79807e3
                    ]),
                    c1: Fp::from_raw_unchecked([
                        0x7d1397788f5f2ddf,
                        0xab2907144ff0d8e8,
                        0x5b7573e0cdb91f92,
                        0x4cb8932dd31daf28,
                        0x62bbfac6db052a54,
                        0x11f95c16d14c3bbe
                    ])
                },
                z: Fp2::one()
            })
        );
        assert!(!bool::from(c.is_identity()));
        assert!(bool::from(c.is_on_curve()));
    }*/
}

#[test]
fn test_projective_negation_and_subtraction() {
    let a = G2Projective::generator().double();
    assert_eq!(a + (-a), G2Projective::identity());
    assert_eq!(a + (-a), a - a);
}

#[test]
fn test_affine_negation_and_subtraction() {
    let a = G2Affine::generator();
    assert_eq!(G2Projective::from(a) + (-a), G2Projective::identity());
    assert_eq!(G2Projective::from(a) + (-a), G2Projective::from(a) - a);
}

#[test]
fn test_projective_scalar_multiplication() {
    let g = G2Projective::generator();
    let a = Scalar::from_raw([
        0x2b568297a56da71c,
        0xd8c39ecb0ef375d1,
        0x435c38da67bfbf96,
        0x8088a05026b659b2,
    ]);
    let b = Scalar::from_raw([
        0x785fdd9b26ef8b85,
        0xc997f25837695c18,
        0x4c8dbc39e7b756c1,
        0x70d9b6cc6d87df20,
    ]);
    let c = a * b;

    assert_eq!((g * a) * b, g * c);
}

#[test]
fn test_affine_scalar_multiplication() {
    let g = G2Affine::generator();
    let a = Scalar::from_raw([
        0x2b568297a56da71c,
        0xd8c39ecb0ef375d1,
        0x435c38da67bfbf96,
        0x8088a05026b659b2,
    ]);
    let b = Scalar::from_raw([
        0x785fdd9b26ef8b85,
        0xc997f25837695c18,
        0x4c8dbc39e7b756c1,
        0x70d9b6cc6d87df20,
    ]);
    let c = a * b;

    assert_eq!(G2Affine::from(g * a) * b, g * c);
}

#[test]
fn test_is_torsion_free() {
    let a = G2Affine {
        x: Fp2 {
            c0: Fp::from_raw_unchecked([
                0x89f550c813db6431,
                0xa50be8c456cd8a1a,
                0xa45b374114cae851,
                0xbb6190f5bf7fff63,
                0x970ca02c3ba80bc7,
                0x2b85d24e840fbac,
            ]),
            c1: Fp::from_raw_unchecked([
                0x6888bc53d70716dc,
                0x3dea6b4117682d70,
                0xd8f5f930500ca354,
                0x6b5ecb6556f5c155,
                0xc96bef0434778ab0,
                0x5081505515006ad,
            ]),
        },
        y: Fp2 {
            c0: Fp::from_raw_unchecked([
                0x3cf1ea0d434b0f40,
                0x1a0dc610e603e333,
                0x7f89956160c72fa0,
                0x25ee03decf6431c5,
                0xeee8e206ec0fe137,
                0x97592b226dfef28,
            ]),
            c1: Fp::from_raw_unchecked([
                0x71e8bb5f29247367,
                0xa5fe049e211831ce,
                0xce6b354502a3896,
                0x93b012000997314e,
                0x6759f3b6aa5b42ac,
                0x156944c4dfe92bbb,
            ]),
        },
        infinity: Choice::from(0u8),
    };
    assert!(!bool::from(a.is_torsion_free()));

    assert!(bool::from(G2Affine::identity().is_torsion_free()));
    assert!(bool::from(G2Affine::generator().is_torsion_free()));
}

#[test]
fn test_batch_normalize() {
    let a = G2Projective::generator().double();
    let b = a.double();
    let c = b.double();

    for a_identity in (0..1).map(|n| n == 1) {
        for b_identity in (0..1).map(|n| n == 1) {
            for c_identity in (0..1).map(|n| n == 1) {
                let mut v = [a, b, c];
                if a_identity {
                    v[0] = G2Projective::identity()
                }
                if b_identity {
                    v[1] = G2Projective::identity()
                }
                if c_identity {
                    v[2] = G2Projective::identity()
                }

                let mut t = [
                    G2Affine::identity(),
                    G2Affine::identity(),
                    G2Affine::identity(),
                ];
                let expected = [
                    G2Affine::from(v[0]),
                    G2Affine::from(v[1]),
                    G2Affine::from(v[2]),
                ];

                G2Projective::batch_normalize(&v[..], &mut t[..]);

                assert_eq!(&t[..], &expected[..]);
            }
        }
    }
}
