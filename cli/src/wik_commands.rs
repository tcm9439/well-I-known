use clap::{Parser, Subcommand};

/// the arguments for running the cli
#[derive(Parser)]
#[command()]
#[clap(name = "wellik")]
pub struct WikMainArgs {
    #[command(subcommand)]
    pub command: Option<WikMainCommands>,
}

#[derive(Subcommand)]
pub enum WikMainCommands {
    /// Initialize the server
    Init {
        #[arg(short, long, value_name = "CONFIG_FILE")]
        file: Option<String>,
    },
    /// Login to the server
    Login {
        user: String,
    },
}

#[derive(Parser)]
#[command()]
#[clap(name = ">", disable_help_flag = true)]
pub struct WikLoggedInCommandArgs {
    #[command(subcommand)]
    pub command: Option<WikLoggedInCommands>,
}

#[derive(Subcommand)]
#[clap(disable_help_subcommand = true)]
pub enum WikLoggedInCommands {
    /// Exit the shell
    Exit,
    /// Create a new user
    Create {
        user_role: String,
        public_key_path: String,

        #[arg(long)]
        access: Option<String>,
    },
    /// Remove a user
    Remove {
        name: String,
    },
    /// Update admin access right
    AlterAdmin {
        name: String,
        operation: String,
        app_name: String,
    },
    /// Get config
    Get {
        app_name: String,
        config_key: String,
    },
    /// Set config
    Set {
        app_name: String,
        config_key: String,
        config_value: String,
    },
    /// Run commands from script
    Run {
        script_path: String,
    },
}
