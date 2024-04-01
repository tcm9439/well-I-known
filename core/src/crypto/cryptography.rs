use std::path::{Path, PathBuf};

use rsa::{pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey}, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use base64::{Engine as _, engine::general_purpose};
use anyhow::Result;

const RSA_KEY_SIZE: usize = 2048;

// ref: https://docs.rs/rsa/latest/rsa/

/// A key pair for RSA encryption
pub struct RsaKeyPair {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
}

impl RsaKeyPair {
    /// Generate a new key pair
    pub fn new() -> Result<Self> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)?;
        let public_key = RsaPublicKey::from(&private_key);

        Ok(Self {
            public_key,
            private_key
        })
    }

    /// Load a key pair from a private key string
    pub fn from_private_key_str(private_key: &str) -> Result<Self> {
        let private_key = RsaPrivateKey::from_pkcs1_pem(private_key)?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok(Self {
            public_key,
            private_key
        })
    }

    /// Load a key pair from a private key file
    pub fn from_private_key_file(key_file: &Path) -> Result<Self> {
        let private_key = RsaPrivateKey::read_pkcs1_pem_file(key_file)?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok(Self {
            public_key,
            private_key
        })
    }

    pub fn save_to_pem_file(&self, directory: &PathBuf, 
        private_key_filename: &str, public_key_file_name: &str) -> Result<()> {
        self.public_key.write_pkcs1_pem_file(directory.join(public_key_file_name), rsa::pkcs8::LineEnding::LF)?;
        self.private_key.write_pkcs1_pem_file(directory.join(private_key_filename), rsa::pkcs8::LineEnding::LF)?;
        Ok(())
    }
}

pub trait Encryption {
    // trait items always share the visibility of their trait
    fn encrypt_string(&self, data: &str) -> Result<String>;
}

pub trait Decryption {
    fn decrypt_string(&self, data: &str) -> Result<String>;
}

impl Encryption for RsaPublicKey {
    fn encrypt_string(&self, data: &str) -> Result<String> {
        let mut rng = rand::thread_rng();
        let data = data.as_bytes();
        let encrypted_data = self.encrypt(&mut rng, Pkcs1v15Encrypt, &data)?;
        Ok(general_purpose::STANDARD_NO_PAD.encode(encrypted_data))
    }
}

impl Decryption for RsaPrivateKey {
    fn decrypt_string(&self, data: &str) -> Result<String> {
        let data = &general_purpose::STANDARD_NO_PAD.decode(data)?;
        let decrypted_data = self.decrypt(Pkcs1v15Encrypt, &data)?;
        Ok(String::from_utf8(decrypted_data)?)
    }
}

#[cfg(test)]
mod tests {
    use mt_test_util;
    use super::*;
    use indoc::indoc;

