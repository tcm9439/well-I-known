use serde::{self, Deserialize};

/// GET config query param
#[derive(Deserialize, Debug)]
pub struct GetDataQuery {
    pub app: String,
    pub key: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateDataParam {
    pub app: String,
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct DeleteDataParam {
    pub app: String,
    pub key: String,
}
