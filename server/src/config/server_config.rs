use tracing::Level;
use tracing_appender::rolling::RollingFileAppender;

use serde::Deserialize;
use figment::{Figment, providers::{Format, Json}};
use axum_server::tls_rustls::RustlsConfig;
use std::path::PathBuf;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct WIKServerConfig {
    server_ip: Option<String>,
    server_port: u16,
    pub tls: WIKServerTlsConfig,
    pub logging: WIKServerLoggerConfig,
}

#[derive(Deserialize)]
pub struct WIKServerTlsConfig {
    cert_file: String,
    key_file: String,
}

#[derive(Deserialize)]
pub struct WIKServerLoggerConfig {
    log_dir: String,
    log_file: String,
    level: String,
    rotate_interval: String,
}

impl WIKServerConfig {
    pub fn new(json_config_filepath: &str) -> WIKServerConfig {
        Figment::new()
        .merge(Json::file(json_config_filepath))
        .extract().expect("Fail to read config.")
    }

    /// Get the ip for hosting the server.
    pub fn get_server_ip(&self) -> SocketAddr {
        if let Some(ip_str) = &self.server_ip {
            format!("{}:{}", ip_str, self.server_port).parse().unwrap()
        } else {
            // default to localhost
            SocketAddr::from(([127, 0, 0, 1], self.server_port))
        }
    }
}

impl WIKServerTlsConfig {
    /// Get the cert for the HTTPS.
    pub async fn get_rtlus_config(&self) -> RustlsConfig {
        RustlsConfig::from_pem_file(
            PathBuf::from(self.cert_file.clone()),
            PathBuf::from(self.key_file.clone())
            ).await
            .unwrap()
        // unwrap as it is fatal failing to get this config
    }
}

impl WIKServerLoggerConfig {
    pub fn get_logging_file_appender(&self) -> RollingFileAppender {
        match self.rotate_interval.as_str() {
            "hour" => tracing_appender::rolling::hourly(self.log_dir.clone(), self.log_file.clone()),
            "day" | _ => tracing_appender::rolling::daily(self.log_dir.clone(), self.log_file.clone()),
        }
    }

    pub fn get_logging_level(&self) -> Level {
        match self.level.to_lowercase().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" | _ => Level::ERROR,
        }
    }
}