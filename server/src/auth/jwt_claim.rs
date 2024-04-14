use chrono::{Duration, Utc, TimeZone};
use serde::{Deserialize, Serialize};
use well_i_known_core::modal::user::UserRole;
use std::fmt::{Display, Debug};
use std::ops::Add;
use std::str::FromStr;

use crate::error::ApiError;
use super::jwt_key::JwtKeys;

/// The content of the JWT token.
#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,        // Subject (whom the token refers to)
    pub exp: usize,         // Expiration time (as UTC timestamp)
    pub role: String,       // User role
}

impl Display for JwtClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // e.g. User: <test, role = admin>
        write!(f, "User: <{}, role = {}>", self.sub, self.role)
    }
}

impl Debug for JwtClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // e.g. User: <test, role = admin, exp = 2024-04-05 09:13:51 UTC>
        let expired = Utc.timestamp_opt(self.exp as i64, 0).unwrap();
        write!(f, "User: <{}, role = {}, exp = {}>", self.sub, self.role, expired)
    }
}

impl JwtClaims {
    pub fn new(sub: &str, role: &str) -> Self {
        let expired_ts = Utc::now().add(Duration::days(1));
        Self { 
            sub: sub.to_string(), 
            exp: expired_ts.timestamp() as usize,
            role: role.to_string(),
        }
    }

    /// Generate a JWT token from the claims with the key.
    pub fn gen_token(&self, jwt_key: &JwtKeys) -> Result<String, ApiError> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &self, &jwt_key.encoding)
            .map_err(|_| ApiError::TokenCreation)?;
        Ok(token)
    }

    pub fn get_role(&self) -> UserRole {
        UserRole::from_str(&self.role).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_claims() {
        let jwt_key = JwtKeys::new(b"secret");
        let claims = JwtClaims::new("test", "admin");
        println!("Claims: {} | {:?}", claims, claims);
        let token = claims.gen_token(&jwt_key).unwrap();
        let decoded = jsonwebtoken::decode::<JwtClaims>(&token, &jwt_key.decoding, &jsonwebtoken::Validation::default());
        assert!(decoded.is_ok());
        let decoded_claims = decoded.unwrap().claims;
        assert_eq!(claims.sub, decoded_claims.sub);
        assert_eq!(claims.role, decoded_claims.role);
    }
}
