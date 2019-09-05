use bls12_381::{Scalar, G1Projective, G2Projective};

pub struct PrivateKey {
    sk: Scalar,
}

impl PrivateKey {
    pub fn to_public(&self) -> PublicKey {
        PublicKey::from_pk(&(G1Projective::generator() * &self.sk))
    }
}

pub struct PublicKey {
    pk: G1Projective,
}

impl PublicKey {
    pub fn from_pk(pk: &G1Projective) -> PublicKey {
        PublicKey { pk: pk.clone() }
    }
}

pub struct Signature {
    sig: G2Projective,
}

