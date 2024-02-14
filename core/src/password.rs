use bcrypt::{hash, verify};
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;

pub struct Password {
    hash: String,
    salt: String,
}

/// generate a random 32 bits salt 
fn generate_salt() -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    general_purpose::STANDARD_NO_PAD.encode(salt)
}

fn hash_password(password: &str, salt: &str) -> Result<String>{
    let to_be_hashed = format!("{}{}", password, salt);
    match hash(to_be_hashed, 6) {
        Ok(h) => Ok(h),
        Err(e) => Err(anyhow::anyhow!("Error hashing password: {}", e))
    }
}

impl Password {
    pub fn new(password: &str) -> Result<Password> {
        let salt = generate_salt();
        let hash = hash_password(password, &salt)?;
        Ok(Password {
            hash,
            salt
        })
    }

    pub fn verify(&self, password_to_verify: &str) -> bool {
        let to_be_hashed = format!("{}{}", password_to_verify, self.salt);
        match verify(to_be_hashed, &self.hash) {
            Ok(v) => v,
            Err(_) => false
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_then_verify() {
        let my_password = "SimpleSecret!";
        let password = Password::new(my_password).unwrap();
        assert!(password.verify(my_password));
        assert!(!password.verify("WrongPassword"));
    }
}