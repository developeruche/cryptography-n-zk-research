use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::pkey::Private;
use openssl::ecdsa::EcdsaSig;


pub struct VRF {
    key_pair: EcKey<Private>
}


impl VRF {
    pub fn new() -> Self {
        let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
        let key_pair = EcKey::generate(&group).unwrap();

        Self { key_pair }
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let s = EcdsaSig::sign(data, &self.key_pair).unwrap();
        s.to_der().unwrap()
    }

    pub fn verify(&self, data: &[u8], sign: &[u8]) -> bool {
        let s = EcdsaSig::from_der(sign).unwrap();
        s.verify(data, &self.key_pair).unwrap()
    }

    pub fn generate_vrf(&self, data: &[u8]) -> Vec<u8> {
        let s = self.sign(data);
        s
    }

    pub fn verify_vrf(&self, data: &[u8], sign: &[u8]) -> bool {
        self.verify(data, sign)
    }
}
