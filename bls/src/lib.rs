#![cfg_attr(not(gen_header), no_std)]
extern crate libc;
//extern crate rand;
//extern crate blake2s_simd;

pub mod bls;
//pub mod hash;
pub mod error;

use bls12_377::{G1Projective, G2Projective, Scalar};
use crate::bls::keys::{PublicKey, PrivateKey, Signature};
use crate::error::ErrorCode;
use bls12_377::util::{os_exit, os_rng, os_multm};
use subtle::CtOption;

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

#[no_mangle]
pub extern "C" fn generate_private_key(out_private_key: *mut *mut PrivateKey) -> bool {
    let mut key = PrivateKey::default();
    unsafe {
        *out_private_key = &mut key;
    };
    true
}

#[no_mangle]
pub extern "C" fn is_valid_key(in_private_key: *const u8) -> bool {
   /*   let pk_array = in_private_key as *const [u64; 4];
   let private_key = unsafe { PrivateKey::from_scalar(&Scalar::from_raw(*pk_array))  };
   private_key < Scalar::MODULUS */
   let pk_array = in_private_key as *const [u8; 32];
   let priv_key = unsafe { Scalar::from_bytes(&*pk_array) } ;
   bool::from(CtOption::is_some(&priv_key))
//   true
}

#[no_mangle]
pub extern "C" fn generate_signature(out_signature: *mut *mut Signature) -> bool {
    let mut sig = Signature::default();
    unsafe {
        *out_signature = &mut sig;
    };
    true
}

#[no_mangle]
pub extern "C" fn generate_hash(out_hash: *mut *mut G2Projective) -> bool {
    let mut hash = G2Projective::generator();
    unsafe {
        *out_hash = &mut hash;
    }
    true
}

#[no_mangle]
pub extern "C" fn test_syscall() {
    let mut buf: [u8; 4] = [0; 4];
    let mut left: [u8; 4] = [0, 0, 0, 0];
    let mut right: [u8; 4] = [0, 0, 0, 0];
    let mut modulus: [u8; 4] = [1, 1, 1, 1];
    let len: u32 = 4;
//    os_rng(&mut buf);

    os_multm(&mut buf, &mut left, &mut right, &mut modulus, len);
}

#[no_mangle]
pub extern "C" fn deserialize_private_key(
    _in_private_key_bytes: *const u8,
    _in_private_key_bytes_len: i32,
    _out_private_key: *mut *mut PrivateKey,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn serialize_private_key(
    _in_private_key: *const PrivateKey,
    _out_bytes: *mut *mut u8,
    _out_len: *mut i32,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn private_key_to_public_key(
    in_private_key: *const PrivateKey,
    out_public_key: *mut *mut PublicKey,
) -> bool {
   let priv_key = unsafe { &*in_private_key };
   let mut pub_key = priv_key.to_public();
   unsafe { *out_public_key = &mut pub_key; }
   true
}

#[no_mangle]
pub extern "C" fn sign_message(
    in_private_key: *mut u64,
    in_message: *const u8,
    in_message_len: i32,
    in_extra_data: *const u8,
    in_extra_data_len: i32,
    should_use_composite: bool,
) -> bool {
//    convert_result_to_bool::<_, ErrorCode, _>(|| {
        let pk_array = in_private_key as *mut [u64; 4];
        let pk = unsafe { &Scalar::from_raw(*pk_array)};
        let private_key = unsafe { PrivateKey::from_scalar(&Scalar::from_raw(*pk_array))  };
        let message = unsafe { slice::from_raw_parts(in_message, in_message_len as usize) };
        let extra_data = unsafe { slice::from_raw_parts(in_extra_data, in_extra_data_len as usize) };
        let hash = G2Projective::generator();//unsafe { &*hash };
        private_key.sign(message, extra_data, &hash);

        true
}

#[no_mangle]
pub extern "C" fn sign_pop(
    _in_private_key: *const PrivateKey,
    _out_signature: *mut *mut Signature,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn destroy_private_key(_private_key: *mut PrivateKey) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn free_vec(_bytes: *mut u8, _len: i32) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn destroy_public_key(_public_key: *mut PublicKey) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn destroy_signature(_signature: *mut Signature) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn deserialize_public_key(
    _in_public_key_bytes: *const u8,
    _in_public_key_bytes_len: i32,
    _out_public_key: *mut *mut PublicKey,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn serialize_public_key(
    _in_public_key: *const PublicKey,
    _out_bytes: *mut *mut u8,
    _out_len: *mut i32,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn deserialize_signature(
    _in_signature_bytes: *const u8,
    _in_signature_bytes_len: i32,
    _out_signature: *mut *mut Signature,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn serialize_signature(
    _in_signature: *const Signature,
    _out_bytes: *mut *mut u8,
    _out_len: *mut i32,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn verify_signature(
    _in_public_key: *const PublicKey,
    _in_message: *const u8,
    _in_message_len: i32,
    _in_extra_data: *const u8,
    _in_extra_data_len: i32,
    _in_signature: *const Signature,
    _should_use_composite: bool,
    _out_verified: *mut bool,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn verify_pop(
    _in_public_key: *const PublicKey,
    _in_signature: *const Signature,
    _out_verified: *mut bool,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn aggregate_public_keys(
    _in_public_keys: *const *const PublicKey,
    _in_public_keys_len: i32,
    _out_public_key: *mut *mut PublicKey,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn aggregate_public_keys_subtract(
    _in_aggregated_public_key: *const PublicKey,
    _in_public_keys: *const *const PublicKey,
    _in_public_keys_len: i32,
    _out_public_key: *mut *mut PublicKey,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn aggregate_signatures(
    _in_signatures: *const *const Signature,
    _in_signatures_len: i32,
    _out_signature: *mut *mut Signature,
) -> bool {
    unimplemented!();
}
