use crate::crypto::cryptography::{RsaKeyPair, Decryption};
use anyhow::Result;

pub struct ConfigData {
    pub app_name: String,
    pub key: String,
    pub value: String,          // plaintext
}

impl ConfigData {
    /// A new ConfigData instance (plaintext value)
    pub fn new(app_name: String, key: String, value: String) -> Self {
        ConfigData {
            app_name,
            key,
            value,
        }
    }

    /// A new ConfigData instance from a database record (stored as encrypted value)
    pub fn new_from_db(app_name: String, key: String, encrypted_value: String, private_key: &RsaKeyPair) -> Result<ConfigData> {
        let value = private_key.private_key.decrypt_string(&encrypted_value)?;
        Ok(ConfigData {
            app_name,
            key,
            value,
        })
    }
}