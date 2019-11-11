#[derive(Debug, PartialEq)]
pub enum LegendreSymbol {
    Zero = 0,
    QuadraticResidue = 1,
    QuadraticNonResidue = -1,
}

/*#[inline(always)]
fn supervisor_call(syscall_id: (u32, u32), params: &[u32]) -> u32 {
    let ret_id: u32;
    let ret_val: u32;
    unsafe {
        // Invoke the supervisor
        asm!("svc #1"
            : "={r0}"(ret_id), "={r1}"(ret_val)
            : "{r0}"(syscall_id.0),
              "{r1}"(params.as_ptr())
            : "r0", "r1"
            : "volatile");
        ret_val
    }
}

#[inline(always)]
pub fn os_exit(exit_code: u32) {
    const SYSCALL_ID: (u32, u32) = (0x600062e1, 0x9000626f);
    let params = [
//        exit_code,
        0,
    ];
    supervisor_call(SYSCALL_ID, &params);
}

#[inline(always)]
pub fn os_rng(buf: &mut [u8]) {
    const SYSCALL_ID: (u32, u32) = (0x600089ec, 0x900089d4);
    let params = [
        buf.as_mut_ptr() as u32,
        buf.len() as u32,
    ];
    supervisor_call(SYSCALL_ID, &params);
}

#[inline(always)]
pub fn os_multm(result: &mut [u8], left: &mut [u8], right: &mut [u8], modulus: &mut [u8], len: u32) -> () {
    const SYSCALL_ID: (u32, u32) = (0x60004445, 0x900044f3);
    let params = [
        result.as_mut_ptr() as u32,
        left.as_mut_ptr() as u32,
        right.as_mut_ptr() as u32,
        modulus.as_mut_ptr() as u32,
        len,
    ];
    supervisor_call(SYSCALL_ID, &params);
    ()
}*/

/// Compute a + b + carry, returning the result and the new carry over.
#[inline(always)]
pub const fn adc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + (b as u128) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}

/// Compute a - (b + borrow), returning the result and the new borrow.
#[inline(always)]
pub const fn sbb(a: u64, b: u64, borrow: u64) -> (u64, u64) {
    let ret = (a as u128).wrapping_sub((b as u128) + ((borrow >> 63) as u128));
    (ret as u64, (ret >> 64) as u64)
}

/// Compute a + (b * c) + carry, returning the result and the new carry over.
//#[inline(always)]
#[no_mangle]
pub const fn mac(a: u64, b: u64, c: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + ((b as u128) * (c as u128)) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}

macro_rules! impl_add_binop_specify_output {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> Add<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: &'b $rhs) -> $output {
                &self + rhs
            }
        }

        impl<'a> Add<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs) -> $output {
                self + &rhs
            }
        }

        impl Add<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs) -> $output {
                &self + &rhs
            }
        }
    };
}

macro_rules! impl_sub_binop_specify_output {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> Sub<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: &'b $rhs) -> $output {
                &self - rhs
            }
        }

        impl<'a> Sub<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs) -> $output {
                self - &rhs
            }
        }

        impl Sub<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs) -> $output {
                &self - &rhs
            }
        }
    };
}

macro_rules! impl_binops_additive_specify_output {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl_add_binop_specify_output!($lhs, $rhs, $output);
        impl_sub_binop_specify_output!($lhs, $rhs, $output);
    };
}

macro_rules! impl_binops_multiplicative_mixed {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> Mul<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: &'b $rhs) -> $output {
                &self * rhs
            }
        }

        impl<'a> Mul<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs) -> $output {
                self * &rhs
            }
        }

        impl Mul<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs) -> $output {
                &self * &rhs
            }
        }
    };
}

macro_rules! impl_binops_additive {
    ($lhs:ident, $rhs:ident) => {
        impl_binops_additive_specify_output!($lhs, $rhs, $lhs);

        impl SubAssign<$rhs> for $lhs {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs) {
                *self = &*self - &rhs;
            }
        }

        impl AddAssign<$rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs) {
                *self = &*self + &rhs;
            }
        }

        impl<'b> SubAssign<&'b $rhs> for $lhs {
            #[inline]
            fn sub_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self - rhs;
            }
        }

        impl<'b> AddAssign<&'b $rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self + rhs;
            }
        }
    };
}

macro_rules! impl_binops_multiplicative {
    ($lhs:ident, $rhs:ident) => {
        impl_binops_multiplicative_mixed!($lhs, $rhs, $lhs);

        impl MulAssign<$rhs> for $lhs {
            #[inline]
            fn mul_assign(&mut self, rhs: $rhs) {
                *self = &*self * &rhs;
            }
        }

        impl<'b> MulAssign<&'b $rhs> for $lhs {
            #[inline]
            fn mul_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self * rhs;
            }
        }
    };
}
