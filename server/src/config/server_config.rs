use tracing::Level;
use tracing_appender::rolling::RollingFileAppender;

use serde::{Deserialize, Serialize};
use figment::{Figment, providers::{Format, Json, Serialized}};
use axum_server::tls_rustls::RustlsConfig;
use std::path::PathBuf;
use std::net::SocketAddr;

// set a const string for environment variable name
const WIK_SERVER_HOME_ENV_VAR_NAME: &str = "WELLIK_HOME";

/// All config needed for the server.
pub struct WIKServerEnvironmentConfig {
    pub base_dir: PathBuf,          // base directory of the server
    pub config: WIKServerConfig,    // server config
}

impl WIKServerEnvironmentConfig {
    pub fn to_full_path(&self, relative_path: &str) -> PathBuf {
        self.base_dir.join(relative_path)
    }

    pub fn get_base_dir_from_env() -> Option<PathBuf> {
        let path = std::env::var(WIK_SERVER_HOME_ENV_VAR_NAME);
        match path {
            Ok(path) => Some(PathBuf::from(path)),
            Err(_) => None,
        }
    }

    pub fn set_base_dir_to_env(base_dir: PathBuf) {
        // panic if the path is not valid
        std::env::set_var(WIK_SERVER_HOME_ENV_VAR_NAME, base_dir.to_str().unwrap());
    }

    /// Get the current directory where this binary is executed.
    pub fn get_base_dir_from_current_dir() -> PathBuf {
        // panic if the current directory is not found
        std::env::current_dir().unwrap()
    }

    pub fn get_config_dir_path(&self) -> PathBuf {
        self.base_dir.join("config")
    }

    pub fn get_config_file_path(&self) -> PathBuf {
        self.get_config_dir_path().join("wellik-server.json")
    }

    pub fn get_tls_certs_dir_path(&self) -> PathBuf {
        self.base_dir.join("certs").join("tls")
    }

    pub fn get_root_certs_dir_path(&self) -> PathBuf {
        self.base_dir.join("certs").join("root")
    }

    pub fn get_users_certs_dir_path(&self) -> PathBuf {
        self.base_dir.join("certs").join("users")
    }

    pub fn get_data_dir_path(&self) -> PathBuf {
        self.base_dir.join("data")
    }

    pub fn get_db_path(&self) -> PathBuf {
        self.get_data_dir_path().join("wellik.sqlite")
    }

    pub fn get_log_dir_path(&self) -> PathBuf {
        let log_dir = &self.config.logging.log_dir;
        self.to_full_path(log_dir)
    }
}

// ====== JSON config parsing ======

#[derive(Deserialize, Serialize, Clone)]
pub struct WIKServerConfig {
    // pub server_ip: Option<String>,
    pub server_ip: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub db_path: String,
    pub tls: WIKServerTlsConfig,
    pub logging: WIKServerLoggerConfig,
}

impl Default for WIKServerConfig {
    fn default() -> Self {
        WIKServerConfig {
            server_ip: "127.0.0.1".to_string(),             // default to localhost
            server_port: 3001,
            jwt_secret: "secret".to_string(),               // should be overwritten
            db_path: "./data/wellik.sqlite".to_string(),
            tls: WIKServerTlsConfig::default(),
            logging: WIKServerLoggerConfig::default(),
        }
    }
}

/// Certs for the HTTPS server.
#[derive(Deserialize, Serialize, Clone)]
pub struct WIKServerTlsConfig {
    cert_file: String,
    key_file: String,
}

impl Default for WIKServerTlsConfig {
    fn default() -> Self {
        WIKServerTlsConfig {
            cert_file: "./certs/tls/cert.pem".to_string(),
            key_file: "./certs/tls/key.pem".to_string(),
        }
    }
}

/// Log files & levels config.
#[derive(Deserialize, Serialize, Clone)]
pub struct WIKServerLoggerConfig {
    log_dir: String,
    log_file: String,
    level: String,
    rotate_interval: String,
}

impl Default for WIKServerLoggerConfig {
    fn default() -> Self {
        WIKServerLoggerConfig {
            log_dir: "./logs".to_string(),
            log_file: "wellik_server.log".to_string(),
            level: "info".to_string(),
            rotate_interval: "day".to_string(),
        }
    }
}

// ====== Json Config Constructor / Getter ======

impl WIKServerConfig {
    pub fn new(json_config_filepath: &str) -> WIKServerConfig {
        // check if the file exists
        if !std::path::Path::new(json_config_filepath).exists() {
            panic!("Config file not found: {}", json_config_filepath);
        }
        
        // Figment::new()
        Figment::from(Serialized::defaults(WIKServerConfig::default()))
            .merge(Json::file(json_config_filepath))
            .extract().expect("Fail to read config.")
    }

    /// Get the ip for hosting the server.
    pub fn get_server_ip(&self) -> SocketAddr {
        format!("{}:{}", &self.server_ip, self.server_port).parse().unwrap()
    }
}

impl WIKServerTlsConfig {
    /// Get the cert for the HTTPS.
    pub async fn get_rustls_config(&self) -> RustlsConfig {
        RustlsConfig::from_pem_file(
            PathBuf::from(self.cert_file.clone()),
            PathBuf::from(self.key_file.clone())
            ).await
            .unwrap()
        // unwrap as failing to get this config is FATAL
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