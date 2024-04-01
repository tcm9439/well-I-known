use crate::crypto::cryptography::RsaKeyPair;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum UserRole {
    #[strum(ascii_case_insensitive)]
    Root,
    #[strum(ascii_case_insensitive)]
    Admin,
    #[strum(ascii_case_insensitive)]
    App,
}

pub struct UserModal {
    username: String,
    role: UserRole,
    password: String,           // plaintext
    private_key: RsaKeyPair,
}