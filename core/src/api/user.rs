use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UpdateUserParam {
    // #[serde(default, deserialize_with = "empty_string_as_none")]
    pub username: String,
    pub password: String,           // plaintext
    pub role: String,               // only for new user
    pub public_key: String,         // only for new user
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
    pub encrypted: String,
}
