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

#[derive(Copy, Clone, Debug)]
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
impl Eq for PublicKey {}
impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.pk == other.pk
    }
}

#[derive(Copy, Clone, Debug)]
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
impl Eq for Signature {} impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.sig == other.sig
    }
}

#[test]
fn test_signature_serialization() {
    let elem = [0, 23, 5, 45, 78, 62, 182, 66, 211, 46, 244, 152, 154, 242, 83, 204, 42, 48, 173, 55, 108, 232, 240, 178, 60, 146, 185, 135, 233, 92, 199, 24, 208, 32, 114, 187, 120, 211, 124, 9, 253, 118, 247, 1, 78, 236, 247, 151, 1, 108, 32, 107, 231, 56, 191, 70, 68, 250, 255, 16, 187, 130, 177, 159, 111, 7, 119, 153, 3, 166, 173, 37, 36, 128, 156, 226, 159, 148, 104, 59, 227, 43, 189, 208, 114, 236, 11, 230, 106, 224, 237, 13, 135, 129, 242, 119];
    let elem_result = Signature { sig: (G1Projective::generator() * &Scalar::from(5)) }.serialize();
    assert_eq!(&elem[..], &elem_result[..]);
}

#[test]
fn test_publickey_serialization() {
    let elem = [0, 31, 183, 170, 199, 212, 167, 3, 66, 81, 201, 4, 241, 48, 79, 223, 24, 52, 101, 225, 116, 36, 166, 246, 213, 127, 77, 200, 154, 183, 73, 53, 249, 207, 6, 102, 170, 157, 11, 128, 177, 20, 254, 185, 15, 142, 231, 68, 0, 222, 228, 89, 156, 13, 254, 199, 91, 133, 241, 129, 173, 74, 215, 198, 210, 32, 83, 154, 161, 153, 255, 92, 239, 64, 69, 147, 39, 48, 118, 242, 26, 126, 220, 109, 229, 226, 101, 150, 25, 228, 38, 133, 96, 89, 73, 238, 0, 105, 186, 188, 162, 17, 191, 123, 4, 159, 165, 161, 68, 105, 85, 121, 63, 19, 169, 22, 165, 195, 165, 66, 206, 1, 108, 166, 186, 198, 49, 232, 110, 212, 243, 6, 4, 6, 2, 95, 165, 241, 12, 160, 98, 34, 217, 143, 1, 42, 244, 0, 161, 173, 241, 170, 146, 11, 183, 159, 9, 30, 138, 40, 3, 30, 231, 111, 97, 118, 217, 229, 221, 205, 106, 218, 224, 24, 116, 233, 237, 223, 225, 180, 55, 239, 219, 248, 119, 10, 49, 96, 145, 22, 219, 26];
    let elem_result = PublicKey { pk: (G2Projective::generator() * &Scalar::from(5)) }.serialize();
    assert_eq!(&elem[..], &elem_result[..]);
}

#[test]
fn test_pubkey_derivation() {
    let priv_key = PrivateKey { sk: Scalar::from_bytes(&[4, 197, 118, 144, 113, 147, 204, 28, 197, 162, 85, 243, 160, 128, 173, 172, 138, 199, 168, 49, 17, 57, 129, 121, 37, 178, 165, 25, 163, 125, 236, 235]).unwrap() };
    let pub_key = PublicKey { pk: G2Projective::from(G2Affine::from_uncompressed(&[0, 31, 183, 170, 199, 212, 167, 3, 66, 81, 201, 4, 241, 48, 79, 223, 24, 52, 101, 225, 116, 36, 166, 246, 213, 127, 77, 200, 154, 183, 73, 53, 249, 207, 6, 102, 170, 157, 11, 128, 177, 20, 254, 185, 15, 142, 231, 68, 0, 222, 228, 89, 156, 13, 254, 199, 91, 133, 241, 129, 173, 74, 215, 198, 210, 32, 83, 154, 161, 153, 255, 92, 239, 64, 69, 147, 39, 48, 118, 242, 26, 126, 220, 109, 229, 226, 101, 150, 25, 228, 38, 133, 96, 89, 73, 238, 0, 105, 186, 188, 162, 17, 191, 123, 4, 159, 165, 161, 68, 105, 85, 121, 63, 19, 169, 22, 165, 195, 165, 66, 206, 1, 108, 166, 186, 198, 49, 232, 110, 212, 243, 6, 4, 6, 2, 95, 165, 241, 12, 160, 98, 34, 217, 143, 1, 42, 244, 0, 161, 173, 241, 170, 146, 11, 183, 159, 9, 30, 138, 40, 3, 30, 231, 111, 97, 118, 217, 229, 221, 205, 106, 218, 224, 24, 116, 233, 237, 223, 225, 180, 55, 239, 219, 248, 119, 10, 49, 96, 145, 22, 219, 26]).unwrap()) };
    let pub_key_result = priv_key.to_public();
    assert_eq!(pub_key, pub_key_result);
}

#[test]
fn test_sign_hash() {
    let pk = PrivateKey { sk: Scalar::from_bytes(&[0, 29, 95, 218, 224, 78, 133, 135, 130, 200, 43, 28, 142, 10, 80, 84, 125, 253, 33, 2, 180, 167, 65, 126, 218, 142, 236, 214, 208, 210, 173, 110]).unwrap() };
    let hash = [1, 58, 188, 92, 140, 12, 189, 117, 105, 197, 33, 105, 185, 16, 87, 31, 159, 115, 44, 102, 71, 51, 201, 36, 231, 93, 175, 149, 158, 151, 231, 106, 242, 151, 62, 100, 112, 228, 56, 96, 91, 123, 19, 167, 79, 52, 11, 89, 0, 218, 81, 55, 151, 57, 97, 148, 53, 165, 167, 94, 191, 153, 94, 17, 208, 185, 18, 67, 232, 26, 148, 72, 247, 83, 96, 234, 253, 154, 57, 1, 207, 252, 125, 10, 161, 166, 174, 76, 146, 169, 49, 43, 174, 128, 88, 134];
    let sig = Signature { sig: G1Projective::from(G1Affine::from_uncompressed(&[0, 23, 5, 45, 78, 62, 182, 66, 211, 46, 244, 152, 154, 242, 83, 204, 42, 48, 173, 55, 108, 232, 240, 178, 60, 146, 185, 135, 233, 92, 199, 24, 208, 32, 114, 187, 120, 211, 124, 9, 253, 118, 247, 1, 78, 236, 247, 151, 1, 108, 32, 107, 231, 56, 191, 70, 68, 250, 255, 16, 187, 130, 177, 159, 111, 7, 119, 153, 3, 166, 173, 37, 36, 128, 156, 226, 159, 148, 104, 59, 227, 43, 189, 208, 114, 236, 11, 230, 106, 224, 237, 13, 135, 129, 242, 119]).unwrap()) };

    let sig_result = pk.sign_hash(&hash).unwrap();
    assert_eq!(sig, sig_result);
}
