use well_i_known_core::crypto::cryptography::{Decryption, WikRsaKeyPair};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_key(){
    let key = WikRsaKeyPair::new().unwrap();
    key.private_key.decrypt_string("hello").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
