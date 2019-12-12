use bls12_377::{G1Affine, G1Projective, Scalar};
use std::fmt;

struct Array<T> {
    data: [T; 96]
}

impl<T: fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

fn main() {
    let pk_arr: [u64;4] = [0xb0d221e9b28ee17c, 0x24c102256640b07a, 0xefc2f5fe37f3f98c, 0x0165f07185c6f996];
    let private_key = unsafe { &Scalar::from_raw(pk_arr) };
    let pk_bytes = private_key.to_bytes();
    println!("{:?}", pk_bytes);
    let id = G1Projective::generator();
    let id2 = id * Scalar::from(47);
    let id_des = Array { data: G1Affine::to_uncompressed(&G1Affine::from(id2)) };
    println!("{:?}", id_des);
}
