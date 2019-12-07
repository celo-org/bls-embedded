use bls12_377::{Scalar, G1Affine, G2Affine, G1Projective, G2Projective};
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
        PublicKey::from_pk(&(G2Projective::generator() * &self.sk))
    }

    #[inline(always)]
    pub fn sign(&self, message: &[u8], extra_data: &[u8], hash: &G1Projective) -> Result<Signature, ErrorCode> {
        self.sign_message(PRF_KEY, SIG_DOMAIN, message, extra_data, hash)
    }

    #[inline(always)]
    pub fn sign_message(&self, key: &[u8], domain: &[u8], message: &[u8], extra_data: &[u8], hash: &G1Projective) -> Result<Signature, ErrorCode> {
        Ok(Signature::from_sig(&hash.mul(&self.sk)))
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
    pub fn serialize(&self) -> G2Affine/*[u8; 192]*/ {
        G2Affine::from(&self.pk)//.to_uncompressed()
    }
}

pub struct Signature {
    sig: G1Projective,
}

impl Signature {
    pub fn default() -> Self {
       Self { sig: G1Projective::generator() }  
    }

    #[inline(always)]
    pub fn from_sig(sig: &G1Projective) -> Signature {
        Signature { sig: sig.clone() }
    }

    #[inline(always)]
    pub fn serialize(&self) -> [u8; 96] {
        G1Affine::from(self.sig).to_uncompressed()
    }
}
