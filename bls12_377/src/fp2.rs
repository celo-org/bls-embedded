//! This module implements arithmetic over the quadratic extension field Fp2.

use core::fmt;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign};

use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::fp::{Fp};
use crate::util::LegendreSymbol;


/// beta = -5
#[inline(always)]
const fn nonresidue() -> Fp {
    Fp::from_raw_unchecked([
        0xfc0b8000000002fa,
        0x97d39cf6e000018b,
        0x2072420fbfa05044,
        0xcbbcbd50d97c3802,
        0xbaf1ec35813f9eb,
        0x9974a2c0945ad2,
    ])
}

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
    #[inline]
    fn ct_eq(&self, other: &Self) -> Choice {
        self.c0.ct_eq(&other.c0) & self.c1.ct_eq(&other.c1)
    }
}

impl Eq for Fp2 {}
impl PartialEq for Fp2 {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).unwrap_u8() == 1
    }
}

impl ConditionallySelectable for Fp2 {
    #[inline]
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Fp2 {
            c0: Fp::conditional_select(&a.c0, &b.c0, choice),
            c1: Fp::conditional_select(&a.c1, &b.c1, choice),
        }
    }
}

impl<'a> Neg for &'a Fp2 {
    type Output = Fp2;

    #[inline(always)]
    fn neg(self) -> Fp2 {
        self.neg()
    }
}

impl Neg for Fp2 {
    type Output = Fp2;

    #[inline(always)]
    fn neg(self) -> Fp2 {
        -&self
    }
}

impl<'a, 'b> Sub<&'b Fp2> for &'a Fp2 {
    type Output = Fp2;

    #[inline(always)]
    fn sub(self, rhs: &'b Fp2) -> Fp2 {
        self.sub(rhs)
    }
}

impl<'a, 'b> Add<&'b Fp2> for &'a Fp2 {
    type Output = Fp2;

   #[inline(always)]
    fn add(self, rhs: &'b Fp2) -> Fp2 {
        self.add(rhs)
    }
}

impl<'a, 'b> Mul<&'b Fp2> for &'a Fp2 {
    type Output = Fp2;

    #[inline(always)]
    fn mul(self, rhs: &'b Fp2) -> Fp2 {
        self.mul(rhs)
    }
}

impl_binops_additive!(Fp2, Fp2);
impl_binops_multiplicative!(Fp2, Fp2);

impl Fp2 {
    #[inline(always)]
    pub const fn zero() -> Fp2 {
        Fp2 {
            c0: Fp::zero(),
            c1: Fp::zero(),
        }
    }

    #[inline(always)]
    pub const fn one() -> Fp2 {
        Fp2 {
            c0: Fp::one(),
            c1: Fp::zero(),
        }
    }

    #[inline(always)]
    pub fn is_zero(&self) -> Choice {
        self.c0.is_zero() & self.c1.is_zero()
    }

    /// Raises this element to p.
    pub fn frobenius_map(&self) -> Self {
        // This is always just a conjugation. If you're curious why, here's
        // an article about it: https://alicebob.cryptoland.net/the-frobenius-endomorphism-with-finite-fields/
        self.conjugate()
    }

    pub fn conjugate(&self) -> Self {
        Fp2 {
            c0: self.c0,
            c1: -self.c1,
        }
    }

    pub fn mul_by_nonresidue(&self) -> Fp2 {
        // Multiply a + bu by u + 1, getting
        // au + a + bu^2 + bu
        // and because u^2 = -1, we get
        // (a - b) + (a + b)u

        Fp2 {
            c0: self.c0 + (nonresidue() * self.c1),
            c1: self.c0 + self.c1,
        }
    }

    /// Returns whether or not this element is strictly lexicographically
    /// larger than its negation.
    pub fn lexicographically_largest(&self) -> Choice {
        // If this element's c1 coefficient is lexicographically largest
        // then it is lexicographically largest. Otherwise, in the event
        // the c1 coefficient is zero and the c0 coefficient is
        // lexicographically largest, then this element is lexicographically
        // largest.

        self.c1.lexicographically_largest()
            | (self.c1.is_zero() & self.c0.lexicographically_largest())
    }

    #[inline]
    pub fn square(&self) -> Fp2 {
        // Complex squaring:
        //
        // v0  = c0 * c1
        // c0' = (c0 + c1) * (c0 + \beta*c1) - v0 - \beta * v0
        // c1' = 2 * v0
        //
        let mut v0 = (&self.c0).sub(&self.c1);
        let v3 = (&self.c0).sub(&(&self.c1).mul(&nonresidue()));
        let v2 = (&self.c0).mul(&self.c1);
        v0 = (&v0).mul(&v3);
        v0 = (&v0).add(&v2);

        Fp2 {
            c0: (&v0).add(&((&v2).mul(&nonresidue()))),
            c1: (&v2).add(&v2),
        }
    }

