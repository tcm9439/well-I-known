use crate::crypto::cryptography::RsaKeyPair;

pub enum UserRole {
    Root,
    Admin,
    App,
}

pub struct User {
    username: String,
    role: UserRole,
    password: String,           // plaintext
    private_key: RsaKeyPair,
}