mod auth;
pub mod config; // expose config module to outside the crate
mod controller;
mod db;
mod dao;
mod error;
mod repository;
mod server_state;
pub mod server_init;

use auth::jwt_controller::authorize_handler;
use controller::user::*;
use controller::admin::*;
use controller::config_data::*;
use repository::user::UserRepository;
use config::server_config::*;
use server_state::ServerState;

// HTTP server framework
use axum::{routing::{delete, get, post}, Router};
// error handling
use anyhow::Result;
// tracing
use tracing::*;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;

pub struct WIKServer {}

/// The 'main' of the server.
impl WIKServer {
    /// Init tracing by the loaded logging config.
    /// Return the guard to the main().
    pub fn init_tracing(server_config: &WIKServerConfig) -> WorkerGuard {
        // register tracing file appender
        let (non_blocking_trace_file_appender, guard) = tracing_appender::non_blocking(
            server_config.logging.get_logging_file_appender());

        // change timestamp to local time instead of UTC
        // tracing-subscriber = { version = "0.3.18", features = ["env-filter", "chrono"] }
        //                                                                       ^^^^^^
        // let timer = ChronoLocal::rfc_3339();
        let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string());

        // logging to stdout seems to be enabled by default for fmt::Subscriber
        let subscriber = fmt::Subscriber::builder()
            .with_max_level(server_config.logging.get_logging_level())
            .with_timer(timer)
            .finish()
            .with(fmt::Layer::default()
                .with_ansi(false)
                .with_writer(non_blocking_trace_file_appender));

        tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber.");

        // guard is needed to be in / returned to main()
        guard
    }

    /// Start the server with the loaded server config.
    /// 'main' function of the server.
    pub async fn start_server(server_config: &mut WIKServerEnvironmentConfig) -> Result<()> {
        debug!("Starting server...");

        debug!("Init TLS...");
        let tls_config = server_config.config.tls.get_rustls_config().await;

        debug!("Init database connection...");
        let db_conn = server_config.get_db_conn().await?;

        debug!("Loading root user...");
        // load root user username & keys
        let root_user = UserRepository::get_root_user(&db_conn, server_config).await
            .expect("Fail to get root user.");
        server_config.root_user = Some(root_user);

        let server_state = ServerState {
            db_conn,
            config: server_config.clone(),
            jwt_keys: auth::jwt_key::JwtKeys::new(server_config.config.jwt_secret.as_bytes()),
        };

        // register the routes
        trace!("Registering routes...");
        let app = Router::new()
            .route("/login", post(authorize_handler))
            .route("/data", get(get_data_handler))
            .route("/data", post(alter_data_handler))
            .route("/data", delete(delete_data_handler))
            .route("/users/validate", post(validate_user_handler))
            .route("/users", post(alter_user_handler))
            .route("/users", delete(delete_user_handler))
            .route("/admin/access", post(create_admin_access_handler))
            .route("/admin/access", delete(delete_admin_access_handler))
            // register the server state so that it can be accessed in the handlers
            .with_state(server_state.into());
        
        info!("Server started at: {}", server_config.config.get_server_ip());
        // start the server
        axum_server::bind_rustls(server_config.config.get_server_ip(), tls_config)
            .serve(app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}