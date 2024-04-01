use serde::{self, Deserialize};

/// GET config query param
#[derive(Deserialize)]
pub struct GetDataQuery {
    pub app: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct UpdateDataParam {
    pub app: String,
    pub key: String,
    pub value: String,
}