    #[inline(always)]
    pub fn mul(&self, rhs: &Fp2) -> Fp2 {
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
        let c0 = (&v0).add(&(&nonresidue()).mul(&v1));
        let mut c1 = (&(&self.c0).add(&self.c1)).mul(&(&rhs.c0).add(&rhs.c1));
        c1 = (&c1).sub(&v0);
        c1 = (&c1).sub(&v1);

        Fp2 { c0, c1 }
    }

    #[inline(always)]
    pub fn add(&self, rhs: &Fp2) -> Fp2 {
        Fp2 {
            c0: (&self.c0).add(&rhs.c0),
            c1: (&self.c1).add(&rhs.c1),
        }
    }

    #[inline(always)]
    pub fn sub(&self, rhs: &Fp2) -> Fp2 {
        Fp2 {
            c0: (&self.c0).sub(&rhs.c0),
            c1: (&self.c1).sub(&rhs.c1),
        }
    }

    #[inline(always)]
    pub fn neg(&self) -> Fp2 {
        Fp2 {
            c0: (&self.c0).neg(),
            c1: (&self.c1).neg(),
        }
    }

    fn norm(&self) -> Fp {
        let t0 = self.c0.square();
        let mut t1 = self.c1.square();
        t1 = -(&t1).mul(&nonresidue());
        t1.add_assign(&t0);
        t1
    }

    fn legendre(&self) -> LegendreSymbol {
        self.norm().legendre()
    }


    /// Algorithm 8, https://eprint.iacr.org/2012/685.pdf
    /// TODO: Investigate switching to algo 10
    // TODO: Add sqrt test coverage
    pub fn sqrt_vartime(&self) -> Option<Self> {
        if self.c1 == Fp::zero() {
            return self.c0.sqrt_vartime().map(|c0| Self { c0, c1: Fp::zero() } )
        }

        match self.legendre() {
            LegendreSymbol::Zero => Some(*self),
            LegendreSymbol::QuadraticNonResidue => None,
            LegendreSymbol::QuadraticResidue => {
               let two_inv = Fp::one()
                   .add(Fp::one())
                   .invert()
                   .unwrap();
               let alpha = self
                   .norm()
                   .sqrt_vartime()
                   .unwrap();
               let mut delta = (alpha + self.c0) * two_inv;
               if delta.legendre() == LegendreSymbol::QuadraticNonResidue {
                   delta -= alpha;
               }
               let c0 = delta.sqrt_vartime().unwrap();
               let c0_inv = c0.invert().unwrap();
               Some(Self { c0: c0, c1: self.c1 * two_inv *c0_inv })
            },
        }
    } 

    /// Computes the multiplicative inverse of this field
    /// element, returning None in the case that this element
    /// is zero.
    #[inline(always)]
    pub fn invert(&self) -> CtOption<Self> {
        // We wish to find the multiplicative inverse of a nonzero
        // element a + bu in Fp2. Algorithm 5.19
        // from Guide to Pairing Based Cryptography
        
        let mut v0 = self.c0.square();
        v0 = v0 - nonresidue() * self.c1.square();
        v0 = v0.invert().unwrap_or(Fp::zero());
        CtOption::new(Fp2 {
            c0: self.c0 * v0,
            c1: -(self.c1 * v0),
        }, Choice::not(self.is_zero()))
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
fn test_norm() {
    let a = Fp2 {
        c0: Fp::from_raw_unchecked([
            0x2beed14627d7f9e9,
            0xb6617e06660e5dce,
            0x6c4cc7c2f91d42c,
            0x996dc8474b7a63cc,
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
    let b = Fp::from_raw_unchecked([
        0xf8397a163b69bed0, 
        0xf175823c7236735c, 
        0x5569469835f84b92, 
        0x714deebc8c061c3c, 
        0x7adcc0994eb519c8, 
        0x230d716ceafd4b,
    ]);
    assert_eq!(a.norm(), b);
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
    let c = Fp2 {
        c0: Fp::from_raw_unchecked([
            0xd83e794440e4512a, 
            0xcefc3c5506e3a280, 
            0x3ffc1cdfeaf90efd, 
            0xb1b9d40e44819881, 
            0x2f924802aaa36b72, 
            0x2d0c6faca6ea9ae,
        ]),
        c1: Fp::from_raw_unchecked([
            0x453be3866ae698a7, 
            0xec3a55aa375652d4, 
            0x9d7c6149ad9b5314, 
            0xfe93487107405a00, 
            0xd693f57de7db450b, 
            0x10aed6c368d2ccf5,
        ]),
    };

    assert_eq!(b - a, c);
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
            0xa972fe45912ab0b0, 
            0x2fad422c707d2a7a, 
            0x1e0c99ca54b14292, 
            0x12b35bad27bfbb4b, 
            0xaac12849e9ca08be, 
            0x9ca440f7d792c1,
        ]),
        c1: Fp::from_raw_unchecked([
            0x93f803dee0c6aee, 
            0x85be5ff1bf7a8b20, 
            0x9343d05ec64f00b6, 
            0x91a1db9f810ce2ac, 
            0xc7a4b33169335bd, 
            0xa9202f9769f137,
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