    fn get_example_key_pair() -> RsaKeyPair {
        // indoc => ignore the indentation of the string
        // here, without indoc, the key will not be read correctly
        let pem = indoc! {"
            -----BEGIN RSA PRIVATE KEY-----
            MIIEpAIBAAKCAQEA0rmcDlUKR0FNEdIK80tqfJkntzRE3jKqH58UhiEhJ0eVj/Ah
            52P9GSsM5BN+tDAMfC2qF9CNVRTHC12cgl8/NUU/3o1pZaL7XB9CF44pqPg/J8Ur
            c4k7Hol3ZNAAg+JJcEamm84TytXJJEP86fdxa02UZ3NrvQwMr6DNTInsnFtQM9AR
            ZEmxIEKx28gdf3BLD+vcjPTHmanxVp3yC5jtIXIP6kgZTDTq3W0nYMDoIXmbW6Mk
            qhSfiEAYjOnwQnAGClKwMwTrMeO5L5uTC6uudct9GOitXqxch9voquDquJBZCEDO
            +US7pep6U7NCk9xYUY1Q9FPMR2RChrtw/1E2HwIDAQABAoIBAQCK0EKBHnwa3GWM
            q7US+Ec5tRp4kTIcvXtcQUsf4IsaeQmJPELZTwDXp4hiZoU85boTw3EdJwEzXvCQ
            CCalq2DUEr40OSuJDIhQ8zZyle/To1TXOgeZoHkVcLD59BuixVpjX+c5E9Pt9k7+
            WNsutRY9/WkZjOsYKevhdihHl0QK6tmVT3xRpLhWl6S4ZcBUQIuKxltpIsfJ0fw1
            gO5fST1PyL2I1/xCIckNnO9+LIDMzrOTNBgADPQTs/gQTKKkVvUMQb059wk3obAj
            ErCXcWTuitbbdv2KsRmBfz+rV2dwsOgDkcyu7HXBg0HNlRcFNkZxw5SxAFJd7vTF
            e9AegYHBAoGBAPeR4eUicp2gDCDt++v5lNql1LerAhOjOHGhyp7T9mKDTbes58Jx
            QPsDiyso5CccJbc81BPkD0cuzIz+7GbVBWEkZC5jCRuB/QBw2l8E2+K/4DxUZE9x
            7njKUmmonPJUV+dko3WmheWygLPPTHBEBkED55P8H8fT74bKYjbZsScpAoGBANnm
            ixawmiFPNas7rSoiZd1oWP+91apUQ/hME0W+XJEQMeFl1QHDLJtL0UG4a3Q8AM0H
            x5M0MLn/OUTMjqH4iw9UkIwJgDdMySU1PQrilLWUwtPP8E5zMIXSNzUuAkykHhyO
            U+/XF1PdHHqiiAxi4AH0tmv9OcLauJPcghXqCIQHAoGAGGY1SDrj5usOJvStfm1D
            oWT0mQFum/bbKj/S02J6huz/7NlKohw9Vj/cKG3IRp58jRmeoTM52j8fg8ngDKZz
            +EX45aV3EVH3WGLG8tRsw2U7uVZr6HSHFzqBcs5eYXe0jiaaAY9e5Ot5yb7lfq8F
            msTCvw/7JduaYMNzeIpt7jkCgYEAzhIcugGcUiIR9HWEh5NuWJylhn7mgaYdfcWc
            eFnWOw5gzfQ7JOaK2fcy1/9sB8nzS/Ouh4VVC6HWbD00KvPdt0rXRMh7bFD+7WRS
            7WdpEey08BH4Bokje3tZ4L45SHfxTjpAIVN+aT5z/3qURXqAtPjUSH570M5+vr9M
            eyMQmKMCgYAF1TSuvLwlIBDQJhiPIWr3s7kaIhtK4RCfTTxq3uSyVFMzOVal7jTf
            GozmRS8Nvxc5Y6/bX+ktoiGbMZxpsf/EazgCzoAybolkBg7boEC+IN4r2/Ps70kZ
            q5fx0NtScwimW9715m040/Qrdfv5LHbKfWWW5IaC9QBJoKLYRebqeA==
            -----END RSA PRIVATE KEY-----"};
        RsaKeyPair::from_private_key_str(pem).unwrap()
    }

    #[test]
    fn encrypt_then_decrypt() {
        let key_pair = get_example_key_pair();
        let my_message = "Hello, world!";
        let encrypted_message = key_pair.public_key.encrypt_string(my_message).unwrap();
        let decrypted_message = key_pair.private_key.decrypt_string(&encrypted_message).unwrap();
        assert_eq!(my_message, decrypted_message);
    }

    #[test]
    fn new_key_pair_from_file() {
        let key_file = mt_test_util::get_resource_file("test-private-key.pem");
        println!("{}", key_file.display());
        let loaded_key_pair = RsaKeyPair::from_private_key_file(&key_file).unwrap();
        let expected_key_pair = get_example_key_pair();
        assert_eq!(expected_key_pair.private_key, loaded_key_pair.private_key);
    }

    #[test]
    fn save_key_pair_to_file() {
        let key_pair = get_example_key_pair();
        let temp_dir = mt_test_util::get_output_dir();
        let private_key_filename = "private-key.pem";
        let public_key_filename = "public-key.pem";
        key_pair.save_to_pem_file(&temp_dir, private_key_filename, public_key_filename).unwrap();
        let loaded_key_pair = RsaKeyPair::from_private_key_file(&temp_dir.join(private_key_filename)).unwrap();
        assert_eq!(key_pair.private_key, loaded_key_pair.private_key);
    }
}
