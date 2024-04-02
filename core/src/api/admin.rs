use serde::{self, Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct AdminAccessParam {
    pub operation: String,      // ApiOperation: create / delete
    pub admin: String,          // admin username
    pub app: String,            // app name
}
