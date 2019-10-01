use bls12_377::{Scalar, G1Projective, G2Projective};
use crate::error::ErrorCode;
use core::ops::Mul;

static PRF_KEY: &'static [u8] = b"096b36a5804bfacef1691e173c366a47ff5ba84a44f26ddd7e8d9f79d5b42df0";
static SIG_DOMAIN: &'static [u8] = b"ULforprf";
static POP_DOMAIN: &'static [u8] = b"ULforpop";

pub struct PrivateKey {
    sk: Scalar,
}

impl PrivateKey {
    pub fn default() -> Self {
        Self { sk: Scalar::from(5) }
    }

    pub fn from_scalar(s: &Scalar) -> Self {
        Self { sk: s.clone() }
    }

    pub fn to_public(&self) -> PublicKey {
        PublicKey::from_pk(&(G1Projective::generator() * &self.sk))
    }

    pub fn sign(&self, message: &[u8], extra_data: &[u8], hash: &G2Projective) -> Result<Signature, ErrorCode> {
        self.sign_message(PRF_KEY, SIG_DOMAIN, message, extra_data, hash)
    }

    pub fn sign_message(&self, key: &[u8], domain: &[u8], message: &[u8], extra_data: &[u8], hash: &G2Projective) -> Result<Signature, ErrorCode> {
        Ok(Signature::from_sig(&hash.mul(&self.sk)))
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

impl Signature {
    pub fn default() -> Self {
       Self { sig: G2Projective::generator() }  
    }

    pub fn from_sig(sig: &G2Projective) -> Signature {
        Signature { sig: sig.clone() }
    }
}
