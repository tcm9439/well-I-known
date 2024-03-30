use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Add;

use crate::error::AuthError;
use super::jwt_key::JwtKeys;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    sub: String,        // Subject (whom the token refers to)
    exp: usize,         // Expiration time (as UTC timestamp)
}

impl Display for JwtClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: {}", self.sub)
    }
}

impl JwtClaims {
    pub fn new(sub: &str) -> Self {
        let expired_ts = Utc::now().add(Duration::days(1));
        Self { sub: sub.to_string(), exp: expired_ts.timestamp() as usize }
    }

    pub fn gen_token(&self, jwt_key: &JwtKeys) -> Result<String, AuthError> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &self, &jwt_key.encoding)
            .map_err(|_| AuthError::TokenCreation)?;
        Ok(token)
    }
}
