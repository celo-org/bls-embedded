//! This module implements arithmetic over the quadratic extension field Fp2.

use core::fmt;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::fp::{Fp, MODULUS};

//TODO: Is this actually -5?
const NONRESIDUE: Fp = Fp::from_raw_unchecked([
    0xfc0b8000000002fa,
    0x97d39cf6e000018b,
    0x2072420fbfa05044,
    0xcbbcbd50d97c3802,
    0xbaf1ec35813f9eb,
    0x9974a2c0945ad2,
]);

// from Zexe Bls-377 fp2 implementation
const QUADRATIC_NONRESIDUE: Fp2 = Fp2 {
    c0: Fp::zero(),
    c1: Fp::from_raw_unchecked([
            202099033278250856u64,
            5854854902718660529u64,
            11492539364873682930u64,
            8885205928937022213u64,
            5545221690922665192u64,
            39800542322357402u64,
    ]),
};

#[derive(Copy, Clone)]
pub struct Fp2 {
    pub c0: Fp,
    pub c1: Fp,
}

impl fmt::Debug for Fp2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} + {:?}*u", self.c0, self.c1)
    }
}

impl Default for Fp2 {
    fn default() -> Self {
        Fp2::zero()
    }
}

impl From<Fp> for Fp2 {
    fn from(f: Fp) -> Fp2 {
        Fp2 {
            c0: f,
            c1: Fp::zero(),
        }
    }
}

impl ConstantTimeEq for Fp2 {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.c0.ct_eq(&other.c0) & self.c1.ct_eq(&other.c1)
    }
}

impl Eq for Fp2 {}
impl PartialEq for Fp2 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).unwrap_u8() == 1
    }
}

impl ConditionallySelectable for Fp2 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Fp2 {
            c0: Fp::conditional_select(&a.c0, &b.c0, choice),
            c1: Fp::conditional_select(&a.c1, &b.c1, choice),
        }
    }
}

impl<'a> Neg for &'a Fp2 {
    type Output = Fp2;

    #[inline]
    fn neg(self) -> Fp2 {
        self.neg()
    }
}

impl Neg for Fp2 {
    type Output = Fp2;

    #[inline]
    fn neg(self) -> Fp2 {
        -&self
    }
}

impl<'a, 'b> Sub<&'b Fp2> for &'a Fp2 {
    type Output = Fp2;

    #[inline]
    fn sub(self, rhs: &'b Fp2) -> Fp2 {
        self.sub(rhs)
    }
}

impl<'a, 'b> Add<&'b Fp2> for &'a Fp2 {
    type Output = Fp2;

    #[inline]
    fn add(self, rhs: &'b Fp2) -> Fp2 {
        self.add(rhs)
    }
}

impl<'a, 'b> Mul<&'b Fp2> for &'a Fp2 {
    type Output = Fp2;

    #[inline]
    fn mul(self, rhs: &'b Fp2) -> Fp2 {
        self.mul(rhs)
    }
}

impl_binops_additive!(Fp2, Fp2);
impl_binops_multiplicative!(Fp2, Fp2);

impl Fp2 {
    #[inline]
    pub const fn zero() -> Fp2 {
        Fp2 {
            c0: Fp::zero(),
            c1: Fp::zero(),
        }
    }

    #[inline]
    pub const fn one() -> Fp2 {
        Fp2 {
            c0: Fp::one(),
            c1: Fp::zero(),
        }
    }

    pub fn is_zero(&self) -> Choice {
        self.c0.is_zero() & self.c1.is_zero()
    }

    /// Raises this element to p.
    #[inline(always)]
    pub fn frobenius_map(&self) -> Self {
        // This is always just a conjugation. If you're curious why, here's
        // an article about it: https://alicebob.cryptoland.net/the-frobenius-endomorphism-with-finite-fields/
        self.conjugate()
    }

    #[inline(always)]
    pub fn conjugate(&self) -> Self {
        Fp2 {
            c0: self.c0,
            c1: -self.c1,
        }
    }

