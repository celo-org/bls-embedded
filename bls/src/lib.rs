extern crate libc;

pub mod bls;

use libc::c_int;

use crate::bls::keys::{PublicKey, PrivateKey, Signature};

#[no_mangle]
pub extern "C" fn generate_private_key(_out_private_key: *mut *mut PrivateKey) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn deserialize_private_key(
    _in_private_key_bytes: *const u8,
    _in_private_key_bytes_len: c_int,
    _out_private_key: *mut *mut PrivateKey,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn serialize_private_key(
    _in_private_key: *const PrivateKey,
    _out_bytes: *mut *mut u8,
    _out_len: *mut c_int,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn private_key_to_public_key(
    _in_private_key: *const PrivateKey,
    _out_public_key: *mut *mut PublicKey,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn sign_message(
    _in_private_key: *const PrivateKey,
    _in_message: *const u8,
    _in_message_len: c_int,
    _in_extra_data: *const u8,
    _in_extra_data_len: c_int,
    _should_use_composite: bool,
    _out_signature: *mut *mut Signature,
) -> bool {
    unimplemented!();
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
pub extern "C" fn free_vec(_bytes: *mut u8, _len: c_int) {
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
    _in_public_key_bytes_len: c_int,
    _out_public_key: *mut *mut PublicKey,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn serialize_public_key(
    _in_public_key: *const PublicKey,
    _out_bytes: *mut *mut u8,
    _out_len: *mut c_int,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn deserialize_signature(
    _in_signature_bytes: *const u8,
    _in_signature_bytes_len: c_int,
    _out_signature: *mut *mut Signature,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn serialize_signature(
    _in_signature: *const Signature,
    _out_bytes: *mut *mut u8,
    _out_len: *mut c_int,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn verify_signature(
    _in_public_key: *const PublicKey,
    _in_message: *const u8,
    _in_message_len: c_int,
    _in_extra_data: *const u8,
    _in_extra_data_len: c_int,
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
    _in_public_keys_len: c_int,
    _out_public_key: *mut *mut PublicKey,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn aggregate_public_keys_subtract(
    _in_aggregated_public_key: *const PublicKey,
    _in_public_keys: *const *const PublicKey,
    _in_public_keys_len: c_int,
    _out_public_key: *mut *mut PublicKey,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn aggregate_signatures(
    _in_signatures: *const *const Signature,
    _in_signatures_len: c_int,
    _out_signature: *mut *mut Signature,
) -> bool {
    unimplemented!();
}