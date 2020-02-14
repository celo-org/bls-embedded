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
       let hash_elem = G1Affine::from_uncompressed_unchecked_vartime(hash).unwrap(); 
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
        G2Affine::from(&self.pk).to_uncompressed_littleendian()
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
        G1Affine::from(self.sig).to_uncompressed_littleendian()
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
    let priv_key = PrivateKey { sk: Scalar::from_bytes(&[52, 163, 121, 115, 149, 19, 242, 110, 13, 231, 110, 40, 146, 248, 62, 119, 87, 214, 200, 159, 51, 41, 164, 239, 155, 241, 173, 219, 230, 185, 133, 3]).unwrap() };
    let pub_key = PublicKey { pk: G2Projective::from(G2Affine::from_uncompressed(&[1, 65, 146, 224, 231, 36, 217, 8, 154, 9, 197, 85, 87, 10, 60, 10, 116, 199, 107, 77, 65, 110, 195, 241, 61, 149, 135, 254, 254, 231, 193, 180, 204, 158, 62, 152, 255, 162, 62, 57, 242, 63, 232, 173, 205, 118, 153, 74, 0, 33, 97, 106, 240, 49, 100, 155, 187, 111, 209, 35, 149, 158, 19, 5, 53, 161, 255, 29, 150, 27, 180, 76, 35, 128, 168, 52, 28, 185, 165, 29, 3, 171, 74, 204, 98, 167, 76, 26, 163, 61, 205, 9, 165, 185, 175, 92, 0, 255, 19, 80, 75, 234, 65, 82, 108, 145, 163, 112, 232, 187, 181, 136, 5, 148, 204, 65, 187, 54, 121, 249, 199, 164, 107, 239, 193, 46, 94, 130, 16, 4, 237, 46, 67, 32, 180, 185, 63, 12, 189, 114, 59, 70, 32, 214, 1, 103, 254, 116, 159, 104, 88, 88, 209, 241, 131, 173, 192, 119, 152, 28, 214, 52, 212, 168, 14, 233, 120, 89, 97, 233, 93, 236, 94, 172, 27, 173, 64, 49, 117, 213, 228, 168, 212, 232, 114, 121, 204, 16, 246, 121, 184, 81]).unwrap()) };
    let pub_key_result = priv_key.to_public();
    assert_eq!(G2Affine::from(pub_key.pk), G2Affine::from(pub_key_result.pk));
}

#[test]
fn test_sign_hash() {
    let pk = PrivateKey { sk: Scalar::from_bytes(&[10, 145, 220, 128, 41, 236, 187, 134, 47, 34, 61, 132, 196, 20, 201, 239, 33, 80, 184, 182, 49, 79, 15, 212, 4, 73, 201, 248, 74, 226, 158, 12]).unwrap() };
    let hash = [1, 95, 34, 213, 221, 202, 70, 0, 221, 118, 193, 93, 225, 200, 19, 73, 208, 8, 176, 53, 150, 73, 22, 154, 1, 71, 181, 38, 9, 102, 191, 35, 227, 112, 10, 208, 171, 43, 191, 43, 110, 164, 130, 8, 57, 101, 243, 19, 1, 47, 253, 198, 50, 95, 79, 61, 237, 164, 140, 88, 176, 124, 187, 181, 163, 22, 62, 109, 184, 189, 146, 112, 115, 9, 160, 33, 102, 163, 4, 181, 208, 41, 88, 149, 177, 103, 137, 99, 174, 49, 132, 6, 227, 20, 225, 203];
    let sig = Signature { sig: G1Projective::from(G1Affine::from_uncompressed(&[0, 197, 168, 175, 148, 226, 242, 59, 146, 38, 132, 5, 184, 97, 42, 143, 165, 173, 21, 4, 175, 57, 168, 90, 6, 88, 106, 216, 57, 126, 148, 208, 236, 146, 120, 249, 251, 21, 170, 84, 108, 46, 219, 72, 123, 118, 141, 23, 0, 137, 235, 28, 241, 199, 129, 202, 64, 124, 156, 28, 68, 75, 151, 18, 63, 110, 16, 210, 132, 222, 210, 134, 75, 135, 25, 6, 230, 9, 243, 11, 153, 183, 8, 154, 242, 128, 46, 134, 60, 59, 123, 187, 193, 124, 30, 238]).unwrap()) };

    let sig_result = pk.sign_hash(&hash).unwrap();
    assert_eq!(G1Affine::from(sig.sig), G1Affine::from(sig_result.sig));
}