    #[inline(always)]
    pub fn mul_by_nonresidue(&self) -> Fp2 {
        // Multiply a + bu by u + 1, getting
        // au + a + bu^2 + bu
        // and because u^2 = -1, we get
        // (a - b) + (a + b)u

        Fp2 {
            c0: self.c0 + (NONRESIDUE * self.c1),
            c1: self.c0 + self.c1,
        }
    }

    /// Returns whether or not this element is strictly lexicographically
    /// larger than its negation.
    #[inline]
    pub fn lexicographically_largest(&self) -> Choice {
        // If this element's c1 coefficient is lexicographically largest
        // then it is lexicographically largest. Otherwise, in the event
        // the c1 coefficient is zero and the c0 coefficient is
        // lexicographically largest, then this element is lexicographically
        // largest.

        self.c1.lexicographically_largest()
            | (self.c1.is_zero() & self.c0.lexicographically_largest())
    }

    //TODO: Fix for 377
    pub const fn square(&self) -> Fp2 {
        // Complex squaring:
        //
        // v0  = c0 * c1
        // c0' = (c0 + c1) * (c0 + \beta*c1) - v0 - \beta * v0
        // c1' = 2 * v0
        //
        // In BLS12-381's F_{p^2}, our \beta is -1 so we
        // can modify this formula:
        //
        // c0' = (c0 + c1) * (c0 - c1)
        // c1' = 2 * c0 * c1

        let v0 = Fp::mul(&self.c0, &self.c1);
        let a = (&self.c0).add(&self.c1);
        let b = Fp::add(&self.c0, &(Fp::mul(&NONRESIDUE, &self.c1)));
        let c = Fp::mul(&Fp::add(&NONRESIDUE, &Fp::one()), &v0);

        Fp2 {
            c0: (&a).mul(&b),
            c1: (&c).mul(&self.c1),
        }
    }

    pub const fn mul(&self, rhs: &Fp2) -> Fp2 {
        // Karatsuba multiplication:
        //
        // v0  = a0 * b0
        // v1  = a1 * b1
        // c0 = v0 + \beta * v1
        // c1 = (a0 + a1) * (b0 + b1) - v0 - v1
        //
        // In BLS12-381's F_{p^2}, our \beta is -1 so we
        // can modify this formula. (Also, since we always
        // subtract v1, we can compute v1 = -a1 * b1.)
        //
        // v0  = a0 * b0
        // v1  = (-a1) * b1
        // c0 = v0 + v1
        // c1 = (a0 + a1) * (b0 + b1) - v0 + v1

        let v0 = (&self.c0).mul(&rhs.c0);
        let v1 = (&self.c1).mul(&rhs.c1);
        let c0 = (&v0).add(&(&NONRESIDUE).mul(&v1));
        let c1 = (&(&self.c0).add(&self.c1)).mul(&(&rhs.c0).add(&rhs.c1));
        let c1 = (&c1).sub(&v0);
        let c1 = (&c1).sub(&v1);

        Fp2 { c0, c1 }
    }

    pub const fn add(&self, rhs: &Fp2) -> Fp2 {
        Fp2 {
            c0: (&self.c0).add(&rhs.c0),
            c1: (&self.c1).add(&rhs.c1),
        }
    }

    pub const fn sub(&self, rhs: &Fp2) -> Fp2 {
        Fp2 {
            c0: (&self.c0).sub(&rhs.c0),
            c1: (&self.c1).sub(&rhs.c1),
        }
    }

    pub const fn neg(&self) -> Fp2 {
        Fp2 {
            c0: (&self.c0).neg(),
            c1: (&self.c1).neg(),
        }
    }

