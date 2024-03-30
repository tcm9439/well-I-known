
use clap::{Command, CommandFactory, Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct MainCommandArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    Login {
        #[arg(short, long)]
        user: String,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct SubCommandArgs {
    #[command(subcommand)]
    command: Option<SubCommands>,
}


#[derive(Subcommand)]
enum SubCommands {
    Exit,
    Greet {
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    }
}

// example usage:
// cargo run --bin wellik -- login --user root
// > greet --count 3

fn main() {
    // print the current directory where this cli is executkked
    println!("Current directory: {:?}", std::env::current_dir().unwrap());

    let cli = MainCommandArgs::parse();

    match &cli.command {
        Some(Commands::Login { user }) => {
            println!("Hello, {}!", user);

            // run a interactive "shell"
            let sub_command = SubCommandArgs::command();
            let struct_name: &str = sub_command.get_name();
            loop {
                // print the prompt & flush
                print!("> ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                let split_input = input.split_whitespace();
                // add "SubCommandArgs" at first 
                // get the struct name of SubCommandArgs
                let split_input = std::iter::once(struct_name).chain(split_input);
                let sub_cli = SubCommandArgs::try_parse_from(split_input);

                match sub_cli {
                    Ok(sub_cli) => match &sub_cli.command {
                        Some(SubCommands::Exit) => {
                            println!("Goodbye, {}!", user);
                            break;
                        }
                        Some(SubCommands::Greet { count }) => {
                            for _ in 0..*count {
                                println!("Hello {}!", user)
                            }
                        }
                        _ => {
                            println!("Skip.");
                        }
                    }
                    Err(e) => {
                        println!("Unknown command: {}", input);
                    }
                }
            }
        }
        None => {
            println!("No command provided");
        }
    }
}