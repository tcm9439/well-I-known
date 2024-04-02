use serde::{self, Serialize, Deserialize};

/// Post body parameter for update / insert user
#[derive(Deserialize)]
pub struct UpdateUserParam {
    pub username: String,
    pub password: String,           // plaintext
    pub role: Option<String>,       // only for new user
    pub public_key: Option<String>, // only for new user
}

impl core::fmt::Debug for UpdateUserParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // do not log / print password
        write!(f, "UpdateUserParam {{ username: {}, role: {:?}, public_key: {:?} }}", self.username, self.role, self.public_key)
    }
}

#[derive(Deserialize, Debug)]
pub struct DeleteUserParam {
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct ValidateUserParam {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidateUserResponse {
    pub plaintext: String,
    // encrypted version of the plaintext by the user's (id by jwt) public key 
    pub encrypted: String,
}