    //TODO: Precompute e, f
    //TODO: Make constant time
    //TODO: Fix branching conditions
    // Algorithm 10, https://eprint.iacr.org/2012/685.pdf
    pub fn sqrt(&self) -> CtOption<Self> {
        use crate::CtOptionExt; 
        // Take a quadratic nonresidue c^((q - 1) // 2)
        let d = QUADRATIC_NONRESIDUE.pow_vartime(&[
            0xdcff7fffffffd555,
            0xf55ffff58a9ffff,
            0xb39869507b587b12,
            0xb23ba5c279c2895f,
            0x258dd3db21a5d66b,
            0xd0088f51cbff34d,
        ]);
        let dc = d * QUADRATIC_NONRESIDUE;
        let e = dc.invert().unwrap();
        let f = dc.square();
        // b = self^((q - 1) // 4)
        let b = self.pow_vartime(&[
            0x2142300000000000,
            0x05C2D7510C000000,
            0xC7BCD88BEE825200,
            0xC688B67CC03D44E3,
            0xB18EC1701B28524E,
            0x6B8E9185F1443A,
        ]);
        // a0 = b^(2q + 2)
        let b2 = b.square();
        let b2q = b2.pow_vartime(&MODULUS);
        let a0 = b2 * b2q;
        //if a0 == -1 return false
        let bq = b.pow_vartime(&MODULUS);
        let bqb = bq * b;
        if bqb == Fp2::one() {
            let qr = b2 * self;
            let x0 = Fp::sqrt(&qr.c0).unwrap();
            CtOption::new(Fp2::from(x0) * bq, 1.ct_eq(&1))
        } else {
            let qr = b2 * self * f;
            let x0 = Fp::sqrt(&qr.c0).unwrap();
            CtOption::new(Fp2::from(x0) * bq * e, 1.ct_eq(&1))
        }
    } 

    /// Computes the multiplicative inverse of this field
    /// element, returning None in the case that this element
    /// is zero.
    pub fn invert(&self) -> CtOption<Self> {
        // We wish to find the multiplicative inverse of a nonzero
        // element a + bu in Fp2. Taken from Zexe codebase: algorithm 5.19
        // of Guide to Pairing Based Cryptography
        
        let v0 = self.c0.square();
        let v1 = self.c1.square();
        let v0 = v0 - NONRESIDUE * v1;
        let v1 = v0.invert().unwrap();
        CtOption::new(Fp2 {
            c0: self.c0 * v1,
            c1: self.c1 * v1,
        }, 1.ct_eq(&1))
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
}

#[test]
fn test_conditional_selection() {
    let a = Fp2 {
        c0: Fp::from_raw_unchecked([1, 2, 3, 4, 5, 6]),
        c1: Fp::from_raw_unchecked([7, 8, 9, 10, 11, 12]),
    };
    let b = Fp2 {
        c0: Fp::from_raw_unchecked([13, 14, 15, 16, 17, 18]),
        c1: Fp::from_raw_unchecked([19, 20, 21, 22, 23, 24]),
    };

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
    fn is_equal(a: &Fp2, b: &Fp2) -> bool {
        let eq = a == b;
        let ct_eq = a.ct_eq(&b);

        assert_eq!(eq, ct_eq.unwrap_u8() == 1);

        eq
    }

    assert!(is_equal(
        &Fp2 {
            c0: Fp::from_raw_unchecked([1, 2, 3, 4, 5, 6]),
            c1: Fp::from_raw_unchecked([7, 8, 9, 10, 11, 12]),
        },
        &Fp2 {
            c0: Fp::from_raw_unchecked([1, 2, 3, 4, 5, 6]),
            c1: Fp::from_raw_unchecked([7, 8, 9, 10, 11, 12]),
        }
    ));

    assert!(!is_equal(
        &Fp2 {
            c0: Fp::from_raw_unchecked([2, 2, 3, 4, 5, 6]),
            c1: Fp::from_raw_unchecked([7, 8, 9, 10, 11, 12]),
        },
        &Fp2 {
            c0: Fp::from_raw_unchecked([1, 2, 3, 4, 5, 6]),
            c1: Fp::from_raw_unchecked([7, 8, 9, 10, 11, 12]),
        }
    ));

    assert!(!is_equal(
        &Fp2 {
            c0: Fp::from_raw_unchecked([1, 2, 3, 4, 5, 6]),
            c1: Fp::from_raw_unchecked([2, 8, 9, 10, 11, 12]),
        },
        &Fp2 {
            c0: Fp::from_raw_unchecked([1, 2, 3, 4, 5, 6]),
            c1: Fp::from_raw_unchecked([7, 8, 9, 10, 11, 12]),
        }
    ));
}

#[test]
fn test_squaring() {
    let a = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xc9a2183163ee70d4,
            0xbc3770a7196b5c91,
            0xa247f8c1304c5f44,
            0xb01fc2a3726c80b5,
            0xe1d293e5bbd919c9,
            0x4b78e80020ef2ca,
        ]),
        c1: Fp::from_raw_unchecked([
            0x952ea4460462618f,
            0x238d5eddf025c62f,
            0xf6c94b012ea92e72,
            0x3ce24eac1c93808,
            0x55950f945da483c,
            0x10a768d0df4eabc,
        ]),
    };
    //9180cfbd5231eb92, 80ba5cc15826ee06, 6e4810398ff8110a, 17b1565c3b5de972, adfa03c911c9f3d, 45616e22b1a459])), c1: Fp384(BigInteger384([de372dea33981b66, 235f7eb8baf88c85, 3837e2636f0d07bc, ba39294a74709e4b, 274cb0edb1fdd1e2, 11abe141195cea6
    let b = Fp2 {
        c0: Fp::from_raw_unchecked([
            0x9180cfbd5231eb92, 
            0x80ba5cc15826ee06, 
            0x6e4810398ff8110a, 
            0x17b1565c3b5de972, 
            0xadfa03c911c9f3d, 
            0x45616e22b1a459,
        ]),
        c1: Fp::from_raw_unchecked([
            0xde372dea33981b66, 
            0x235f7eb8baf88c85, 
            0x3837e2636f0d07bc, 
            0xba39294a74709e4b, 
            0x274cb0edb1fdd1e2, 
            0x11abe141195cea6,
        ]),
    };

    assert_eq!(a.square(), b);
}

