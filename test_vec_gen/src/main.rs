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
    let id = G1Projective::generator();
    let id2 = id * Scalar::from(47);
    let id_des = Array { data: G1Affine::to_uncompressed(&G1Affine::from(id2)) };
    println!("{:?}", id_des);
}
