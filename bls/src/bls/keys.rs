use bls12_377::{Scalar, G1Affine, G2Affine, G1Projective, G2Projective};
use crate::error::ErrorCode;
use core::ops::Mul;

pub struct PrivateKey {
    sk: Scalar,
}

impl PrivateKey {
    pub fn from_scalar(s: &Scalar) -> Self {
        Self { sk: s.clone() }
    }

    pub fn to_public(&self) -> PublicKey {
        PublicKey::from_pk(&(G2Projective::generator() * &self.sk))
    }

    #[inline(always)]
    pub fn sign_hash(&self, hash: &[u8; 96]) -> Result<Signature, ErrorCode> {
       let hash_elem = G1Affine::from_uncompressed_unchecked(hash).unwrap(); 
       Ok(Signature::from_sig(&hash_elem.mul(&self.sk)))
    }
}

pub struct PublicKey {
    pk: G2Projective,
}

impl PublicKey {
    pub fn from_pk(pk: &G2Projective) -> PublicKey {
        PublicKey { pk: pk.clone() }
    }

    #[inline(always)]
    pub fn serialize(&self) -> [u8; 192] {
        G2Affine::from(&self.pk).to_uncompressed()
    }
}

pub struct Signature {
    sig: G1Projective,
}

impl Signature {
    #[inline(always)]
    pub fn from_sig(sig: &G1Projective) -> Signature {
        Signature { sig: sig.clone() }
    }

    #[inline(always)]
    pub fn serialize(&self) -> [u8; 96] {
        G1Affine::from(self.sig).to_uncompressed()
    }
}