#[test]
fn test_multiplication() {
    let a = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xc9a2183163ee70d4,
            0xbc3770a7196b5c91,
            0xa247f8c1304c5f44,
            0xb01fc2a3726c80b5,
            0xe1d293e5bbd919c9,
            0x4b78e80020ef2ca,
        ]),
        c1: Fp::from_raw_unchecked([
            0x952ea4460462618f,
            0x238d5eddf025c62f,
            0xf6c94b012ea92e72,
            0x3ce24eac1c93808,
            0x55950f945da483c,
            0x10a768d0df4eabc,
        ]),
    };
    let b = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xa1e09175a4d2c1fe,
            0x8b33acfc204eff12,
            0xe24415a11b456e42,
            0x61d996b1b6ee1936,
            0x1164dbe8667c853c,
            0x788557acc7d9c79,
        ]),
        c1: Fp::from_raw_unchecked([
            0xda6a87cc6f48fa36,
            0xfc7b488277c1903,
            0x9445ac4adc448187,
            0x2616d5bc9099209,
            0xdbed46772db58d48,
            0x11b94d5076c7b7b1,
        ]),
    };
    //[a2332499367dd291, 41882f1e421e6c04, bc6a01cea4131ffb, d5ccc0ffed5730d8, 28c08d93d3196725, 113a0b1f3ec936b])), c1: Fp384(BigInteger384([c00e498bee3a3b12, 3ac6975d105a3631, 99d635ebdedee2ca, bc815bde58a6ecc8, 26382035f22c7652, 54f5a96fa8aef8]
    let c = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xa2332499367dd291, 
            0x41882f1e421e6c04, 
            0xbc6a01cea4131ffb, 
            0xd5ccc0ffed5730d8, 
            0x28c08d93d3196725, 
            0x113a0b1f3ec936b,
        ]),
        c1: Fp::from_raw_unchecked([
            0xc00e498bee3a3b12, 
            0x3ac6975d105a3631, 
            0x99d635ebdedee2ca, 
            0xbc815bde58a6ecc8, 
            0x26382035f22c7652, 
            0x54f5a96fa8aef8,
        ]),
    };

    assert_eq!(a * b, c);
}

