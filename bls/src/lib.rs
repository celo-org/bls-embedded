#![cfg_attr(not(gen_header), no_std)]
#![no_std]
extern crate libc;

pub mod bls;
pub mod error;

use bls12_377::Scalar;
use crate::bls::keys::PrivateKey;
use subtle::CtOption;
use core::ptr::copy;

use core::slice;

#[cfg(not(gen_header))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn convert_result_to_bool<T, E, F: Fn() -> Result<T, E>>(f: F) -> bool {
    match f() {
        Err(e) => {
            false
        }
        _ => true,
    }
}

pub const fn mac(a: u64, b: u64, c: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + ((b as u128) * (c as u128)) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}

#[no_mangle]
pub extern "C" fn is_valid_key(in_private_key: *const u8) -> bool {
   let pk_array = in_private_key as *const [u8; 32];
   let priv_key = unsafe { Scalar::from_bytes(&*pk_array) } ;
   bool::from(CtOption::is_some(&priv_key))
}

#[no_mangle]
pub extern "C" fn sign_hash(
    in_private_key: *mut u64,
    in_hash: *mut u8,
    out_signature: *mut u8,
) -> bool {
    let pk_array = in_private_key as *mut [u64; 4];
    let private_key = unsafe { PrivateKey::from_scalar(&Scalar::from_raw(*pk_array)) };
    let hash = unsafe { slice::from_raw_parts(in_hash, 96) };
    let mut hash_arr: [u8; 96] = [0; 96];
    hash_arr.copy_from_slice(&hash[0..96]);
    let sig = private_key.sign_hash(&hash_arr).unwrap();
    let sig_arr = sig.serialize();
    unsafe { copy(sig_arr.as_ptr(), out_signature, 96); };
    true
}

#[no_mangle]
pub extern "C" fn get_pubkey(
    in_private_key: *mut u64,
    out_public_key: *mut u8,
) -> bool {
    let private_key = unsafe { PrivateKey::from_scalar(&Scalar::from_raw(*(in_private_key as *mut [u64; 4]))) }; 
    let pub_arr = private_key.to_public().serialize();
    unsafe { copy(pub_arr.as_ptr(), out_public_key, 192) };
    true
}
