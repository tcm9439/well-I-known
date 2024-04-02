use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use well_i_known_core::modal::user::UserRole;
use std::fmt::{Display, Debug};
use std::ops::Add;
use std::str::FromStr;

use crate::error::ApiError;
use super::jwt_key::JwtKeys;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,        // Subject (whom the token refers to)
    pub exp: usize,         // Expiration time (as UTC timestamp)
    role: String,       // User role
}

impl Display for JwtClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: <{}, role = {}>", self.sub, self.role)
    }
}

impl Debug for JwtClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
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

    pub fn gen_token(&self, jwt_key: &JwtKeys) -> Result<String, ApiError> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &self, &jwt_key.encoding)
            .map_err(|_| ApiError::TokenCreation)?;
        Ok(token)
    }

    pub fn get_role(&self) -> UserRole {
        UserRole::from_str(&self.role).unwrap()
    }
}
