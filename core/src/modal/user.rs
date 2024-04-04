use crate::crypto::cryptography::{WikRsaKeyPair, WikRsaPublicKey};

use std::{path::PathBuf, str::FromStr};
use anyhow::Result;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, Clone)]
pub enum UserRole {
    #[strum(ascii_case_insensitive)]
    Root,
    #[strum(ascii_case_insensitive)]
    Admin,
    #[strum(ascii_case_insensitive)]
    App,
}

// User modal for the client side (for self's user data)
#[derive(Clone)]
pub struct UserModal {
    pub username: String,
    pub role: UserRole,
    pub password: String,           // own plaintext password
    pub private_key: WikRsaKeyPair,  // own private key
}

#[derive(Clone)]
pub struct UserKeyModal {
    pub username: String,
    pub key: WikRsaKeyPair,
}

impl UserKeyModal {
    pub fn new(username: &str, private_key_path: &PathBuf) -> Result<Self> {
        Ok(UserKeyModal {
            username: username.to_string(),
            key: WikRsaKeyPair::from_private_key_file(private_key_path)?,
        })
    }
}

/// User modal at the server side. 
/// - only holds the public key but not the private key
/// - password is only used for auth so not needed in this struct (?)
#[derive(Clone)]
pub struct SeverUserModal {
    pub username: String,
    pub role: UserRole,
    public_key_path: PathBuf,
}

impl SeverUserModal {
    pub fn new(username: &str, role: &str, public_key_path: &PathBuf) -> Result<Self> {
        let role: UserRole = UserRole::from_str(role)?;
        Ok(SeverUserModal {
            username: username.to_string(),
            role,
            public_key_path: public_key_path.clone(),
        })
    }

    pub fn get_public_key(&self) -> Result<WikRsaPublicKey> {
        Ok(WikRsaPublicKey::from_file(&self.public_key_path)?)
    }
}

#[derive(Clone)]
pub struct ServerUserKeyModal {
    pub username: String,
    pub public_key: WikRsaPublicKey,
}

impl ServerUserKeyModal {
    pub fn new(username: &str, public_key_path: &PathBuf) -> Result<Self> {
        Ok(ServerUserKeyModal {
            username: username.to_string(),
            public_key: WikRsaPublicKey::from_file(public_key_path)?,
        })
    }

    pub fn new_from_key(username: &str, public_key: &WikRsaPublicKey) -> Self {
        ServerUserKeyModal {
            username: username.to_string(),
            public_key: public_key.clone(),
        }
    }
}