use serde::{self, Serialize, Deserialize};

/// Post body parameter for update / insert user
#[derive(Deserialize)]
pub struct UpdateUserParam {
    pub username: String,
    pub password: String,           // plaintext
    pub role: Option<String>,       // only for new user
    pub public_key: Option<String>, // only for new user
}

#[derive(Deserialize)]
pub struct DeleteUserParam {
    pub username: String,
}

#[derive(Deserialize)]
pub struct ValidateUserParam {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidateUserResponse {
    pub plaintext: String,
    // encrypted version of the plaintext by the user's (id by jwt) public key 
    pub encrypted: String,
}

#[derive(Deserialize)]
pub struct AdminAccessParam {
    pub operation: String,      // ApiOperation: create / delete
    pub admin: String,          // admin username
    pub app: String,            // app name
}