#[test]
fn test_addition() {
    let a = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xc9a2183163ee70d4,
            0xbc3770a7196b5c91,
            0xa247f8c1304c5f44,
            0xb01fc2a3726c80b5,
            0xe1d293e5bbd919c9,
            0x4b78e80020ef2ca,
        ]),
        c1: Fp::from_raw_unchecked([
            0x952ea4460462618f,
            0x238d5eddf025c62f,
            0xf6c94b012ea92e72,
            0x3ce24eac1c93808,
            0x55950f945da483c,
            0x10a768d0df4eabc,
        ]),
    };
    let b = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xa1e09175a4d2c1fe,
            0x8b33acfc204eff12,
            0xe24415a11b456e42,
            0x61d996b1b6ee1936,
            0x1164dbe8667c853c,
            0x788557acc7d9c79,
        ]),
        c1: Fp::from_raw_unchecked([
            0xda6a87cc6f48fa36,
            0xfc7b488277c1903,
            0x9445ac4adc448187,
            0x2616d5bc9099209,
            0xdbed46772db58d48,
            0x11b94d5076c7b7b1,
        ]),
    };
    //[e679e9a708c132d1, 305fc05f09ba5ba3, 6598ac3291888587, f7d67f622865865d, 2cfc6a0db5b455ca, a91a9b4b6c77e59])), c1: Fp384(BigInteger384([ea906c1273ab5bc4, 1c49b621e7a1df32, 6c1b951c50e467f9, ec0cb85389ddb683, 1b0b91b006ee8c48, 111589976cf79183]
    let c = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xe679e9a708c132d1, 
            0x305fc05f09ba5ba3, 
            0x6598ac3291888587, 
            0xf7d67f622865865d, 
            0x2cfc6a0db5b455ca, 
            0xa91a9b4b6c77e59,
        ]),
        c1: Fp::from_raw_unchecked([
            0xea906c1273ab5bc4, 
            0x1c49b621e7a1df32, 
            0x6c1b951c50e467f9, 
            0xec0cb85389ddb683, 
            0x1b0b91b006ee8c48, 
            0x111589976cf79183,
        ]),
    };

    assert_eq!(a + b, c);
}

#[test]
fn test_subtraction() {
    let a = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xc9a2183163ee70d4,
            0xbc3770a7196b5c91,
            0xa247f8c1304c5f44,
            0xb01fc2a3726c80b5,
            0xe1d293e5bbd919c9,
            0x4b78e80020ef2ca,
        ]),
        c1: Fp::from_raw_unchecked([
            0x952ea4460462618f,
            0x238d5eddf025c62f,
            0xf6c94b012ea92e72,
            0x3ce24eac1c93808,
            0x55950f945da483c,
            0x10a768d0df4eabc,
        ]),
    };
    let b = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xa1e09175a4d2c1fe,
            0x8b33acfc204eff12,
            0xe24415a11b456e42,
            0x61d996b1b6ee1936,
            0x1164dbe8667c853c,
            0x788557acc7d9c79,
        ]),
        c1: Fp::from_raw_unchecked([
            0xda6a87cc6f48fa36,
            0xfc7b488277c1903,
            0x9445ac4adc448187,
            0x2616d5bc9099209,
            0xdbed46772db58d48,
            0x11b94d5076c7b7b1,
        ]),
    };
    // [acca46bbbf1baed7, 480f20ef291c5d7f, def7454fcf103902, 686905e4bc737b0d, 96a8bdbdc1fdddc8, fedd734b4d56673c])), c1: Fp384(BigInteger384([3fccdc799519675a, 2ad10799f8a9ad2c, 817700e60c6df4eb, 1b8f9181f9b4b98e, efa7104284c6042f, f0ff6382aef243f4]
    let c = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xacca46bbbf1baed7, 
            0x480f20ef291c5d7f, 
            0xdef7454fcf103902, 
            0x686905e4bc737b0d, 
            0x96a8bdbdc1fdddc8, 
            0xfedd734b4d56673c,
        ]),
        c1: Fp::from_raw_unchecked([
            0x3fccdc799519675a, 
            0x2ad10799f8a9ad2c, 
            0x817700e60c6df4eb, 
            0x1b8f9181f9b4b98e, 
            0xefa7104284c6042f, 
            0xf0ff6382aef243f4,
        ]),
    };

    assert_eq!(a - b, c);
}

