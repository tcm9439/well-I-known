use crate::{auth::jwt_key::JwtKeys, db::db_connection::DbConnection, WIKServerConfig};

#[derive(Clone)]
pub struct ServerState {
    pub db_conn: DbConnection,
    pub config: WIKServerConfig,
    pub jwt_keys: JwtKeys,
}
