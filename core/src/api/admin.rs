use serde::{self, Deserialize};

#[derive(Deserialize, Debug)]
pub struct AdminAccessParam {
    pub admin: String,          // admin username
    pub app: String,            // app name
}