#[test]
fn test_negation() {
    let a = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xc9a2183163ee70d4,
            0xbc3770a7196b5c91,
            0xa247f8c1304c5f44,
            0xb01fc2a3726c80b5,
            0xe1d293e5bbd919c9,
            0x4b78e80020ef2ca,
        ]),
        c1: Fp::from_raw_unchecked([
            0x952ea4460462618f,
            0x238d5eddf025c62f,
            0xf6c94b012ea92e72,
            0x3ce24eac1c93808,
            0x55950f945da483c,
            0x10a768d0df4eabc,
        ]),
    };
    let b = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xbb66a7ce9c118f2d, 
            0x5ad3ec9d1694a36e, 
            0x7cab696e89bce8bb, 
            0x6a03174f8e8892d9, 
            0xe46871dab0c82f71, 
            0xfcf6abc615b61e1f,
        ]),
        c1: Fp::from_raw_unchecked([
            0xefda1bb9fb9d9e72, 
            0xf37dfe663fda39d0, 
            0x282a172e8b60198d, 
            0x1654b5083f2bdb86, 
            0xc0e1b4c726c700ff, 
            0xa3c3b909d0262e,
        ]),
    };

    assert_eq!(-a, b);
}

#[test]
fn test_sqrt() {
    // a = 1488924004771393321054797166853618474668089414631333405711627789629391903630694737978065425271543178763948256226639*u + 784063022264861764559335808165825052288770346101304131934508881646553551234697082295473567906267937225174620141295
    let a = Fp2 {
        c0: Fp::from_raw_unchecked([
            0x2beed14627d7f9e9,
            0xb6614e06660e5dce,
            0x6c4cc7c2f91d42c,
            0x996d78474b7a63cc,
            0xebaebc4c820d574e,
            0x18865e12d93fd845,
        ]),
        c1: Fp::from_raw_unchecked([
            0x7d828664baf4f566,
            0xd17e663996ec7339,
            0x679ead55cb4078d0,
            0xfe3b2260e001ec28,
            0x305993d043d91b68,
            0x626f03c0489b72d,
        ]),
    };

    assert_eq!(a.sqrt().unwrap().square(), a);

    // b = 5, which is a generator of the p - 1 order
    // multiplicative subgroup
    let b = Fp2 {
        c0: Fp::from_raw_unchecked([
            0x6631000000105545,
            0x211400400eec000d,
            0x3fa7af30c820e316,
            0xc52a8b8d6387695d,
            0x9fb4e61d1e83eac5,
            0x5cb922afe84dc7,
        ]),
        c1: Fp::zero(),
    };

    assert_eq!(b.sqrt().unwrap().square(), b);

    // c = 25, which is a generator of the (p - 1) / 2 order
    // multiplicative subgroup
    let c = Fp2 {
        c0: Fp::from_raw_unchecked([
            0x44f600000051ffae,
            0x86b8014199480043,
            0xd7159952f1f3794a,
            0x755d6e3dfe1ffc12,
            0xd36cd6db5547e905,
            0x2f8c8ecbf1867bb,
        ]),
        c1: Fp::zero(),
    };

    assert_eq!(c.sqrt().unwrap().square(), c);

    // 2155129644831861015726826462986972654175647013268275306775721078997042729172900466542651176384766902407257452753362*u + 2796889544896299244102912275102369318775038861758288697415827248356648685135290329705805931514906495247464901062529
    // is nonsquare.
    assert!(bool::from(
        Fp2 {
            c0: Fp::from_raw_unchecked([
                0xc5fa1bc8fd00d7f6,
                0x3830ca454606003b,
                0x2b287f1104b102da,
                0xa7fb30f28230f23e,
                0x339cdb9ee953dbf0,
                0xd78ec51d989fc57
            ]),
            c1: Fp::from_raw_unchecked([
                0x27ec4898cf87f613,
                0x9de1394e1abb05a5,
                0x947f85dc170fc14,
                0x586fbc696b6114b7,
                0x2b3475a4077d7169,
                0x13e1c895cc4b6c22
            ])
        }
        .sqrt()
        .is_none()
    ));
}

