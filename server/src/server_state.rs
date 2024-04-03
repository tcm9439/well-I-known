use crate::{auth::jwt_key::JwtKeys, db::db_connection::DbConnection, WIKServerEnvironmentConfig};

/// The server state that will be shared across the api controllers.
#[derive(Clone)]
pub struct ServerState {
    pub db_conn: DbConnection,
    pub config: WIKServerEnvironmentConfig,
    pub jwt_keys: JwtKeys,
}
