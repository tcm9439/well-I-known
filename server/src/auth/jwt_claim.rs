use super::jwt_key::JWT_KEYS;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Add;

use crate::error::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    sub: String,        // Subject (whom the token refers to)
    // role: String,       // User role 
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

    pub fn gen_token(&self) -> Result<String, AuthError> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &self, &JWT_KEYS.encoding)
            .map_err(|_| AuthError::TokenCreation)?;
        Ok(token)
    }
}

// axum extractor for decoding & verifying the JWT token
// See https://docs.rs/axum/0.7.4/axum/extract/index.html for what is an extractor
#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<JwtClaims>(bearer.token(), 
            &JWT_KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}