#[test]
fn test_inversion() {
    let a = Fp2 {
        c0: Fp::from_raw_unchecked([
            0x1128ecad67549455,
            0x9e7a1cff3a4ea1a8,
            0xeb208d51e08bcf27,
            0xe98ad40811f5fc2b,
            0x736c3a59232d511d,
            0x10acd42d29cfcbb6,
        ]),
        c1: Fp::from_raw_unchecked([
            0xd328e37cc2f58d41,
            0x948df0858a605869,
            0x6032f9d56f93a573,
            0x2be483ef3fffdc87,
            0x30ef61f88f483c2a,
            0x1333f55a35725be0,
        ]),
    };

    let b = Fp2 {
        c0: Fp::from_raw_unchecked([
            0x581a1333d4f48a6,
            0x58242f6ef0748500,
            0x292c955349e6da5,
            0xba37721ddd95fcd0,
            0x70d167903aa5dfc5,
            0x11895e118b58a9d5,
        ]),
        c1: Fp::from_raw_unchecked([
            0xeda09d2d7a85d17,
            0x8808e137a7d1a2cf,
            0x43ae2625c1ff21db,
            0xf85ac9fdf7a74c64,
            0x8fccdda5b8da9738,
            0x8e84f0cb32cd17d,
        ]),
    };

    assert_eq!(a.invert().unwrap(), b);

    assert!(Fp2::zero().invert().is_none().unwrap_u8() == 1);
}

#[test]
fn test_lexicographic_largest() {
    assert!(!bool::from(Fp2::zero().lexicographically_largest()));
    assert!(!bool::from(Fp2::one().lexicographically_largest()));
    assert!(bool::from(
        Fp2 {
            c0: Fp::from_raw_unchecked([
                0x1128ecad67549455,
                0x9e7a1cff3a4ea1a8,
                0xeb208d51e08bcf27,
                0xe98ad40811f5fc2b,
                0x736c3a59232d511d,
                0x10acd42d29cfcbb6,
            ]),
            c1: Fp::from_raw_unchecked([
                0xd328e37cc2f58d41,
                0x948df0858a605869,
                0x6032f9d56f93a573,
                0x2be483ef3fffdc87,
                0x30ef61f88f483c2a,
                0x1333f55a35725be0,
            ]),
        }
        .lexicographically_largest()
    ));
    assert!(!bool::from(
        Fp2 {
            c0: -Fp::from_raw_unchecked([
                0x1128ecad67549455,
                0x9e7a1cff3a4ea1a8,
                0xeb208d51e08bcf27,
                0xe98ad40811f5fc2b,
                0x736c3a59232d511d,
                0x10acd42d29cfcbb6,
            ]),
            c1: -Fp::from_raw_unchecked([
                0xd328e37cc2f58d41,
                0x948df0858a605869,
                0x6032f9d56f93a573,
                0x2be483ef3fffdc87,
                0x30ef61f88f483c2a,
                0x1333f55a35725be0,
            ]),
        }
        .lexicographically_largest()
    ));
    assert!(!bool::from(
        Fp2 {
            c0: Fp::from_raw_unchecked([
                0x1128ecad67549455,
                0x9e7a1cff3a4ea1a8,
                0xeb208d51e08bcf27,
                0xe98ad40811f5fc2b,
                0x736c3a59232d511d,
                0x10acd42d29cfcbb6,
            ]),
            c1: Fp::zero(),
        }
        .lexicographically_largest()
    ));
    assert!(bool::from(
        Fp2 {
            c0: -Fp::from_raw_unchecked([
                0x1128ecad67549455,
                0x9e7a1cff3a4ea1a8,
                0xeb208d51e08bcf27,
                0xe98ad40811f5fc2b,
                0x736c3a59232d511d,
                0x10acd42d29cfcbb6,
            ]),
            c1: Fp::zero(),
        }
        .lexicographically_largest()
    ));
}
